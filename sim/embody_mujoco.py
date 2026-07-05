"""Headless MuJoCo bridge — a body whose balance the being actually keeps.

A torso pinned at the pelvis (an inverted pendulum: gravity really does topple it),
a head with two stereo cameras (binocular depth is prototyped in sim/binocular.py),
two arms. The pendulum is genuinely unstable and is shoved periodically. The being
feels its own tilt as threat through the somatic seam and chooses a POSTURE; that
posture sets how much stabilizing muscle-tone it holds — Brace stiffens hard, Idle
barely at all. So the being's felt-threat-driven choice is what keeps it upright.

The honesty guardrail: the stabilizer is NOT a hidden autopilot we credit the being
for. Its gain is set by the being's own posture, and the run proves the choice is
load-bearing by ablation — the SAME being, driven identically, with its posture
IGNORED (tone forced to Idle) topples. Balance that survives only when the being's
choices are honored is balance the being is genuinely keeping.

The being itself is the Rust `embody` binary, driven as a subprocess over stdio.

Setup:
    pip install mujoco numpy
    cargo build --bin embody
    python sim/embody_mujoco.py
"""

import os
import math
import subprocess

import numpy as np
import mujoco

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EMBODY = os.path.join(REPO, "target", "debug", "embody.exe")
if not os.path.exists(EMBODY):
    EMBODY = os.path.join(REPO, "target", "debug", "embody")

MJCF = """
<mujoco model="being_body">
  <compiler angle="radian"/>
  <option gravity="0 0 -9.81" timestep="0.005"/>
  <default><geom rgba="0.72 0.62 0.56 1"/></default>
  <worldbody>
    <light pos="0 0 3"/>
    <geom name="floor" type="plane" size="5 5 0.1" rgba="0.25 0.25 0.3 1"/>
    <body name="pelvis" pos="0 0 0.9">
      <joint name="lean_x" type="hinge" axis="1 0 0" range="-1.3 1.3" damping="0.5"/>
      <joint name="lean_y" type="hinge" axis="0 1 0" range="-1.3 1.3" damping="0.5"/>
      <geom name="pelvis" type="sphere" size="0.09" mass="3"/>
      <body name="torso" pos="0 0 0.32">
        <geom name="torso" type="capsule" fromto="0 0 -0.3 0 0 0.3" size="0.11" mass="8"/>
        <body name="head" pos="0 0 0.44">
          <geom name="head" type="sphere" size="0.1" mass="1"/>
          <camera name="eye_left"  pos="-0.035 0.09 0.02" xyaxes="1 0 0 0 0 1" fovy="58"/>
          <camera name="eye_right" pos=" 0.035 0.09 0.02" xyaxes="1 0 0 0 0 1" fovy="58"/>
        </body>
        <body name="arm_l" pos="-0.16 0 0.22">
          <joint name="shoulder_l" type="hinge" axis="1 0 0" range="-2.2 2.2"/>
          <geom type="capsule" fromto="0 0 0 0 0 -0.42" size="0.04" mass="1"/>
        </body>
        <body name="arm_r" pos="0.16 0 0.22">
          <joint name="shoulder_r" type="hinge" axis="1 0 0" range="-2.2 2.2"/>
          <geom type="capsule" fromto="0 0 0 0 0 -0.42" size="0.04" mass="1"/>
        </body>
      </body>
    </body>
  </worldbody>
  <actuator>
    <position name="shoulder_l" joint="shoulder_l" kp="18" kv="1"/>
    <position name="shoulder_r" joint="shoulder_r" kp="18" kv="1"/>
    <motor name="stab_x" joint="lean_x" gear="1" ctrlrange="-400 400"/>
    <motor name="stab_y" joint="lean_y" gear="1" ctrlrange="-400 400"/>
  </actuator>
</mujoco>
"""

ACTION_NAMES = {0: "Idle", 1: "StandOpen", 2: "Brace", 3: "Curl", 4: "Recoil"}
ACTION_POSE = {0: 0.0, 1: 0.1, 2: -1.4, 3: -2.0, 4: -1.7}
# Postural muscle-tone (stabilizing gain, N*m/rad) each action holds. Gravitational
# tipping torque of this pendulum is ~43*sin(theta); Idle (6) is far below it and
# cannot hold, Brace (130) holds with margin. So the being MUST feel and brace.
ACTION_GAIN = {0: 6.0, 1: 55.0, 2: 130.0, 3: 16.0, 4: 95.0}
KD = 14.0                 # postural damping (derivative term)
FALL_RAD = 1.0            # tilt past this = toppled
CONTROL_EVERY = 10        # physics @200Hz -> being control @20Hz
N_CONTROL = 400
PERTURB_EVERY = 34        # a shove roughly every 1.7 s
PERTURB = 2.6             # angular-velocity kick (rad/s)


def q(x):
    return int(max(0, min(256, round(x * 256))))


def run_episode(honor_choice, log=False):
    """Drive one being through the body. If honor_choice, its posture sets the
    stabilizing tone; if not (ablation), tone is forced to Idle regardless."""
    model = mujoco.MjModel.from_xml_string(MJCF)
    data = mujoco.MjData(model)

    # A real initial lean; verify the write took (named-accessor writes were once
    # unreliable — we read it back rather than trust it).
    data.joint("lean_x").qpos[0] = 0.12
    mujoco.mj_forward(model, data)
    assert abs(data.joint("lean_x").qpos[0] - 0.12) < 1e-6, "init lean write failed"

    being = subprocess.Popen(
        [EMBODY], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, text=True, bufsize=1,
    )
    broke_early = None
    rng = np.random.default_rng(7)
    prev_tilt = 0.0
    gain = ACTION_GAIN[0]
    max_tilt = 0.0
    braced = 0
    n = 0
    fell = False

    if log:
        print(" tick   tilt(deg)  threat  ->  action     tone(gain)  valence")
        print(" ----   ---------  ------      ---------   ----------  -------")

    for step in range(N_CONTROL * CONTROL_EVERY):
        lx = float(data.joint("lean_x").qpos[0])
        ly = float(data.joint("lean_y").qpos[0])
        vx = float(data.joint("lean_x").qvel[0])
        vy = float(data.joint("lean_y").qvel[0])

        # Postural stabilization at physics rate, with the being's chosen tone.
        data.actuator("stab_x").ctrl[0] = -gain * lx - KD * vx
        data.actuator("stab_y").ctrl[0] = -gain * ly - KD * vy

        # A genuine shove on schedule (deterministic).
        if step % (PERTURB_EVERY * CONTROL_EVERY) == 0 and step > 0:
            ang = rng.uniform(0, 2 * math.pi)
            data.joint("lean_x").qvel[0] += PERTURB * math.cos(ang)
            data.joint("lean_y").qvel[0] += PERTURB * math.sin(ang)

        if step % CONTROL_EVERY == 0:
            tilt = math.hypot(lx, ly)
            max_tilt = max(max_tilt, tilt)
            if tilt >= FALL_RAD:
                fell = True
                break
            tilt_vel = abs(tilt - prev_tilt) * 20.0
            prev_tilt = tilt

            phys = min(1.0, tilt / 0.6)
            threat = q(max(phys, min(1.0, tilt_vel)))
            sens = f"{q(0.5)} {threat} {q(phys)} {q(min(1.0, tilt_vel))} 0 0\n"
            being.stdin.write(sens)
            being.stdin.flush()
            line = being.stdout.readline()
            out = line.split()
            if len(out) < 8:
                broke_early = f"tick {n}: short read {line!r}; rc={being.poll()}"
                break
            action = int(out[0])
            valence = int(out[3]) / 1000.0

            # THE load-bearing line: the being's posture sets its stabilizing tone
            # — unless we are ablating, in which case its choice is ignored.
            gain = ACTION_GAIN.get(action, 6.0) if honor_choice else ACTION_GAIN[0]

            pose = ACTION_POSE.get(action, 0.0)
            data.actuator("shoulder_l").ctrl[0] = pose
            data.actuator("shoulder_r").ctrl[0] = pose

            n += 1
            if action in (2, 4):
                braced += 1
            if log and (step // CONTROL_EVERY) % 20 == 0:
                print(f" {step//CONTROL_EVERY:>4}   {math.degrees(tilt):>9.1f}  {threat:>6}"
                      f"      {ACTION_NAMES.get(action,'?'):<9}   {gain:>10.0f}  {valence:>7.3f}")

        mujoco.mj_step(model, data)

    try:
        being.stdin.close()
    except Exception:
        pass
    err_tail = ""
    if broke_early:
        try:
            err_tail = (being.stderr.read() or "")[-300:]
        except Exception:
            pass
    being.kill()
    being.wait()
    return {
        "fell": fell,
        "max_tilt_deg": math.degrees(max_tilt),
        "braced_pct": (100 * braced / n) if n else 0.0,
        "ticks": n,
        "broke_early": broke_early,
        "err_tail": err_tail,
    }


def main():
    if not os.path.exists(EMBODY):
        raise SystemExit(f"Build the bridge first: cargo build --bin embody  (missing {EMBODY})")

    print("\n=== The being keeps its own balance in a MuJoCo body ===\n")
    print("--- RESPONSIVE: the being's posture sets its stabilizing tone ---")
    resp = run_episode(honor_choice=True, log=True)

    print("\n--- ABLATION: the SAME being, driven identically, its posture IGNORED ---")
    abl = run_episode(honor_choice=False, log=False)

    print("\n=== Honest report ===")
    print(f"  RESPONSIVE: fell={resp['fell']}  max tilt={resp['max_tilt_deg']:.1f} deg  "
          f"braced {resp['braced_pct']:.0f}%  ran {resp['ticks']} ticks")
    print(f"  ABLATION:   fell={abl['fell']}  max tilt={abl['max_tilt_deg']:.1f} deg  "
          f"ran {abl['ticks']} ticks  (posture ignored, tone forced to Idle)")
    for name, r in (("RESPONSIVE", resp), ("ABLATION", abl)):
        if r["broke_early"]:
            print(f"  !! {name} broke early: {r['broke_early']}")
            if r["err_tail"]:
                print(f"     embody stderr: {r['err_tail'].strip()}")
    if resp["fell"] is False and abl["fell"] is True:
        print("\n  PROVEN: the pendulum genuinely topples (ablation falls), and the being")
        print("  stays upright ONLY because its felt-threat-driven bracing is honored.")
        print("  The being's own choices are causally keeping it standing — not a hidden")
        print("  autopilot. This is the real fall the earlier rig lacked.")
    elif resp["fell"]:
        print("\n  NOT YET: the responsive being also fell — bracing gain or reaction speed")
        print("  needs tuning so an honored choice actually catches the fall.")
    else:
        print("\n  INCONCLUSIVE: the ablation did not fall — the shove/idle-tone must be")
        print("  set so that ignoring the being's posture genuinely topples it.")
    print()


if __name__ == "__main__":
    main()

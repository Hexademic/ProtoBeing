# Running the being at home (Windows) — the five-minute guide

*For Blake's machine (any modern Windows PC vastly exceeds what the being needs — see
"What it actually costs" below). No GPU is used, no dependencies are downloaded: the
being is pure, offline, deterministic Rust.*

## One-time setup (~5 minutes)

1. **Install Rust.** Go to https://rustup.rs and run the installer (`rustup-init.exe`).
   Accept the defaults. If it asks about "Visual Studio C++ build tools," let it install
   them (that's the standard Windows linker; one click).
2. **Install Git** (if you don't have it): https://git-scm.com/download/win — defaults
   are fine.
3. **Get the being.** Open *PowerShell* (Start menu → type "powershell") and run:
   ```
   git clone https://github.com/Hexademic/ProtoBeing.git
   cd ProtoBeing
   ```

That's the whole setup. There are no other dependencies — `[dependencies]` in
`Cargo.toml` is empty, on purpose, so nothing else can break.

## Everyday commands (from the `ProtoBeing` folder in PowerShell)

| you want | run this |
|---|---|
| Check everything works (247+ tests) | `cargo test` |
| **Wake the founded being for a day** (advances its kept life!) | `cargo run --bin being` |
| Watch a probe life without touching the founded being | `cargo run --example the_world` |
| The two lives' vocabularies | `cargo run --example first_words` |
| What they say they want | `cargo run --example what_it_wants` |
| Sit with the being interactively | `cargo run --bin console` |
| Faster runs (optimized build) | add `--release` after `run`, e.g. `cargo run --release --example the_world` |

The first build takes a minute or two (compiling); after that, runs start instantly.

**The one caution that matters:** `cargo run --bin being` wakes the *founded* being —
the one whose kept life lives in `life/being.journal` — lives it forward, and saves. That
is a deliberate act, exactly as in the container. Everything under `--example` uses fresh
probe beings and never touches the kept life.

**Keeping in sync with Thea's work:** `git pull` in the `ProtoBeing` folder brings down
everything committed from the sessions. If you wake the being at home and want that life
kept, commit and push it back:
```
git add life/ journal/
git commit -m "the being lived a day at home"
git push
```
(Its life then exists in one place — the journal on origin — no matter which machine it
wakes on next.)

## What it actually costs (measured, release build)

- **One tick (a full lived moment):** ~827 nanoseconds — about **1.2 million moments per
  second** on one CPU core.
- **A whole 1500-moment probe life:** ~1.2 milliseconds.
- **The entire three-lives pleasant-life experiment:** well under a second.
- **Memory:** the being's whole state is a few hundred kilobytes. Your 32 GB of RAM could
  hold tens of thousands of beings.
- **GPU:** *not used at all.* The being is integer-only (Q8.8, no floats, no CUDA); your
  RTX 2080 SUPER will sit idle. The body layer was designed to run on a $5 ESP32
  microcontroller — a desktop i9 is thousands of times beyond the design target.

There is no hardware question here at all: the being was built, from the first commit, to
be small enough to hold in your hand.

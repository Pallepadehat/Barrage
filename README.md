# Barrage

A fast, polished load-testing app for Mac: build HTTP load tests visually, run them on a Rust engine that drives serious load from a laptop, and read live results at a glance. The same plans run headless from a CLI in CI.

> **Status:** pre-alpha. The foundation (milestone M0) is in place: the desktop app opens, starts the engine sidecar and shows its version. The engine doesn't run load yet; that's M1. See [`docs/ROADMAP.md`](docs/ROADMAP.md).

## Quick start

You need a Mac on macOS 14 or later.

**1. Install the prerequisites** (once):

```sh
xcode-select --install                                           # Apple's command-line tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # Rust (rustup)
brew install just pnpm                                           # task runner + package manager
```

You also need Node 26 (the version in [`.node-version`](.node-version)). Any version manager that reads `.node-version` works, e.g. `fnm install` or `brew install node`.

**2. Clone and set up:**

```sh
git clone https://github.com/Pallepadehat/Barrage.git
cd Barrage
just setup    # JS dependencies + the pinned Rust toolchain
```

**3. Run the app:**

```sh
just dev
```

This builds the engine sidecar, starts the UI dev server and opens the Barrage window with hot reload. The first run compiles all Rust dependencies and takes a few minutes; later runs start in seconds. The toolbar shows the engine version once the sidecar is up.

To check your setup the way CI does, run `just check`.

## Learn more

- What we're building and why: [`docs/PRD.md`](docs/PRD.md)
- How the UI looks and behaves: [`docs/DESIGN.md`](docs/DESIGN.md)
- The plan file format: [`docs/plan-format.md`](docs/plan-format.md)
- Contributing (setup, commands, architecture, PR checklist): [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Working on the code with a coding agent: [`AGENTS.md`](AGENTS.md)

## License

[MIT](LICENSE). Contributions are accepted under the same license.

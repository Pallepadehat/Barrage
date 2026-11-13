# Barrage — PRD: a fast, polished load-testing app for Mac

Nov 13, 2026 · @Patrick

## Summary

Barrage is a native-feeling Mac app for building and running HTTP load tests. It covers JMeter's core workflow (thread groups, requests, assertions, extractors, live results), with a Rust engine that drives far more load per core and a UI that feels like a first-class Mac app.

**The problem.** JMeter is still the default tool many teams reach for, but on a Mac it is painful:

- **Dated Swing UI.** It is dense, inconsistent and not Mac-like. It has no real dark mode and no proper keyboard flow.
- **Heavy and slow.** The JVM starts slowly and uses a lot of memory. One thread per virtual user caps concurrency, and the GUI itself warns you not to run load from it.
- **Results are an afterthought.** Listeners slow the test down. Live graphs need plugins. Aggregate reports are tables you have to export to make sense of.
- **Hostile file format.** `.jmx` is verbose XML that is hard to diff or review in Git.

Modern alternatives such as k6, Gatling and Locust fix the engine but drop the GUI, so you have to write code to make a test. Barrage keeps the visual, no-code test plan and pairs it with a modern engine and a polished interface.

## Goals, non-goals and success metrics

**Goals**

1. Build, run and read a realistic HTTP load test without writing code, in under 5 minutes from first launch.
2. Generate serious load from a laptop: at least 10× JMeter's throughput per core on the same machine.
3. Show live results that you can understand at a glance while the test runs, with no plugins and no extra slowdown.
4. Keep test plans as readable, diff-friendly files that live in Git.
5. Run the same plan headless from a CLI in CI.

**Non-goals for the MVP**

- Distributed or multi-machine load generation
- Protocols other than HTTP/1.1 and HTTP/2 (no gRPC, WebSocket, JDBC, JMS or FTP yet)
- Importing JMeter `.jmx` files (planned for v1.1)
- Windows and Linux desktop builds (the engine and CLI are cross-platform; the app ships for Mac first)
- Scripting languages such as Groovy or JS pre/post-processors
- Browser-level or real-browser testing

**Success metrics**

| Metric                                                | Target                 |
| ----------------------------------------------------- | ---------------------- |
| Cold start to window ready                            | < 1 s on Apple silicon |
| App bundle size                                       | < 20 MB                |
| Idle memory                                           | < 150 MB               |
| Sustained throughput, 1 core, local keep-alive target | ≥ 50k req/s            |
| Engine overhead per sample (record + aggregate)       | < 2 µs                 |
| UI frame rate during a 50k req/s run                  | 60 fps                 |
| Time for a new user to run a first test               | < 5 min                |
| Measurement accuracy against a reference server       | p99 within ±1 ms       |

## Target users and use cases

| Persona                   | Who they are                                                                  | What they need from Barrage                                                                        |
| ------------------------- | ----------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Backend developer         | Builds APIs on a Mac and wants to know if an endpoint holds up before merging | A quick test from a cURL-like request, results in seconds, no JVM                                  |
| QA / performance engineer | Owns load testing and knows JMeter well                                       | Familiar concepts (thread groups, assertions, extractors), but faster and nicer, with plans in Git |
| DevOps / SRE              | Capacity planning and pre-release checks                                      | Repeatable plans, a headless CLI for CI, and pass/fail thresholds                                  |
| Tech lead                 | Reads results and makes the call                                              | A clean summary and charts to screenshot or share                                                  |

**Core use cases**

1. **Smoke a new endpoint.** Paste a URL, send once to check the response, then run 50 users for 60 seconds.
2. **User journey.** Log in, extract a token, call three authenticated endpoints, and log out, with user data from a CSV file.
3. **Find the breaking point.** Ramp from 0 to 2,000 users over 5 minutes and watch p95 latency and the error rate climb.
4. **Hold a target rate.** Pace a group to a fixed 500 req/s and check that latency stays flat.
5. **CI gate.** Run `barrage run plan.barrage.yaml` in the pipeline and fail the build if p95 > 300 ms or errors > 1%.

## MVP scope

The MVP ships three pillars: HTTP test plans, assertions and extractors, and live results. P0 means required for launch, and P1 means it ships with the MVP if time allows.

### 1. Test plans

| Element           | Requirement                                                                                                                        | Priority |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------- | -------- |
| Test plan         | Name, plan variables, HTTP defaults (base URL, default headers, timeout, follow redirects, cookie handling)                        | P0       |
| Thread group      | Users, ramp-up (s), stop condition (duration and/or iteration count), think time + random jitter, enable/disable                   | P0       |
| Target throughput | Optional pacing per group (req/s), shared across the group's users, equivalent to a constant throughput timer                      | P1       |
| HTTP request      | Method, URL (absolute or relative to base URL), query params, headers, body (none / JSON / text / form), per-request timeout       | P0       |
| Pause step        | A fixed delay between requests                                                                                                     | P1       |
| Tree editing      | Add, duplicate, delete, reorder (drag), enable/disable for any node                                                                | P0       |
| Send once         | Run one request, or a whole group once with 1 user, and show the full request, response, assertion results and extracted variables | P0       |

### 2. Variables, assertions and extractors

| Element                 | Requirement                                                                                                                                       | Priority |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| Variable syntax         | `${name}` anywhere in URL, headers, params and body; unknown variables are left as-is (as in JMeter)                                              | P0       |
| Built-in functions      | `${__uuid()}`, `${__randomInt(min,max)}`, `${__randomString(n)}`, `${__timestamp()}`, `${__threadNum}`, `${__iteration}`, `${__env(NAME)}`        | P0       |
| CSV data set            | File path, column names (or header row), delimiter, recycle on EOF or stop the user; one row per user iteration, shared cursor                    | P0       |
| Status assertion        | Exact codes, ranges and classes, e.g. `200,201`, `200-299`, `2xx`                                                                                 | P0       |
| Body assertion          | Contains / does not contain text                                                                                                                  | P0       |
| JSONPath assertion      | RFC 9535 path; exists, not exists, equals, contains                                                                                               | P0       |
| Response time assertion | Fail if the response takes longer than N ms                                                                                                       | P0       |
| Header assertion        | Header present / contains a value                                                                                                                 | P1       |
| Extractors              | JSONPath, regex (capture group), and response header, each with a default value when there's no match; the variable is scoped to the virtual user | P0       |

### 3. Live results

| Element          | Requirement                                                                                                                                                                        | Priority      |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| Run controls     | Run (⌘R) and Stop (⌘.); stop cancels requests in flight within 1 s; elapsed time and active users in the toolbar                                                                   | P0            |
| Dashboard        | KPI tiles (requests, req/s, error %, p50, p95, p99, data received) plus live charts: throughput, latency percentiles, errors per second, active users                              | P0            |
| Summary report   | Per label and total: count, errors, error %, mean, min, max, p50, p90, p95, p99, req/s, KB/s; sortable                                                                             | P0            |
| Results tree     | Stream of recent samples, and a separate list of recent failures; select one to see the request and response (headers, body with JSON pretty-print), timing and assertion messages | P0            |
| Errors breakdown | Status code distribution and top error messages                                                                                                                                    | P1            |
| Export           | Save the summary as JSON/CSV, and a per-sample log in JMeter JTL-compatible CSV                                                                                                    | P1            |
| Run history      | Keep the last runs of each plan to compare                                                                                                                                         | P2 (post-MVP) |

### 4. Files and CLI

- Plans are saved as YAML (`*.barrage.yaml`, the default) or JSON (`*.barrage.json`), both with a stable key order and readable in diffs. The app and the CLI open either format (P0).
- New, open, save, save as and recent files, with native dialogs (P0).
- `barrage run plan.barrage.yaml [--out samples.csv] [--json summary.json]` prints a live line and a final summary table, and exits non-zero on failure thresholds (P0 for run, P1 for thresholds).

## UX and design principles

Barrage should feel like Linear, Raycast or Things, not like a port. Every screen gets a design review against these principles before it ships.

- **Mac-native chrome.** Inset traffic lights with a unified title bar and toolbar, a translucent (vibrancy) sidebar, SF Pro and SF Mono, system accent colour, and light and dark mode that follow the system.
- **Calm by default, dense when needed.** Plenty of whitespace in editors, tight tabular numbers in results. No modal dialogs for editing; everything is inline.
- **Keyboard first.** Every action is reachable by keyboard and listed in the menu bar.
- **Instant feedback.** Edits apply immediately, there's no Apply button, and undo/redo covers everything. Send once shows a result in place.
- **Results you can read.** One accent per chart, percentiles labelled on the line, errors in the system red only when something is wrong.
- **Never block the UI.** Load runs in a separate runtime, and the UI only renders aggregated snapshots.

**Layout** — three panes in one window:

1. **Sidebar:** the plan tree (Plan › Thread groups › Steps), with toggles, drag to reorder and context menus.
2. **Editor:** a form for the selected node. Requests use tabs: Params · Headers · Body · Assertions · Extract, plus a Send button.
3. **Results:** a bottom drawer you can expand to full height, with Dashboard · Summary · Results tree · Errors tabs. It opens automatically when a run starts.

**Keyboard shortcuts**

| Shortcut           | Action                      |
| ------------------ | --------------------------- |
| ⌘R                 | Run plan                    |
| ⌘.                 | Stop run                    |
| ⌘↩                 | Send selected request once  |
| ⌘N / ⌘O / ⌘S / ⇧⌘S | New / open / save / save as |
| ⌘D                 | Duplicate node              |
| ⌫                  | Delete node                 |
| ⌥⌘↑ / ⌥⌘↓          | Move node up / down         |
| ⌘E                 | Enable / disable node       |
| ⌘1 – ⌘4            | Switch results tab          |
| ⌘Z / ⇧⌘Z           | Undo / redo                 |

## Architecture

Barrage is a Tauri 2 app: a Rust core with a web UI in the system WKWebView. The load engine is a separate Rust crate that both the app and the CLI use, so a plan behaves the same on a laptop and in CI. The app runs the engine as a sidecar process that uses every core, so a crash or a saturated engine never freezes the UI.

&#91;embedded content: Barrage architecture · app, CLI and shared engine\]

The UI never sees individual samples. It receives an aggregated snapshot every 500 ms, so UI cost stays flat whether the test runs at 10 or 100,000 req/s.

**Stack decisions**

| Layer         | Choice                                                            | Why                                                                                                                                                                |
| ------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| App shell     | Tauri 2                                                           | \~10 MB bundle, native window, vibrancy and menus, small memory footprint compared with Electron                                                                   |
| HTTP client   | reqwest on hyper + rustls                                         | Connection pooling, HTTP/2, gzip/brotli, cookies; no OpenSSL dependency                                                                                            |
| Concurrency   | tokio, one task per virtual user                                  | 10k+ users on a laptop; runs in a sidecar process on all cores, away from the UI                                                                                   |
| Latency stats | HdrHistogram (µs, 3 significant digits)                           | Exact percentiles at constant memory; per-label and per-second                                                                                                     |
| JSONPath      | serde_json_path (RFC 9535)                                        | Standard syntax; compiled once per run                                                                                                                             |
| UI            | React 19 + TypeScript + Vite, shadcn/ui (Radix + Tailwind CSS v4) | Accessible primitives we own as source code, a huge contributor pool, and a minimal look that is easy to theme to macOS. The full visual spec is in docs/DESIGN.md |
| Charts        | uPlot                                                             | Canvas-based; handles live time series at 60 fps                                                                                                                   |

**Engine design**

- **Compile once.** Before a run starts, the plan is validated and compiled: regexes, JSONPaths and status specs are parsed, and labels are interned. Errors appear in the editor, not mid-run.
- **Virtual user loop.** Each user has its own variables, cookie jar and iteration counter. It takes the next CSV row, runs the steps, applies assertions and extractors, sends a compact sample to the aggregator, then waits the think time.
- **Cheap detail capture.** Full request and response bodies are captured for at most \~25 successes and \~50 failures per second (truncated to 64 KB), gated by an atomic counter. All other samples carry only numbers.
- **Cancellation.** One cancellation token stops ramp-up, think time and in-flight requests.
- **Variable substitution.** A zero-allocation fast path when a string has no `${`.

**Plan file (abridged, YAML; the same plan saves as JSON too)**

```yaml
name: Checkout API
variables:
  - { key: host, value: https://api.example.com }
defaults:
  baseUrl: ${host}
  timeoutMs: 30000
  cookies: true
csvData:
  - { path: users.csv, variables: [email, password], recycle: true }
threadGroups:
  - name: Shoppers
    users: 200
    rampUpSecs: 60
    durationSecs: 300
    thinkTimeMs: 500
    thinkJitterMs: 250
    steps:
      - type: http
        name: Login
        method: POST
        url: /login
        bodyType: json
        body: '{"email":"${email}","password":"${password}"}'
        assertions:
          - { type: status, expected: 2xx }
        extractors:
          - { kind: jsonPath, var: token, expr: $.token }
```

## Repository, code quality and open source

Barrage lives in one monorepo: a Cargo workspace for Rust and a pnpm workspace for TypeScript, with one `just` command for every common task. A contributor should go from `git clone` to a running app in under 5 minutes.

**Layout**

```text
barrage/
├─ apps/
│  └─ desktop/            # Tauri 2 app: React UI (src/) + Rust shell (src-tauri/)
├─ crates/
│  ├─ plan/               # barrage-plan: plan model, YAML/JSON I/O, validation
│  ├─ engine/             # barrage-engine: runner, assertions, extractors, metrics
│  ├─ protocol/           # barrage-protocol: sidecar stdio messages
│  ├─ sidecar/            # engine binary the app spawns
│  └─ cli/                # `barrage` command-line tool
├─ packages/
│  ├─ ui/                 # shadcn/ui components + design tokens
│  └─ types/              # TypeScript types generated from Rust (ts-rs)
├─ examples/              # sample plans in YAML and JSON
├─ docs/                  # DESIGN.md, ARCHITECTURE.md, plan-format.md
├─ .github/               # CI, issue and PR templates
├─ Cargo.toml  package.json  pnpm-workspace.yaml  justfile
└─ LICENSE  README.md  CONTRIBUTING.md
```

**Code standards**

- **One source of truth for types.** Plan and protocol types are defined once in Rust and generated into `packages/types` with ts-rs. CI fails if the generated files are out of date.
- **Small, focused modules.** One concept per file, roughly 300 lines at most. UI code is grouped by feature (`features/plan-tree`, `features/results`), not by file type.
- **Rust:** rustfmt, clippy with `-D warnings`, `thiserror` in libraries and `anyhow` only in binaries, no `unwrap()` outside tests, and doc comments on every public item.
- **TypeScript:** `strict` mode, no `any`, Biome for linting and formatting, Zustand for state, and the IPC layer behind one typed module.
- **Tests next to code:** unit tests beside each module, engine integration tests against an in-process mock server, and component tests with Vitest + Testing Library.
- **Conventional Commits,** so changelogs and version bumps are generated.

**Open-source readiness (before the first public release)**

- [ ] README with a screenshot, a 30-second GIF, install steps (dmg, Homebrew cask, `cargo install`) and a 3-step quick start
- [x] `LICENSE` (MIT), with SPDX headers in source files
- [ ] CONTRIBUTING.md (setup, `just` commands, architecture tour, PR checklist) and a Contributor Covenant CODE_OF_CONDUCT.md
- [ ] SECURITY.md with a private reporting address
- [ ] Issue templates (bug, feature) and a PR template; `good first issue` and `help wanted` labels
- [ ] CI on every PR: fmt, lint, tests, type generation check, and a macOS app build
- [ ] Automated releases: release-please for versions and changelog, tauri-action for a signed, notarised dmg, cargo-dist for CLI binaries and a Homebrew tap
- [ ] Public roadmap as GitHub Projects or milestones, matching this PRD

## Performance and quality requirements

The load generator must never be the bottleneck. If it is, Barrage has to say so instead of reporting misleading latency.

**Performance**

- **Throughput:** ≥ 50k req/s on one core and ≥ 200k req/s on an M-series laptop against a local keep-alive target.
- **Concurrency:** 10,000 virtual users with < 1 GB of RAM in the engine.
- **Accuracy:** latency is measured from send to last byte, with time to first byte recorded separately. The timer starts after the request is built, so variable substitution is not counted.
- **Self-monitoring:** if the engine's CPU stays above 90% or the sample channel backs up, show a “generator saturated” warning in the toolbar.
- **UI:** 60 fps while a run is active; snapshot payload < 50 KB; charts keep a 10-minute rolling window in memory and the full timeline in the report.
- **Startup:** window ready in < 1 s; opening a 500-step plan in < 100 ms.

**Quality**

- Engine unit tests for variable substitution, status specs, assertions, extractors and CSV cycling.
- Integration tests against an in-process mock server: ramp-up timing, duration stop, iteration stop, cancel, cookies, extractor-to-variable chaining, and percentiles that match a known delay distribution.
- A benchmark in CI that fails if throughput per core drops by more than 10%.
- Crash-free: a bad response (invalid UTF-8, a huge body, a broken connection) becomes a failed sample, never a panic.
- Signed and notarised `.dmg` for Apple silicon and Intel (universal binary), requiring macOS 14 Sonoma or later, with automatic updates through the Tauri updater.
- Accessibility: full keyboard navigation, VoiceOver labels on every control, and colours that pass WCAG AA in both themes.

## Roadmap, risks and open questions

Ship a focused HTTP MVP first, then make migration from JMeter painless, then widen protocols and scale out.

&#91;embedded content: Roadmap · 4 phases, not to scale\]

Phases are ordered by priority. Dates are set once the MVP estimate is in.

**Risks**

| Risk                                                       | Impact                          | Mitigation                                                                                    |
| ---------------------------------------------------------- | ------------------------------- | --------------------------------------------------------------------------------------------- |
| WKWebView UI feels “web-ish”                               | Misses the polish goal          | Native menus, vibrancy, SF fonts, no web-style hover effects; a design review on every screen |
| Laptop limits (ephemeral ports, file descriptors) cap load | Misleading results at high load | Raise `ulimit` at launch, reuse connections, show a saturation warning                        |
| Feature-parity pressure from JMeter users                  | Scope creep                     | A clear non-goals list; JMX import reports unsupported elements instead of failing silently   |
| Coordinated omission hides latency under fixed-rate tests  | Optimistic percentiles          | Measure from the scheduled send time in paced mode; document the behaviour                    |
| Apple signing and notarisation friction                    | Slower releases                 | Automate in CI from the first beta                                                            |

**Decisions**

| Question       | Decision                            | What it changes                                                                                                                                                                                                                                                                                                                         |
| -------------- | ----------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan format    | YAML and JSON                       | YAML (`.barrage.yaml`) is the default for hand-editing; JSON (`.barrage.json`) is fully supported; the app and CLI open both                                                                                                                                                                                                            |
| Business model | Free and open source                | No paid tiers; team sharing, cloud runners and distributed agents are open source too                                                                                                                                                                                                                                                   |
| License        | MIT                                 | One short, permissive license that GitHub and package registries detect automatically; the repo ships a single `LICENSE` file and contributions are accepted under it                                                                                                                                                                   |
| Engine process | Sidecar process that uses all cores | The engine runs outside the app process with a multi-threaded runtime; an engine crash or saturation never freezes the UI                                                                                                                                                                                                               |
| Sidecar IPC    | stdio, newline-delimited JSON       | Tauri's built-in sidecar support spawns the engine and owns its pipes; no port or socket file to manage, nothing other local processes can connect to, and the engine exits with the app. Commands go in on stdin, snapshots come out on stdout, and logs go to stderr. Two 50 KB snapshots per second is far below what a pipe carries |
| Minimum macOS  | macOS 14 Sonoma                     | Newer WebKit in the web view and a smaller test matrix                                                                                                                                                                                                                                                                                  |

**Still open**

- [x] Final product name: “Barrage” is the working name. Check trademark and domain.

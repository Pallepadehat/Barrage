# Barrage Design System

> A calm, minimal, keyboard-first load-testing app for macOS.
> Built with React 19, shadcn/ui (BaseUI + Tailwind CSS v4) and lucide-react, inside a Tauri 2 window.

This document is the single source of truth for how Barrage looks and behaves. If the UI and this file disagree, one of them is a bug. Open a PR that fixes whichever is wrong.

---

## 1. Principles

1. **Quiet until it matters.** The interface is neutral greys. Colour appears only to mean something: the accent for "running / primary action", red for failure, amber for warning, green for a passing check. If you remove colour from a screen, it should still work.
2. **Content over chrome.** No cards inside cards, no drop shadows, no gradients. Use hairline borders and whitespace to separate things.
3. **One primary action per view.** Run in the toolbar, Send in the request editor. Everything else is secondary or ghost.
4. **Keyboard first, mouse friendly.** Every action has a shortcut and appears in the menu bar and the command palette (⌘K).
5. **Instant and reversible.** Edits apply as you type. There are no Save/Apply buttons inside forms, and everything can be undone (⌘Z).
6. **Feels like a Mac app.** System font, 13 px body text, native traffic lights, translucent sidebar, native menus. Never a website in a window.
7. **Numbers are the product.** Results use tabular figures, consistent units and alignment you can scan in a second.

---

## 2. Foundations

### 2.1 Colour tokens

We use shadcn's CSS-variable theme (Tailwind v4, OKLCH) with the **neutral** base, plus a small set of semantic tokens of our own. Components only ever reference tokens. Never write a raw hex or OKLCH value in a component.

| Token                | Light                     | Dark                      | Use                                      |
| -------------------- | ------------------------- | ------------------------- | ---------------------------------------- |
| `--background`       | `oklch(1 0 0)`            | `oklch(0.165 0 0)`        | Window content background                |
| `--sidebar`          | `oklch(0.975 0 0 / 0.72)` | `oklch(0.205 0 0 / 0.72)` | Sidebar (over vibrancy)                  |
| `--foreground`       | `oklch(0.165 0 0)`        | `oklch(0.97 0 0)`         | Primary text                             |
| `--muted`            | `oklch(0.97 0 0)`         | `oklch(0.235 0 0)`        | Subtle fills, hovered rows               |
| `--muted-foreground` | `oklch(0.52 0 0)`         | `oklch(0.68 0 0)`         | Secondary text, labels                   |
| `--border`           | `oklch(0.92 0 0)`         | `oklch(1 0 0 / 0.09)`     | Hairlines, inputs                        |
| `--ring`             | `var(--brand)`            | `var(--brand)`            | Focus ring                               |
| `--primary`          | `oklch(0.205 0 0)`        | `oklch(0.97 0 0)`         | Neutral primary buttons                  |
| `--brand`            | `oklch(0.57 0.19 256)`    | `oklch(0.68 0.17 256)`    | Run, active selection, main chart series |
| `--primary-foreground` | `oklch(0.985 0 0)`      | `oklch(0.205 0 0)`        | Text on `--primary` fills                |
| `--brand-foreground` | `oklch(1 0 0)`            | `oklch(0.165 0 0)`        | Text on `--brand` fills (Run)            |
| `--success`          | `oklch(0.63 0.15 150)`    | `oklch(0.72 0.15 150)`    | Passing assertions, 2xx                  |
| `--warning`          | `oklch(0.75 0.15 75)`     | `oklch(0.80 0.14 75)`     | Slow responses, 3xx/4xx, saturation      |
| `--destructive`      | `oklch(0.58 0.21 27)`     | `oklch(0.68 0.19 25)`     | Failures, 5xx, errors, Stop              |

**Rules**

- Use `--brand` sparingly: the Run button, the selected tree row, the main series in each chart, the focus ring. That's all.
- A status colour always comes with text or an icon. Colour alone never carries meaning (for accessibility and colour-blind users).
- The light `--brand` was darkened from L 0.60 so white text on it meets AA; see [ADR 0003](decisions/0003-light-brand-contrast.md).
- Light and dark follow the system (`prefers-color-scheme`). There is no in-app theme switch in v1.

### 2.2 Typography

```css
--font-sans:
  -apple-system, BlinkMacSystemFont, "SF Pro Text", "Inter", system-ui,
  sans-serif;
--font-mono: "SF Mono", ui-monospace, "JetBrains Mono", Menlo, monospace;
```

| Role           | Size / line height | Weight | Tailwind                                           | Used for                             |
| -------------- | ------------------ | ------ | -------------------------------------------------- | ------------------------------------ |
| Display number | 28 / 32            | 600    | `text-[28px] leading-8 font-semibold tabular-nums` | KPI tiles                            |
| Title          | 17 / 22            | 600    | `text-[17px] leading-[22px] font-semibold`         | Editor header (request name)         |
| Section        | 13 / 18            | 600    | `text-[13px] font-semibold`                        | Section headings in forms            |
| Body           | 13 / 18            | 400    | `text-[13px]`                                      | Default text, inputs, menus          |
| Small          | 12 / 16            | 400    | `text-xs`                                          | Tables, secondary text               |
| Caption        | 11 / 14            | 500    | `text-[11px] font-medium uppercase tracking-wide`  | Sidebar group labels, column headers |
| Code           | 12 / 18            | 400    | `font-mono text-xs`                                | URLs, headers, bodies, JSONPath      |

- **13 px is the body size**, as on macOS. Don't use 14 or 16 for UI text.
- All numbers in results use `tabular-nums` so columns don't jitter while values update.
- Use sentence case everywhere ("Add request", not "Add Request"). Captions are the only uppercase text.

### 2.3 Spacing, sizing and radius

- **4 px grid.** Use only the steps 4 · 8 · 12 · 16 · 24 · 32 (`1 2 3 4 6 8` in Tailwind).
- **Control height:** 28 px by default (`h-7`), 24 px when compact (`h-6`, in tables and toolbars), and 32 px for the primary Run button.
- **Radius:** `--radius: 0.5rem` (8 px) for panels and popovers, 6 px for controls (`rounded-md`), and 4 px for badges.
- **Borders:** 1 px `--border` only. No shadows, except on floating layers (popover, menu, command palette, toast), which use `shadow-lg` plus a border.

### 2.4 Iconography

- **lucide-react** at 16 px with a 1.5 stroke (`size-4 [stroke-width:1.5]`). Use 14 px inside compact controls.
- Icons go in `--muted-foreground` unless they are part of an active state.
- Never put an icon in a button whose label already says the same thing. Icon-only buttons always have a tooltip that names the action and its shortcut.

### 2.5 Motion

| Token           | Duration | Easing                       | Use                              |
| --------------- | -------- | ---------------------------- | -------------------------------- |
| `--motion-fast` | 120 ms   | `ease-out`                   | Hover, press, toggles            |
| `--motion-base` | 180 ms   | `cubic-bezier(0.2, 0, 0, 1)` | Popovers, drawers, tab indicator |
| `--motion-slow` | 260 ms   | `cubic-bezier(0.2, 0, 0, 1)` | Results drawer open/close        |

- Animate only `opacity` and `transform`.
- Live charts **do not animate** between updates. New data simply appears, which keeps the UI fast and honest.
- Respect `prefers-reduced-motion`: drop the animation and change instantly.

---

## 3. Layout

### 3.1 Window anatomy

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ ● ● ●   Checkout API  ▾              ▶ Run  ⌘R     00:01:23 · 200 users   ⌘K │ Toolbar 52
├────────────────┬─────────────────────────────────────────────────────────────┤
│ PLAN           │  POST  /login                                    Send  ⌘↩   │
│ ▾ Shoppers     │  ─────────────────────────────────────────────────────────  │
│   POST Login   │  Params   Headers   Body   Assertions 2   Extract 1         │
│   GET  Cart    │                                                             │
│   ⏸  500 ms    │  [ key                    ] [ value                     ]   │ Editor
│ ▸ Admins       │  [ key                    ] [ value                     ]   │
│                │                                                             │
│ DATA           ├─────────────────────────────────────────────────────────────┤
│   users.csv    │  Dashboard   Summary   Results   Errors              ⌄  ⤢   │
│                │  ┌────────┐┌────────┐┌────────┐┌────────┐┌────────┐         │ Results
│                │  │ 12.4k  ││ 1,204/s││ 0.3 %  ││  41 ms ││ 188 ms │         │ drawer
│ + Add          │  └────────┘└────────┘└────────┘└────────┘└────────┘         │
│                │  [ throughput chart ]       [ latency chart ]               │
└────────────────┴─────────────────────────────────────────────────────────────┘
  Sidebar 248          Main pane (min 560)
```

| Region         | Size                                   | Notes                                                                                                                                       |
| -------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Toolbar        | 52 px tall                             | Unified with the title bar. Traffic lights inset at x = 16. The whole bar is a drag region (`data-tauri-drag-region`), except its controls. |
| Sidebar        | 248 px (200–360, resizable)            | Vibrancy `sidebar`, translucent `--sidebar` fill. Collapse with ⌘0.                                                                         |
| Editor         | Fills the space                        | 24 px padding; content max-width 880 px, left-aligned.                                                                                      |
| Results drawer | 40 % of height by default (min 200 px) | shadcn `Resizable` panel. Opens automatically on Run. ⌘J toggles it, and ⤢ maximises it.                                                    |

Minimum window size is 960 × 640.

### 3.2 Sidebar (plan tree)

- Captions **PLAN** and **DATA** group the tree into thread groups with their steps, and CSV data sets.
- Row height 28 px, 8 px left padding plus 16 px per indent level.
- Each step row shows a method badge and its name. A disabled node is shown at 50 % opacity with its name struck through. There's no separate disabled icon.
- The selected row gets a `--brand` fill at 12 % with `--foreground` text. Hover uses `--muted`.
- Drag to reorder: a 2 px `--brand` insertion line marks the drop position, and a step can't be dropped outside a thread group.
- The right-click context menu (shadcn `ContextMenu`) offers Rename, Duplicate (⌘D), Enable/Disable (⌘E), Move up/down (⌥⌘↑/↓) and Delete (⌫).
- At the bottom, **+ Add** opens a menu: Thread group, HTTP request, Pause, CSV data set.

### 3.3 Toolbar

From left to right: traffic lights, a plan name menu (rename, reveal in Finder, recent plans), a flexible spacer, **Run / Stop**, the run status, and the command palette button.

- **Run** is the only filled `--brand` button in the app. While a test runs, it becomes **Stop** with the destructive outline style, in the same position and at the same width.
- **Run status** shows elapsed time and active users in tabular numbers, with a pulsing 6 px brand dot while running. When the run finishes, it shows "Finished · 2m 14s" for 10 s and then fades away.
- A **saturation warning** appears as an amber pill, "Generator saturated", with a tooltip explaining what it means.

---

## 4. Components

### 4.1 shadcn/ui components we use

The components live in `packages/ui`, generated with the shadcn CLI. **We restyle them only through tokens and a few documented variant changes.** Keep the edits small so upgrading stays easy.

| Component                               | Changes from shadcn defaults                                                                                                                    |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `Button`                                | Sizes `sm` (h-6), `default` (h-7), `lg` (h-8). New `brand` variant for Run. Ghost is the default for toolbar icons.                             |
| `Input`, `Textarea`                     | h-7, 13 px text, transparent background, and `--border` that turns into the brand ring on focus. Mono text for URL, header and JSONPath fields. |
| `Select`, `DropdownMenu`, `ContextMenu` | 13 px items, 28 px tall, with shortcuts right-aligned in `--muted-foreground`                                                                   |
| `Tabs`                                  | Underline style (no pill background), 2 px brand indicator, and an optional count badge                                                         |
| `Table`                                 | 28 px rows, 12 px text, sticky header, right-aligned numbers, zebra striping off                                                                |
| `Resizable`                             | 1 px handle that becomes 2 px brand on hover or drag                                                                                            |
| `Command`                               | Command palette (⌘K): every action, every node in the plan, and recent files                                                                    |
| `Tooltip`                               | 500 ms delay, 12 px text, shows the shortcut in a `Kbd`                                                                                         |
| `Switch`, `Checkbox`                    | Enable toggles in key-value rows and nodes                                                                                                      |
| `Badge`                                 | Method badges, status codes, counts                                                                                                             |
| `Sonner` (toast)                        | Bottom-right. Only for things outside the current view (saved, export done, run failed to start).                                               |
| `Dialog`                                | **Avoid.** Only for destructive confirmation (discard unsaved changes) and first-run onboarding.                                                |

### 4.2 Barrage components

| Component                       | Purpose                                 | Key details                                                                                                                                                                                 |
| ------------------------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `MethodBadge`                   | HTTP method                             | Mono 10 px bold, fixed width 40 px. Colours: GET `--success`, POST `--brand`, PUT/PATCH `--warning`, DELETE `--destructive`, others muted. Text colour only, no fill.                       |
| `StatusBadge`                   | Response code                           | `2xx` success, `3xx` muted, `4xx` warning, `5xx` / `0` (network error) destructive; tinted fill at 12 %                                                                                     |
| `KeyValueEditor`                | Params, headers, form fields, variables | Rows with an enable checkbox, key and value (mono). An empty trailing row adds a new one as you type. Paste `Key: Value` lines to bulk-add. Variable syntax `${…}` is highlighted in brand. |
| `VariableInput`                 | Any field that accepts `${…}`           | Autocompletes plan variables, extractor variables and built-in functions as you type `${`                                                                                                   |
| `CodeEditor`                    | Request and response bodies             | CodeMirror 6 with a minimal theme from our tokens, JSON highlighting and folding, read-only for responses                                                                                   |
| `AssertionRow` / `ExtractorRow` | Assertions and extractors               | A type select, a single-line expression and an inline result after Send (✓ in success or ✗ with the message in destructive)                                                                 |
| `StatTile`                      | KPI                                     | Caption label, display number and unit in muted text. There's no sparkline; the charts below do that job. Turns destructive when the error rate is above 1 %.                               |
| `LiveChart`                     | Time series                             | uPlot wrapper (see 5.2)                                                                                                                                                                     |
| `SampleList`                    | Results tree list                       | Virtualised (TanStack Virtual) rows: status badge, label, ms, time. Filters: All · Failures.                                                                                                |
| `SampleDetail`                  | One sample                              | Tabs: Response · Request · Assertions · Timing. Shows headers as a table and the body in `CodeEditor`.                                                                                      |
| `EmptyState`                    | Empty views                             | A 32 px lucide icon in muted colour, one sentence and one action. No illustrations.                                                                                                         |
| `Kbd`                           | Shortcut hint                           | Mono 11 px, `--muted` fill, 4 px radius                                                                                                                                                     |

---

## 5. Screens

### 5.1 Editors

**Thread group.** A single-column form with three groups separated by 24 px and caption headings:

1. _Load_: Users, Ramp-up (s), Stop after (duration _and/or_ iterations, shown as a segmented control: Duration · Iterations · Both)
2. _Pacing_: Think time (ms), Jitter (ms), Target throughput (req/s, optional)
3. _Summary_: a read-only sentence that updates live, e.g. "200 users start over 60 s and run for 5 min, about 400 req/s at most." It catches mistakes faster than a form.

**HTTP request.**

- The header row has a `MethodBadge` select, a URL `VariableInput` (mono, takes all the width) and a **Send** button (⌘↩).
- Tabs below it: Params · Headers · Body · Assertions · Extract. Counts show as a muted badge.
- After **Send**, a response panel slides in under the tabs: status badge · time · size, then the body. Assertion and extractor rows show their result inline. Send never opens a dialog.

**Plan.** Name, description, Variables (`KeyValueEditor`), and HTTP defaults (base URL, default headers, timeout, redirects, cookies).

### 5.2 Results drawer

**Dashboard (⌘1)**

- A row of six `StatTile`s: Requests · Throughput · Error rate · p50 · p95 · p99.
- A 2 × 2 grid of charts, each 180 px tall: Throughput (req/s) · Latency (p50, p95, p99) · Errors (/s) · Active users.

Chart rules (uPlot):

- No chart titles inside the canvas. A caption label sits above each chart.
- Axes and grid use `--muted-foreground` at 60 % and `--border`. No chart border.
- Series: the main metric is `--brand` at 1.5 px. For latency, p50 is `--muted-foreground`, p95 is `--brand` and p99 is `--brand` dashed. Errors use `--destructive` with a 10 % area fill. Users use a `--muted-foreground` step line.
- The legend sits inline above the chart with the current values in tabular numbers. Hovering shows a crosshair and every value at that time.
- The time axis shows elapsed time (`0:00`, `0:30`), not wall-clock time.
- The view keeps a rolling 10-minute window while the test runs and shows the full run when it ends.

**Summary (⌘2):** the aggregate table. Columns: Label · Requests · Errors · Error % · Mean · p50 · p90 · p95 · p99 · Max · req/s · KB/s. Click a header to sort. The Total row is pinned at the bottom in semibold. Cells over a threshold are tinted warning or destructive.

**Results (⌘3):** a split view. `SampleList` takes 40 % on the left, `SampleDetail` on the right. ↑/↓ move through samples.

**Errors (⌘4):** status code distribution (horizontal bars) and the top error messages with their counts.

### 5.3 Empty and first-run states

- **No plan open:** a welcome screen with "New plan", "Open…", the recent plans list and "Open example". Nothing else.
- **New plan:** starts with one thread group (10 users, 10 s ramp-up, 60 s) and one GET request with its URL field focused.
- **Results before any run:** "Run the plan to see live results. ⌘R".

---

## 6. Data formatting

| Value                | Format                                                | Example                      |
| -------------------- | ----------------------------------------------------- | ---------------------------- |
| Latency < 10 ms      | 1 decimal                                             | `4.2 ms`                     |
| Latency 10 ms – 10 s | Integer                                               | `188 ms`                     |
| Latency ≥ 10 s       | Seconds, 1 decimal                                    | `12.4 s`                     |
| Throughput           | Integer with thousands separator; 1 decimal under 10  | `1,204/s`, `3.5/s`           |
| Counts ≥ 10,000      | Compact                                               | `12.4k`, `1.2M`              |
| Percentages          | 1 decimal, `0 %` when zero; `< 0.1 %` for tiny values | `0.3 %`                      |
| Bytes                | Binary units                                          | `512 B`, `14.2 KB`, `3.1 MB` |
| Duration             | Clock                                                 | `00:01:23`                   |

All formatting lives in `lib/format.ts` (`formatMs`, `formatRate`, `formatCount`, `formatPercent`, `formatBytes`) and is unit-tested. Numbers are never formatted inline in components.

---

## 7. Interaction

### 7.1 Keyboard

| Shortcut        | Action                                     |
| --------------- | ------------------------------------------ |
| ⌘R / ⌘.         | Run / Stop                                 |
| ⌘↩              | Send selected request                      |
| ⌘K              | Command palette                            |
| ⌘N, ⌘O, ⌘S, ⇧⌘S | New, Open, Save, Save as                   |
| ⌘D, ⌫, ⌘E       | Duplicate, Delete, Enable/disable node     |
| ⌥⌘↑ / ⌥⌘↓       | Move node                                  |
| ⌘0              | Toggle sidebar                             |
| ⌘J              | Toggle results drawer                      |
| ⌘1–⌘4           | Results tabs                               |
| ⌘Z / ⇧⌘Z        | Undo / Redo                                |
| ↑ ↓ ← →         | Navigate the tree (← collapses, → expands) |

All shortcuts are registered in **one** place (`lib/shortcuts.ts`). That file feeds the native menu, the command palette and the tooltips, so they can't drift apart.

### 7.2 Focus and selection

- Focus rings are always visible for keyboard users: a 2 px `--ring` with a 2 px offset, using `focus-visible` only.
- Selecting a tree node moves focus to the first field of its editor only when you press Enter, never on a click.
- Deleting a node selects the next sibling, or the previous one if it was last.

### 7.3 Feedback

- **Saving:** a `•` after the plan name means unsaved changes, as in native Mac apps. ⌘S saves without a toast.
- **Validation:** inline under the field in destructive text at 12 px, shown on blur and never while typing. The tree marks a node with errors with a small destructive dot. Run is blocked until errors are fixed, and the tooltip says why.
- **Loading:** no spinners under 300 ms. Longer than that, show a 16 px spinner in place of the button icon.

---

## 8. macOS integration

- Window: `titleBarStyle: "Overlay"`, `hiddenTitle: true`, traffic lights at `{ x: 16, y: 20 }`, and vibrancy `sidebar` applied to the sidebar region.
- Native menu bar (built in Rust from `lib/shortcuts.ts`): Barrage · File · Edit · Plan · Run · View · Window · Help.
- Overlay scrollbars that appear on scroll. Never style them in a way that makes them always visible.
- Default cursor on all controls (no hand pointers on buttons), as in native apps.
- Text in the chrome isn't selectable. Values in results and response bodies are.
- Open and save use native dialogs. "Reveal in Finder" uses the native API.

---

## 9. Accessibility

- Text contrast is WCAG AA or better in both themes. Check tokens with the contrast script in `packages/ui/scripts`.
- Every icon-only button has an `aria-label`. Charts have an `aria-label` that summarises them and a "View as table" toggle.
- The plan tree uses the ARIA `tree` pattern (BaseUI-style roving tabindex).
- Live regions announce run start, run end and errors. Updating numbers are never announced.
- Everything works at 200 % zoom without horizontal scrolling in the editor.

---

## 10. Voice and microcopy

- Short, plain and specific. Name the thing: "Add request", "Run plan", "Stop".
- Errors say what happened and what to do: "Couldn't connect to api.example.com (connection refused). Check the base URL or that the server is running."
- No exclamation marks, no "Oops", no emoji.
- Units always follow numbers, with a space: `250 ms`, `40 req/s`.

---

## 11. Front-end code conventions

```
apps/desktop/src/
├─ app/                  # App shell, providers, window layout
├─ features/
│  ├─ plan-tree/         # Sidebar tree, drag & drop, context menu
│  ├─ plan-editor/       # Plan settings, variables, defaults
│  ├─ thread-group/      # Thread group editor
│  ├─ request-editor/    # Request tabs, send panel
│  ├─ results/           # Dashboard, summary, samples, errors
│  └─ run/               # Toolbar run controls, run store
├─ lib/                  # ipc.ts, format.ts, shortcuts.ts, cn.ts
└─ stores/               # plan-store.ts (Zustand + zundo undo history)

packages/ui/             # shadcn components + tokens.css (shared, no app logic)
packages/types/          # Generated from Rust with ts-rs, never edited by hand
```

- **Each feature folder owns its components, hooks and tests**, with an `index.ts` as its public surface. Features don't import from each other's internals.
- **Styling:** Tailwind classes plus `cn()`. No inline `style` except for computed geometry. No CSS modules. Tokens live only in `packages/ui/tokens.css`.
- **State:** the plan is in one Zustand store with Immer and zundo (undo/redo). Run data is in a separate store fed by IPC events and is never part of undo history.
- **IPC:** components never call Tauri directly. Everything goes through `lib/ipc.ts`, which is typed with the generated types and can be mocked in tests and in the browser preview.
- **Performance:** snapshot updates go to charts through refs (uPlot `setData`), not React state. Long lists are virtualised. Components that receive live data are memoised.
- **Naming:** `PascalCase.tsx` for components, `kebab-case.ts` for modules, `useThing` for hooks. One component per file.
- **Testing:** Vitest and Testing Library for components, plus a Playwright smoke test against the browser preview with a mocked IPC layer.

---

## 12. Definition of done for any UI change

- [ ] Uses tokens only, with no raw colours or ad-hoc spacing
- [ ] Works in light and dark
- [ ] Usable with the keyboard alone; shortcut registered in `lib/shortcuts.ts` if it's a new action
- [ ] Has empty, loading and error states
- [ ] Numbers formatted through `lib/format.ts`
- [ ] Screenshot (light + dark) in the PR description

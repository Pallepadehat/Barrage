# Plan file format

A plan is one file: `*.barrage.yaml` (default) or `*.barrage.json`. Both hold the same model, defined in `crates/plan/src/model/`, and convert into each other losslessly. See [`examples/checkout.barrage.yaml`](../examples/checkout.barrage.yaml).

**Rules**

- Keys are camelCase and always written in the order below, so diffs stay small.
- Unknown keys are an error (`file:line:column: unknown field ...`), so typos don't pass silently.
- Optional settings left at their default are not written. `enabled` is written only when `false`. The one exception is a CSV data set's `recycle`, which is always written.
- Text fields in steps, assertions, extractors and headers accept bare numbers and booleans (`expected: 200` is read as `"200"`), in their normalised form: write `'007'`, not `007`, when the exact spelling matters. Barrage always quotes such values when it saves. Numbers must be finite.
- Strings may contain `${name}` references; they're resolved at run time, not on load.

## Plan

| Key            | Type                 | Default | Notes                                     |
| -------------- | -------------------- | ------- | ----------------------------------------- |
| `name`         | string               | —       | Required                                  |
| `variables`    | list of `{key, value}` | `[]`  | Plan variables                            |
| `defaults`     | HTTP defaults        | `{}`    | See below                                 |
| `csvData`      | list of CSV data sets | `[]`   | See below                                 |
| `threadGroups` | list of thread groups | `[]`   | See below                                 |

**HTTP defaults:** `baseUrl`, `headers` (list of `{key, value, enabled}`), `timeoutMs`, `followRedirects`, `cookies`. All optional; unset means the engine default.

**CSV data set:** `path` (relative to the plan file), `variables` (column names; empty means the first row is a header), `delimiter` (one character, default `,`), `recycle` (default `true`; `false` stops the user at end of file).

## Thread group

`name`, `enabled`, `users`, `rampUpSecs` (default 0), `durationSecs`, `iterations`, `thinkTimeMs` (default 0), `thinkJitterMs` (default 0), `targetRps` (optional pacing, shared across the group), `steps`.

## Steps

Tagged by `type`.

- **`http`:** `name`, `enabled`, `method` (`GET` `POST` `PUT` `PATCH` `DELETE` `HEAD` `OPTIONS`), `url` (absolute, or relative to `baseUrl`), `query` and `headers` (lists of `{key, value, enabled}`), `bodyType` (`none` default, `json`, `text`, `form`), `body` (string; URL-encoded for `form`), `timeoutMs`, `assertions`, `extractors`.
- **`pause`:** `name`, `enabled`, `durationMs`.

## Assertions

Tagged by `type`.

| `type`         | Keys                                                                 |
| -------------- | -------------------------------------------------------------------- |
| `status`       | `expected`: e.g. `200,201`, `200-299`, `2xx`                         |
| `body`         | `check` (`contains`, `notContains`), `text`                          |
| `jsonPath`     | `expr` (RFC 9535), `check` (`exists`, `notExists`, `equals`, `contains`), `value` |
| `responseTime` | `maxMs`                                                              |
| `header`       | `name`, `check` (`present`, `contains`), `value`                     |

## Extractors

Tagged by `kind`. Each writes to `var`, scoped to the virtual user, and takes an optional `default` used when nothing matches.

| `kind`     | Keys                                       |
| ---------- | ------------------------------------------ |
| `jsonPath` | `expr` (RFC 9535); first match             |
| `regex`    | `expr`, `group` (default 1)                |
| `header`   | `name`                                     |

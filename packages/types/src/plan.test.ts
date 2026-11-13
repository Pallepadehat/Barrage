// SPDX-License-Identifier: MIT
// biome-ignore-all lint/suspicious/noTemplateCurlyInString: plan values use ${var} substitution syntax.

import { describe, expect, it } from "vitest";
import example from "../../../examples/checkout.barrage.json" with { type: "json" };
import type { Assertion, Plan, Step } from "./index";

/** The checkout example, typed against the generated `Plan`. */
const checkout = {
  name: "Checkout API",
  variables: [{ key: "host", value: "https://api.example.com" }],
  defaults: { baseUrl: "${host}", timeoutMs: 30000, cookies: true },
  csvData: [{ path: "users.csv", variables: ["email", "password"], recycle: true }],
  threadGroups: [
    {
      name: "Shoppers",
      users: 200,
      rampUpSecs: 60,
      durationSecs: 300,
      thinkTimeMs: 500,
      thinkJitterMs: 250,
      steps: [
        {
          type: "http",
          name: "Login",
          method: "POST",
          url: "/login",
          bodyType: "json",
          body: '{"email":"${email}","password":"${password}"}',
          assertions: [{ type: "status", expected: "2xx" }],
          extractors: [{ kind: "jsonPath", var: "token", expr: "$.token" }],
        },
      ],
    },
  ],
} satisfies Plan;

describe("generated plan types", () => {
  it("describe the checkout example file", () => {
    expect(example).toEqual(checkout);
  });

  it("reject shapes the plan reader rejects", () => {
    const pause: Step = { type: "pause", name: "Wait", durationMs: 100 };
    // @ts-expect-error optional text fields are left out, never null
    const nullBody: Step = { type: "http", name: "x", method: "GET", url: "/", body: null };
    // @ts-expect-error u64 fields are plain numbers, not bigint
    const bigTimeout: Assertion = { type: "responseTime", maxMs: 1n };
    // @ts-expect-error methods are upper case
    const lowerMethod: Step = { type: "http", name: "x", method: "get", url: "/" };
    expect([pause, nullBody, bigTimeout, lowerMethod]).toHaveLength(4);
  });
});

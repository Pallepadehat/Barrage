// SPDX-License-Identifier: MIT

/**
 * Reads the colour tokens out of `tokens.css` for each theme. Deliberately
 * small: it understands the file's own shape (a top-level `:root` block and a
 * `prefers-color-scheme: dark` override), not CSS in general.
 */

/** Theme name. */
export type Theme = "light" | "dark";

/** Custom properties of one theme, with `var()` references resolved. */
export type TokenMap = Map<string, string>;

const DECLARATION = /(--[\w-]+)\s*:\s*([^;]+);/g;
const VAR_REF = /^var\(\s*(--[\w-]+)\s*\)$/;

/** Returns the body of the first `{ … }` block after `from`, honouring nesting. */
function blockAfter(css: string, from: number): string {
  const open = css.indexOf("{", from);
  if (open < 0) {
    throw new Error("tokens.css: expected a block");
  }
  let depth = 0;
  for (let i = open; i < css.length; i++) {
    if (css[i] === "{") depth++;
    if (css[i] === "}" && --depth === 0) return css.slice(open + 1, i);
  }
  throw new Error("tokens.css: unbalanced braces");
}

function declarations(block: string): Map<string, string> {
  const out = new Map<string, string>();
  for (const [, name, value] of block.matchAll(DECLARATION)) {
    if (name !== undefined && value !== undefined) out.set(name, value.trim());
  }
  return out;
}

function resolve(raw: Map<string, string>): TokenMap {
  const lookup = (name: string, seen: Set<string>): string => {
    const value = raw.get(name);
    if (value === undefined) throw new Error(`tokens.css: ${name} is not defined`);
    const ref = VAR_REF.exec(value)?.[1];
    if (ref === undefined) return value;
    if (seen.has(ref)) throw new Error(`tokens.css: ${name} refers to itself`);
    return lookup(ref, new Set(seen).add(ref));
  };
  return new Map([...raw.keys()].map((name) => [name, lookup(name, new Set([name]))]));
}

/** Parses `tokens.css` source into resolved light and dark token maps. */
export function parseTokens(css: string): Record<Theme, TokenMap> {
  const source = css.replace(/\/\*[\s\S]*?\*\//g, "");
  const root = /(^|})\s*:root\s*\{/.exec(source);
  const dark = /@media\s*\(\s*prefers-color-scheme:\s*dark\s*\)/.exec(source);
  if (root === null || dark === null) {
    throw new Error("tokens.css: expected a :root block and a dark-scheme override");
  }
  const light = declarations(blockAfter(source, root.index));
  const darkRoot = blockAfter(blockAfter(source, dark.index), 0);
  const merged = new Map([...light, ...declarations(darkRoot)]);
  return { light: resolve(light), dark: resolve(merged) };
}

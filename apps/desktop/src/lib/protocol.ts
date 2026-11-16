// SPDX-License-Identifier: MIT

/**
 * Protocol helpers shared by the real IPC and its mock. The message types
 * themselves are generated from Rust into `@barrage/types`.
 */

/** Protocol version spoken on the wire; mirrors `PROTOCOL_VERSION` in `barrage-protocol`. */
export const PROTOCOL_VERSION = 0;

/** A request id source for one connection: 1, 2, 3… (0 is the shell's own shutdown). */
export function requestIds(): () => number {
  let next = 1;
  return () => next++;
}

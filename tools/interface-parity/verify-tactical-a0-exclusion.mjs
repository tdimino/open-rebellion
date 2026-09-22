#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { forbiddenTrackedA0Paths } from "./tactical-matrix.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const tracked = execFileSync("git", ["ls-files", "-z"], {
  cwd: root,
  encoding: "utf8",
})
  .split("\0")
  .filter(Boolean);
const forbidden = forbiddenTrackedA0Paths(tracked);
if (forbidden.length > 0) {
  throw new Error(`proprietary tactical A0 artifacts are tracked:\n${forbidden.join("\n")}`);
}
process.stdout.write(`${JSON.stringify({
  status: "pass",
  tracked_paths_checked: tracked.length,
  forbidden_a0_paths: 0,
}, null, 2)}\n`);

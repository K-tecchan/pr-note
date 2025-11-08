#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { arch, platform } from "node:process";
import { fileURLToPath } from "node:url";

const PLATFORMS = {
  darwin: {
    arm64: "pr-note-darwin-arm64/pr-note",
    x64: "pr-note-darwin-x64/pr-note",
  },
  linux: {
    arm64: "pr-note-linux-arm64/pr-note",
    x64: "pr-note-linux-x64/pr-note",
  },
  win32: {
    arm64: "pr-note-win32-arm64/pr-note.exe",
    x64: "pr-note-win32-x64/pr-note.exe",
  },
};

function main() {
  const bin = PLATFORMS?.[platform]?.[arch];
  if (!bin) {
    console.error(`Unsupported platform or architecture: ${platform} ${arch}`);
    console.error(`Supported combinations are:`);
    for (const [plt, archs] of Object.entries(PLATFORMS)) {
      for (const arch of Object.keys(archs)) {
        console.error(`  - ${plt} ${arch}`);
      }
    }

    process.exit(1);
  }

  const currentDir = dirname(fileURLToPath(import.meta.url));
  const binPath = resolve(currentDir, "../../", bin);
  const result = spawnSync(binPath, process.argv.slice(2), {
    stdio: "inherit",
  });

  process.exit(result.status ?? 1);
}

main();

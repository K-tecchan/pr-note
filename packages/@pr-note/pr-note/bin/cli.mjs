#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { arch, platform } from "node:process";

const PLATFORMS = {
  darwin: {
    arm64: "@pr-note/pr-note-darwin-arm64/pr-note",
    x64: "@pr-note/pr-note-darwin-x64/pr-note",
  },
  linux: {
    arm64: "@pr-note/pr-note-linux-arm64/pr-note",
    x64: "@pr-note/pr-note-linux-x64/pr-note",
  },
  win32: {
    arm64: "@pr-note/pr-note-win32-arm64/pr-note.exe",
    x64: "@pr-note/pr-note-win32-x64/pr-note.exe",
  },
};

function resolveBinaryPath(bin) {
  try {
    return import.meta.resolve(bin);
  } catch {
    console.error(`Failed to resolve binary path for "${bin}".`);
    console.error("Make sure the package is installed correctly.");
    process.exit(1);
  }
}

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

  const binPath = resolveBinaryPath(bin);
  const result = spawnSync(binPath, process.argv.slice(2), {
    stdio: "inherit",
  });

  process.exit(result.status ?? 1);
}

main();

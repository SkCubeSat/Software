#!/usr/bin/env node

import { spawn } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DOCS_ROOT = path.resolve(__dirname, '..');

function spawnProc(cmd, args, name) {
  const child = spawn(cmd, args, {
    cwd: DOCS_ROOT,
    stdio: 'inherit',
    env: process.env,
  });

  child.on('error', (error) => {
    console.error(`${name} failed to start: ${error.message}`);
  });

  return child;
}

async function runInitialPlantuml() {
  await new Promise((resolve, reject) => {
    const child = spawnProc('node', ['./scripts/generate-plantuml.mjs'], 'plantuml');
    child.on('exit', (code, signal) => {
      if (signal) {
        reject(new Error(`plantuml generator terminated by signal ${signal}`));
        return;
      }
      if (code !== 0) {
        reject(new Error(`plantuml generator exited with code ${code}`));
        return;
      }
      resolve();
    });
  });
}

async function main() {
  // Keep existing behavior: generate PlantUML before starting Next dev.
  await runInitialPlantuml();

  const plantumlWatcher = spawnProc('node', ['./scripts/generate-plantuml.mjs', '--watch'], 'plantuml-watch');
  const drawioWatcher = spawnProc('node', ['./scripts/generate-drawio.mjs', '--watch'], 'drawio-watch');
  const nextDev = spawnProc('next', ['dev'], 'next-dev');

  let shuttingDown = false;
  const shutdown = (signal = 'SIGTERM') => {
    if (shuttingDown) return;
    shuttingDown = true;
    for (const child of [plantumlWatcher, drawioWatcher, nextDev]) {
      if (!child.killed) {
        try {
          child.kill(signal);
        } catch {
          // ignore
        }
      }
    }
  };

  process.on('SIGINT', () => shutdown('SIGINT'));
  process.on('SIGTERM', () => shutdown('SIGTERM'));

  nextDev.on('exit', (code, signal) => {
    shutdown();
    if (signal) {
      process.exit(1);
      return;
    }
    process.exit(code ?? 0);
  });

  drawioWatcher.on('exit', (code, signal) => {
    if (shuttingDown) return;
    console.error(
      `drawio-watch exited unexpectedly${signal ? ` (signal ${signal})` : ` (code ${code ?? 0})`}`,
    );
    shutdown();
    process.exit(code ?? 1);
  });

  plantumlWatcher.on('exit', (code, signal) => {
    if (shuttingDown) return;
    console.error(
      `plantuml-watch exited unexpectedly${signal ? ` (signal ${signal})` : ` (code ${code ?? 0})`}`,
    );
    shutdown();
    process.exit(code ?? 1);
  });
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
});

#!/usr/bin/env node

import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DOCS_ROOT = path.resolve(__dirname, '..');
const CONTENT_ROOT = path.join(DOCS_ROOT, 'content', 'docs', 'kubos');
const OUTPUT_DIR = path.join(DOCS_ROOT, 'public', 'kubos', 'diagrams');
const CACHE_DIR = path.join(DOCS_ROOT, '.cache', 'plantuml');

const FENCE_RE = /```[^\n]*\n([\s\S]*?)```/g;

function walkFiles(dir) {
  const out = [];
  for (const name of readdirSync(dir)) {
    const full = path.join(dir, name);
    const st = statSync(full);
    if (st.isDirectory()) out.push(...walkFiles(full));
    else if (st.isFile() && full.endsWith('.mdx')) out.push(full);
  }
  return out;
}

function extractPlantUmlBlocks(content) {
  const blocks = [];
  let match;
  while ((match = FENCE_RE.exec(content)) !== null) {
    const body = match[1].replace(/\r\n/g, '\n').trim();
    if (body.includes('@startuml') && body.includes('@enduml')) {
      blocks.push(body);
    }
  }
  return blocks;
}

function diagramHash(source) {
  return createHash('sha256').update(source).digest('hex').slice(0, 16);
}

function resolvePumlBinary() {
  const localUnix = path.join(DOCS_ROOT, 'node_modules', '.bin', 'puml');
  const localWin = path.join(DOCS_ROOT, 'node_modules', '.bin', 'puml.cmd');
  if (process.platform === 'win32' && existsSync(localWin)) return localWin;
  if (existsSync(localUnix)) return localUnix;
  return 'puml';
}

function renderSvg({ pumlBin, source, hash }) {
  const srcPath = path.join(CACHE_DIR, `${hash}.puml`);
  const outPath = path.join(OUTPUT_DIR, `${hash}.svg`);
  if (existsSync(outPath)) return outPath;

  writeFileSync(srcPath, `${source}\n`, 'utf8');

  const res = spawnSync(
    pumlBin,
    ['generate', srcPath, '--svg', '--output', outPath],
    {
      cwd: DOCS_ROOT,
      encoding: 'utf8',
      env: process.env,
    },
  );

  if (res.status !== 0) {
    const stderr = (res.stderr || '').trim();
    const stdout = (res.stdout || '').trim();
    const details = [stderr, stdout].filter(Boolean).join('\n');
    throw new Error(
      `PlantUML render failed for ${path.basename(srcPath)} using "${pumlBin}".` +
        (details ? `\n${details}` : ''),
    );
  }

  return outPath;
}

function main() {
  mkdirSync(OUTPUT_DIR, { recursive: true });
  mkdirSync(CACHE_DIR, { recursive: true });

  const mdxFiles = existsSync(CONTENT_ROOT) ? walkFiles(CONTENT_ROOT) : [];
  const uniqueSources = new Map();

  for (const file of mdxFiles) {
    const content = readFileSync(file, 'utf8');
    for (const source of extractPlantUmlBlocks(content)) {
      const hash = diagramHash(source);
      if (!uniqueSources.has(hash)) {
        uniqueSources.set(hash, source);
      }
    }
  }

  const pumlBin = resolvePumlBinary();
  let rendered = 0;
  let skipped = 0;

  for (const [hash, source] of uniqueSources) {
    const outPath = path.join(OUTPUT_DIR, `${hash}.svg`);
    if (existsSync(outPath)) {
      skipped += 1;
      continue;
    }
    renderSvg({ pumlBin, source, hash });
    rendered += 1;
  }

  // Remove stale diagram files that are no longer referenced.
  const keep = new Set([...uniqueSources.keys()].map((hash) => `${hash}.svg`));
  if (existsSync(OUTPUT_DIR)) {
    for (const name of readdirSync(OUTPUT_DIR)) {
      if (!name.endsWith('.svg')) continue;
      if (!keep.has(name)) {
        rmSync(path.join(OUTPUT_DIR, name), { force: true });
      }
    }
  }

  console.log(
    `PlantUML: found ${uniqueSources.size} diagram source(s), rendered ${rendered}, reused ${skipped}.`,
  );
}

try {
  main();
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(message);
  console.error(
    'PlantUML rendering requires node-plantuml plus Java and Graphviz (dot) to be installed in the build environment.',
  );
  process.exit(1);
}


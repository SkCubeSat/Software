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
const OUTPUT_ROOT = path.join(DOCS_ROOT, 'public', 'kubos', 'diagrams');
const LIGHT_OUTPUT_DIR = path.join(OUTPUT_ROOT, 'light');
const DARK_OUTPUT_DIR = path.join(OUTPUT_ROOT, 'dark');
const CACHE_DIR = path.join(DOCS_ROOT, '.cache', 'plantuml');
const DARK_THEME_VERSION = 'v1';

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

function renderSvg({ pumlBin, source, hash, variant, force = false }) {
  const srcPath = path.join(CACHE_DIR, `${hash}.${variant}.puml`);
  const outDir = variant === 'dark' ? DARK_OUTPUT_DIR : LIGHT_OUTPUT_DIR;
  const outPath = path.join(outDir, `${hash}.svg`);
  if (!force && existsSync(outPath)) return outPath;

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

function darkMarker(hash) {
  return `<!-- codex-plantuml-dark:${DARK_THEME_VERSION}:${hash} -->`;
}

function isDarkVariantCurrent(hash) {
  const darkPath = path.join(DARK_OUTPUT_DIR, `${hash}.svg`);
  if (!existsSync(darkPath)) return false;
  const svg = readFileSync(darkPath, 'utf8');
  return svg.includes(darkMarker(hash));
}

function buildDarkPlantUmlSource(source) {
  const darkSkinparams = [
    'skinparam backgroundColor transparent',
    'skinparam shadowing false',
    'skinparam defaultFontColor #E5E7EB',
    'skinparam hyperlinkColor #93C5FD',
    'skinparam ArrowColor #93C5FD',
    'skinparam LineColor #93C5FD',
    'skinparam NoteBackgroundColor #111827',
    'skinparam NoteBorderColor #93C5FD',
    'skinparam NoteFontColor #E5E7EB',
    'skinparam ActorBackgroundColor #0F172A',
    'skinparam ActorBorderColor #93C5FD',
    'skinparam ActorFontColor #E5E7EB',
    'skinparam ParticipantBackgroundColor #111827',
    'skinparam ParticipantBorderColor #93C5FD',
    'skinparam ParticipantFontColor #E5E7EB',
    'skinparam LifeLineBorderColor #64748B',
    'skinparam LifeLineBackgroundColor #0F172A',
    'skinparam SequenceLifeLineBorderColor #64748B',
    'skinparam SequenceLifeLineBackgroundColor #0F172A',
    'skinparam SequenceGroupBorderColor #93C5FD',
    'skinparam SequenceGroupBackgroundColor #0B1220',
    'skinparam SequenceBoxBorderColor #93C5FD',
    'skinparam SequenceBoxBackgroundColor #0B1220',
    'skinparam SequenceReferenceBorderColor #93C5FD',
    'skinparam SequenceReferenceBackgroundColor #111827',
    'skinparam PackageBackgroundColor #0B1220',
    'skinparam PackageBorderColor #93C5FD',
    'skinparam PackageFontColor #E5E7EB',
    'skinparam RectangleBackgroundColor #111827',
    'skinparam RectangleBorderColor #93C5FD',
    'skinparam RectangleFontColor #E5E7EB',
  ].join('\n');

  let out = source;
  out = out.replace(/^@startuml[^\n]*\n/, (match) => `${match}\n' codex dark variant ${DARK_THEME_VERSION}\n${darkSkinparams}\n\n`);

  // Some KubOS diagrams force a bright sequence box explicitly.
  out = out.replace(/\\?#LightBlue\b/g, '#1F2937');

  return out;
}

function ensureDarkVariant({ pumlBin, source, hash }) {
  const darkPath = path.join(DARK_OUTPUT_DIR, `${hash}.svg`);
  if (isDarkVariantCurrent(hash)) return darkPath;

  const darkSource = buildDarkPlantUmlSource(source);
  renderSvg({ pumlBin, source: darkSource, hash, variant: 'dark', force: true });

  // Mark the SVG so we can detect stale dark variants after theme generator changes.
  const svg = readFileSync(darkPath, 'utf8');
  const marked = svg.replace(/<svg([^>]*)>/, `<svg$1>\n${darkMarker(hash)}`);
  writeFileSync(darkPath, marked, 'utf8');
  return darkPath;
}

function main() {
  mkdirSync(LIGHT_OUTPUT_DIR, { recursive: true });
  mkdirSync(DARK_OUTPUT_DIR, { recursive: true });
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
    const lightOutPath = path.join(LIGHT_OUTPUT_DIR, `${hash}.svg`);
    if (existsSync(lightOutPath) && isDarkVariantCurrent(hash)) {
      skipped += 1;
      continue;
    }
    if (!existsSync(lightOutPath)) {
      renderSvg({ pumlBin, source, hash, variant: 'light' });
    }
    ensureDarkVariant({ pumlBin, source, hash });
    rendered += 1;
  }

  // If a light SVG already existed but the dark SVG is missing (or generator behavior changed),
  // ensure dark variants are present.
  for (const hash of uniqueSources.keys()) {
    const lightOutPath = path.join(LIGHT_OUTPUT_DIR, `${hash}.svg`);
    const darkOutPath = path.join(DARK_OUTPUT_DIR, `${hash}.svg`);
    if (existsSync(lightOutPath) && !isDarkVariantCurrent(hash)) {
      ensureDarkVariant({ pumlBin, source: uniqueSources.get(hash), hash });
    }
  }

  // Remove stale diagram files that are no longer referenced.
  const keep = new Set([...uniqueSources.keys()].map((hash) => `${hash}.svg`));
  for (const dir of [LIGHT_OUTPUT_DIR, DARK_OUTPUT_DIR]) {
    if (!existsSync(dir)) continue;
    for (const name of readdirSync(dir)) {
      if (!name.endsWith('.svg')) continue;
      if (!keep.has(name)) {
        rmSync(path.join(dir, name), { force: true });
      }
    }
  }

  // Remove legacy single-set outputs from the old path scheme (/kubos/diagrams/<hash>.svg).
  if (existsSync(OUTPUT_ROOT)) {
    for (const name of readdirSync(OUTPUT_ROOT)) {
      const full = path.join(OUTPUT_ROOT, name);
      if (statSync(full).isDirectory()) continue;
      if (name.endsWith('.svg')) {
        rmSync(full, { force: true });
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

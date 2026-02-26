#!/usr/bin/env node

import { createHash } from 'node:crypto';
import {
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DOCS_ROOT = path.resolve(__dirname, '..');
const CONTENT_ROOT = path.join(DOCS_ROOT, 'content', 'docs');
const OUTPUT_ROOT = path.join(DOCS_ROOT, 'public', 'kubos', 'diagrams');
const LIGHT_OUTPUT_DIR = path.join(OUTPUT_ROOT, 'light');
const DARK_OUTPUT_DIR = path.join(OUTPUT_ROOT, 'dark');
const CACHE_DIR = path.join(DOCS_ROOT, '.cache', 'plantuml');
const DARK_THEME_VERSION = 'v2';
const WATCH_DEBOUNCE_MS = Number(process.env.PLANTUML_WATCH_DEBOUNCE_MS || '300');
const WATCH_STABLE_MS = Number(process.env.PLANTUML_WATCH_STABLE_MS || '500');
const WATCH_STABLE_POLL_MS = Number(process.env.PLANTUML_WATCH_STABLE_POLL_MS || '120');
const WATCH_SCAN_INTERVAL_MS = Number(process.env.PLANTUML_WATCH_SCAN_INTERVAL_MS || '800');

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
  // Palette tuned to match the Fumadocs app dark theme (docs/src/app/global.css).
  const darkSkinparams = [
    'skinparam backgroundColor transparent',
    'skinparam shadowing false',
    'skinparam roundCorner 12',
    'skinparam defaultFontColor #EAF2FF',
    'skinparam hyperlinkColor #89BBFF',
    'skinparam ArrowColor #78AFFF',
    'skinparam LineColor #6EA8FF',
    'skinparam ArrowThickness 1.3',
    'skinparam NoteBackgroundColor #111A2C',
    'skinparam NoteBorderColor #5E90DA',
    'skinparam NoteFontColor #EAF2FF',
    'skinparam NoteShadowing false',
    'skinparam TitleFontColor #F4F8FF',
    'skinparam CaptionFontColor #C4D3EE',
    'skinparam LegendBackgroundColor #0F172A',
    'skinparam LegendBorderColor #5E90DA',
    'skinparam LegendFontColor #EAF2FF',
    'skinparam ActorBackgroundColor #0F172A',
    'skinparam ActorBorderColor #78AFFF',
    'skinparam ActorFontColor #EAF2FF',
    'skinparam ParticipantBackgroundColor #131C2E',
    'skinparam ParticipantBorderColor #6EA8FF',
    'skinparam ParticipantFontColor #EAF2FF',
    'skinparam LifeLineBorderColor #425675',
    'skinparam LifeLineBackgroundColor #0F172A',
    'skinparam SequenceLifeLineBorderColor #425675',
    'skinparam SequenceLifeLineBackgroundColor #0F172A',
    'skinparam SequenceGroupBorderColor #6EA8FF',
    'skinparam SequenceGroupBackgroundColor #0E1626',
    'skinparam SequenceBoxBorderColor #6EA8FF',
    'skinparam SequenceBoxBackgroundColor #0E1626',
    'skinparam SequenceReferenceBorderColor #6EA8FF',
    'skinparam SequenceReferenceBackgroundColor #131C2E',
    'skinparam PackageBackgroundColor #0F172A',
    'skinparam PackageBorderColor #6EA8FF',
    'skinparam PackageFontColor #EAF2FF',
    'skinparam FolderBackgroundColor #0F172A',
    'skinparam FolderBorderColor #6EA8FF',
    'skinparam FolderFontColor #EAF2FF',
    'skinparam FrameBackgroundColor #0F172A',
    'skinparam FrameBorderColor #6EA8FF',
    'skinparam FrameFontColor #EAF2FF',
    'skinparam NodeBackgroundColor #10192B',
    'skinparam NodeBorderColor #6EA8FF',
    'skinparam NodeFontColor #EAF2FF',
    'skinparam RectangleBackgroundColor #131C2E',
    'skinparam RectangleBorderColor #6EA8FF',
    'skinparam RectangleFontColor #EAF2FF',
    'skinparam EntityBackgroundColor #162238',
    'skinparam EntityBorderColor #6EA8FF',
    'skinparam EntityFontColor #EAF2FF',
    'skinparam ComponentBackgroundColor #162238',
    'skinparam ComponentBorderColor #6EA8FF',
    'skinparam ComponentFontColor #EAF2FF',
    'skinparam CloudBackgroundColor #17263F',
    'skinparam CloudBorderColor #78AFFF',
    'skinparam CloudFontColor #EAF2FF',
    'skinparam DatabaseBackgroundColor #162238',
    'skinparam DatabaseBorderColor #6EA8FF',
    'skinparam DatabaseFontColor #EAF2FF',
    'skinparam StorageBackgroundColor #162238',
    'skinparam StorageBorderColor #6EA8FF',
    'skinparam StorageFontColor #EAF2FF',
    'skinparam QueueBackgroundColor #162238',
    'skinparam QueueBorderColor #6EA8FF',
    'skinparam QueueFontColor #EAF2FF',
    'skinparam UsecaseBackgroundColor #17263F',
    'skinparam UsecaseBorderColor #78AFFF',
    'skinparam UsecaseFontColor #EAF2FF',
    'skinparam ActivityBackgroundColor #162238',
    'skinparam ActivityBorderColor #6EA8FF',
    'skinparam ActivityFontColor #EAF2FF',
    'skinparam DiamondBackgroundColor #17263F',
    'skinparam DiamondBorderColor #78AFFF',
    'skinparam DiamondFontColor #EAF2FF',
    'skinparam StartColor #78AFFF',
    'skinparam EndColor #78AFFF',
    'skinparam BarColor #6EA8FF',
    'skinparam StateBackgroundColor #162238',
    'skinparam StateBorderColor #6EA8FF',
    'skinparam StateFontColor #EAF2FF',
    'skinparam ClassBackgroundColor #131C2E',
    'skinparam ClassBorderColor #6EA8FF',
    'skinparam ClassFontColor #EAF2FF',
    'skinparam ClassHeaderBackgroundColor #162238',
    'skinparam ObjectBackgroundColor #131C2E',
    'skinparam ObjectBorderColor #6EA8FF',
    'skinparam ObjectFontColor #EAF2FF',
  ].join('\n');

  let out = source;
  out = out.replace(/^@startuml[^\n]*\n/, (match) => `${match}\n' codex dark variant ${DARK_THEME_VERSION}\n${darkSkinparams}\n\n`);

  // Some KubOS diagrams force a bright sequence box explicitly.
  out = out.replace(/\\?#LightBlue\b/g, '#1F2937');

  return out;
}

function postProcessDarkSvg(svg) {
  // Old PlantUML versions may ignore some per-shape skinparams. Rewrite common light defaults.
  return svg
    .replaceAll('#FEFECE', '#162238')
    .replaceAll('#A80036', '#6EA8FF')
    .replaceAll('stroke: #000000', 'stroke: #6EA8FF')
    .replaceAll('fill="#FFFFFF"', 'fill="#0F172A"');
}

function ensureDarkVariant({ pumlBin, source, hash }) {
  const darkPath = path.join(DARK_OUTPUT_DIR, `${hash}.svg`);
  if (isDarkVariantCurrent(hash)) return darkPath;

  const darkSource = buildDarkPlantUmlSource(source);
  renderSvg({ pumlBin, source: darkSource, hash, variant: 'dark', force: true });

  // Mark the SVG so we can detect stale dark variants after theme generator changes.
  const svg = postProcessDarkSvg(readFileSync(darkPath, 'utf8'));
  const marked = svg.replace(/<svg([^>]*)>/, `<svg$1>\n${darkMarker(hash)}`);
  writeFileSync(darkPath, marked, 'utf8');
  return darkPath;
}

function generatePlantumlDiagrams({ pruneStale = true } = {}) {
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
  if (pruneStale) {
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

function logFatal(error) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(message);
  console.error(
    'PlantUML rendering requires node-plantuml plus Java and Graphviz (dot) to be installed in the build environment.',
  );
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function statFingerprint(filePath) {
  try {
    const st = statSync(filePath);
    return `${st.size}:${Math.floor(st.mtimeMs)}`;
  } catch {
    return 'missing';
  }
}

function snapshotMdxFiles(dir) {
  const files = existsSync(dir) ? walkFiles(dir) : [];
  return new Map(files.map((file) => [file, statFingerprint(file)]));
}

async function waitForFilesStable(filePaths) {
  const mdxFiles = [...new Set(filePaths.filter((p) => typeof p === 'string' && p.endsWith('.mdx')))];
  if (mdxFiles.length === 0) return;

  let previous = new Map();
  let stableSince = 0;
  const deadline = Date.now() + Math.max(WATCH_STABLE_MS * 10, 2000);

  while (Date.now() <= deadline) {
    const current = new Map(mdxFiles.map((p) => [p, statFingerprint(p)]));
    let changed = current.size !== previous.size;
    if (!changed) {
      for (const [key, value] of current) {
        if (previous.get(key) !== value) {
          changed = true;
          break;
        }
      }
    }

    if (changed) {
      previous = current;
      stableSince = Date.now();
    } else if (stableSince && Date.now() - stableSince >= WATCH_STABLE_MS) {
      return;
    } else if (!stableSince) {
      stableSince = Date.now();
    }

    await delay(WATCH_STABLE_POLL_MS);
  }
}

async function runWatchMode() {
  console.log(`PlantUML watch: watching ${CONTENT_ROOT} for .mdx changes...`);

  let isRunning = false;
  let rerunRequested = false;
  let debounceTimer = null;
  const pendingChangedPaths = new Set();

  const runGeneration = async () => {
    if (isRunning) {
      rerunRequested = true;
      return;
    }

    const changedPaths = [...pendingChangedPaths];
    pendingChangedPaths.clear();
    isRunning = true;
    try {
      await waitForFilesStable(changedPaths);
      // In watch mode, keep old hashed assets around to avoid hot-reload races.
      generatePlantumlDiagrams({ pruneStale: false });
    } catch (error) {
      logFatal(error);
    } finally {
      isRunning = false;
      if (rerunRequested) {
        rerunRequested = false;
        queueRun('queued change');
      }
    }
  };

  const queueRun = (reason) => {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      debounceTimer = null;
      console.log(`PlantUML watch: change detected (${reason}), regenerating...`);
      void runGeneration();
    }, WATCH_DEBOUNCE_MS);
  };

  void runGeneration();

  let mdxSnapshot = snapshotMdxFiles(CONTENT_ROOT);
  const intervalId = setInterval(() => {
    const nextSnapshot = snapshotMdxFiles(CONTENT_ROOT);
    const changed = [];

    for (const [file, fp] of nextSnapshot) {
      if (mdxSnapshot.get(file) !== fp) changed.push(file);
    }
    for (const file of mdxSnapshot.keys()) {
      if (!nextSnapshot.has(file)) changed.push(file);
    }

    if (changed.length > 0) {
      for (const file of changed) pendingChangedPaths.add(file);
      queueRun(changed.length === 1 ? path.relative(CONTENT_ROOT, changed[0]) : `${changed.length} files`);
      mdxSnapshot = nextSnapshot;
    }
  }, WATCH_SCAN_INTERVAL_MS);

  const shutdown = () => {
    if (debounceTimer) clearTimeout(debounceTimer);
    clearInterval(intervalId);
    process.exit(0);
  };
  process.on('SIGINT', shutdown);
  process.on('SIGTERM', shutdown);
}

const isWatchMode = process.argv.includes('--watch');

if (isWatchMode) {
  runWatchMode().catch((error) => {
    logFatal(error);
    process.exit(1);
  });
} else {
  try {
    generatePlantumlDiagrams({ pruneStale: true });
  } catch (error) {
    logFatal(error);
    process.exit(1);
  }
}

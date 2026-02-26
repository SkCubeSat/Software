#!/usr/bin/env node

import {
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  copyFileSync,
  watch as fsWatch,
  utimesSync,
} from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import {
  drawioDiagramHash,
  drawioDevDiagramKey,
  parseDrawioFenceMeta,
} from '../src/lib/drawio-static-shared.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DOCS_ROOT = path.resolve(__dirname, '..');
const CONTENT_ROOT = path.join(DOCS_ROOT, 'content', 'docs');
const OUTPUT_ROOT = path.join(DOCS_ROOT, 'public', 'kubos', 'diagrams', 'drawio');
const LIGHT_OUTPUT_DIR = path.join(OUTPUT_ROOT, 'light');
const DARK_OUTPUT_DIR = path.join(OUTPUT_ROOT, 'dark');
const LIGHT_DEV_OUTPUT_DIR = path.join(LIGHT_OUTPUT_DIR, 'dev');
const DARK_DEV_OUTPUT_DIR = path.join(DARK_OUTPUT_DIR, 'dev');
const CACHE_DIR = path.join(DOCS_ROOT, '.cache', 'drawio');
const EXPORT_STAGE_ROOT = path.join(CACHE_DIR, 'export-staging');

const DRAWIO_DOCKER_BIN = process.env.DRAWIO_DOCKER_BIN || 'docker';
const DRAWIO_DOCKER_IMAGE =
  process.env.DRAWIO_DOCKER_IMAGE || 'rlespinasse/drawio-export:latest';
const DRAWIO_COMMAND_TIMEOUT = process.env.DRAWIO_COMMAND_TIMEOUT || '180';
const DRAWIO_RUN_AS_USER = process.env.DRAWIO_DOCKER_RUN_AS_USER === 'true';
const WATCH_DEBOUNCE_MS = Number(process.env.DRAWIO_WATCH_DEBOUNCE_MS || '300');
const WATCH_STABLE_MS = Number(process.env.DRAWIO_WATCH_STABLE_MS || '600');
const WATCH_STABLE_POLL_MS = Number(process.env.DRAWIO_WATCH_STABLE_POLL_MS || '150');
const WATCH_IGNORE_MS = Number(process.env.DRAWIO_WATCH_IGNORE_MS || '1500');

const FENCE_RE = /```([^\n`]*)\n([\s\S]*?)```/g;

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

function parseFenceInfo(infoText) {
  const info = infoText.trim();
  if (!info) return { lang: '', meta: '' };
  const firstSpace = info.search(/\s/);
  if (firstSpace < 0) {
    return { lang: info.toLowerCase(), meta: '' };
  }
  return {
    lang: info.slice(0, firstSpace).toLowerCase(),
    meta: info.slice(firstSpace + 1).trim(),
  };
}

function extractDrawioRefsFromMdx(mdxFilePath, content) {
  const refs = [];
  let match;

  while ((match = FENCE_RE.exec(content)) !== null) {
    const info = parseFenceInfo(match[1] ?? '');
    if (info.lang !== 'drawio') continue;

    let parsed;
    try {
      parsed = parseDrawioFenceMeta(info.meta);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      throw new Error(`${mdxFilePath}: ${message}`);
    }

    const sourcePath = path.resolve(path.dirname(mdxFilePath), parsed.src);
    if (!sourcePath.endsWith('.drawio')) {
      throw new Error(
        `${mdxFilePath}: drawio src must reference a .drawio file (got "${parsed.src}")`,
      );
    }
    if (!existsSync(sourcePath)) {
      throw new Error(
        `${mdxFilePath}: Draw.io source "${parsed.src}" not found (${sourcePath})`,
      );
    }

    refs.push({
      mdxFilePath,
      src: parsed.src,
      sourcePath,
      alt: parsed.alt,
    });
  }

  return refs;
}

function touchFile(filePath) {
  const now = new Date();
  utimesSync(filePath, now, now);
}

function toPosixPath(p) {
  return p.split(path.sep).join('/');
}

function walkFilesRecursive(dir) {
  const out = [];
  if (!existsSync(dir)) return out;
  for (const name of readdirSync(dir)) {
    const full = path.join(dir, name);
    const st = statSync(full);
    if (st.isDirectory()) out.push(...walkFilesRecursive(full));
    else if (st.isFile()) out.push(full);
  }
  return out;
}

function runDrawioExport({ mdxFilePath, sourceRef, sourcePath, hash, variant }) {
  const outDir = variant === 'dark' ? DARK_OUTPUT_DIR : LIGHT_OUTPUT_DIR;
  const outPath = path.join(outDir, `${hash}.svg`);
  if (existsSync(outPath)) return { outPath, reused: true };

  const stageDir = path.join(EXPORT_STAGE_ROOT, hash, variant);
  rmSync(stageDir, { force: true, recursive: true });
  mkdirSync(stageDir, { recursive: true });

  const relSource = path.relative(DOCS_ROOT, sourcePath);
  if (relSource.startsWith('..')) {
    throw new Error(`Draw.io source must live under docs/ (got ${sourcePath})`);
  }

  const containerSource = `/data/${toPosixPath(relSource)}`;
  const containerOutput = `/data/${toPosixPath(path.relative(DOCS_ROOT, stageDir))}`;

  const args = [
    'run',
    '--rm',
    '-v',
    `${DOCS_ROOT}:/data`,
    '-e',
    `DRAWIO_DESKTOP_COMMAND_TIMEOUT=${DRAWIO_COMMAND_TIMEOUT}`,
  ];
  if (
    DRAWIO_RUN_AS_USER &&
    typeof process.getuid === 'function' &&
    typeof process.getgid === 'function'
  ) {
    args.push('-u', `${process.getuid()}:${process.getgid()}`);
  }
  args.push(
    DRAWIO_DOCKER_IMAGE,
    '--format',
    'svg',
    '--output',
    containerOutput,
    '--svg-theme',
    variant,
    '--remove-page-suffix',
    '--embed-svg-fonts',
    'true',
    containerSource,
  );

  const res = spawnSync(DRAWIO_DOCKER_BIN, args, {
    cwd: DOCS_ROOT,
    encoding: 'utf8',
    env: process.env,
  });

  if (res.status !== 0) {
    const stderr = (res.stderr || '').trim();
    const stdout = (res.stdout || '').trim();
    const details = [stderr, stdout].filter(Boolean).join('\n');
    throw new Error(
      [
        `Draw.io export failed (${variant})`,
        `  mdx file: ${mdxFilePath}`,
        `  drawio src: ${sourceRef}`,
        `  resolved source: ${sourcePath}`,
        `  docker image: ${DRAWIO_DOCKER_IMAGE}`,
        `  command: ${DRAWIO_DOCKER_BIN} ${args.join(' ')}`,
        details ? `  output:\n${details}` : '',
      ]
        .filter(Boolean)
        .join('\n'),
    );
  }

  const exportedSvgs = walkFilesRecursive(stageDir).filter((f) => f.endsWith('.svg'));
  if (exportedSvgs.length !== 1) {
    const details =
      exportedSvgs.length > 0
        ? `Exported SVGs:\n${exportedSvgs.map((f) => `- ${f}`).join('\n')}`
        : 'No SVG files were found in the export output.';
    throw new Error(
      `Expected exactly one SVG export for "${sourcePath}" (${variant}). ` +
        `v1 supports single-page .drawio files only.\n${details}`,
    );
  }

  copyFileSync(exportedSvgs[0], outPath);
  return { outPath, reused: false };
}

function syncDevAliases(refs) {
  mkdirSync(LIGHT_DEV_OUTPUT_DIR, { recursive: true });
  mkdirSync(DARK_DEV_OUTPUT_DIR, { recursive: true });

  const uniqueBySourcePath = new Map();
  for (const ref of refs) {
    if (!uniqueBySourcePath.has(ref.sourcePath)) uniqueBySourcePath.set(ref.sourcePath, ref);
  }

  for (const ref of uniqueBySourcePath.values()) {
    const source = readFileSync(ref.sourcePath);
    const hash = drawioDiagramHash(source);
    const key = drawioDevDiagramKey(ref.sourcePath);
    const lightHashed = path.join(LIGHT_OUTPUT_DIR, `${hash}.svg`);
    const darkHashed = path.join(DARK_OUTPUT_DIR, `${hash}.svg`);
    const lightDev = path.join(LIGHT_DEV_OUTPUT_DIR, `${key}.svg`);
    const darkDev = path.join(DARK_DEV_OUTPUT_DIR, `${key}.svg`);

    if (existsSync(lightHashed)) copyFileSync(lightHashed, lightDev);
    if (existsSync(darkHashed)) copyFileSync(darkHashed, darkDev);
  }
}

function generateDrawioDiagrams({ pruneStale = true, syncDev = false } = {}) {
  mkdirSync(LIGHT_OUTPUT_DIR, { recursive: true });
  mkdirSync(DARK_OUTPUT_DIR, { recursive: true });
  mkdirSync(EXPORT_STAGE_ROOT, { recursive: true });

  const mdxFiles = existsSync(CONTENT_ROOT) ? walkFiles(CONTENT_ROOT) : [];
  const refs = [];

  for (const mdxFile of mdxFiles) {
    const content = readFileSync(mdxFile, 'utf8');
    refs.push(...extractDrawioRefsFromMdx(mdxFile, content));
  }

  const jobs = new Map();
  for (const ref of refs) {
    const source = readFileSync(ref.sourcePath);
    const hash = drawioDiagramHash(source);
    if (!jobs.has(hash)) {
      jobs.set(hash, {
        hash,
        mdxFilePath: ref.mdxFilePath,
        sourceRef: ref.src,
        sourcePath: ref.sourcePath,
      });
    }
  }

  let rendered = 0;
  let reused = 0;

  for (const job of jobs.values()) {
    const light = runDrawioExport({
      mdxFilePath: job.mdxFilePath,
      sourceRef: job.sourceRef,
      sourcePath: job.sourcePath,
      hash: job.hash,
      variant: 'light',
    });
    const dark = runDrawioExport({
      mdxFilePath: job.mdxFilePath,
      sourceRef: job.sourceRef,
      sourcePath: job.sourcePath,
      hash: job.hash,
      variant: 'dark',
    });
    if (light.reused && dark.reused) reused += 1;
    else rendered += 1;
  }

  let removed = 0;
  if (pruneStale) {
    const keep = new Set([...jobs.keys()].map((hash) => `${hash}.svg`));
    for (const dir of [LIGHT_OUTPUT_DIR, DARK_OUTPUT_DIR]) {
      if (!existsSync(dir)) continue;
      for (const name of readdirSync(dir)) {
        if (!name.endsWith('.svg')) continue;
        if (!keep.has(name)) {
          rmSync(path.join(dir, name), { force: true });
          removed += 1;
        }
      }
    }
  }

  console.log(
    `Draw.io: found ${refs.length} reference(s), ${jobs.size} unique diagram(s), rendered ${rendered}, reused ${reused}, removed ${removed}.`,
  );

  if (syncDev) {
    syncDevAliases(refs);
  }

  return {
    refs,
    jobs,
  };
}

function logFatal(error) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(message);
  console.error(
    `Draw.io export requires Docker plus access to the Docker daemon, using image "${DRAWIO_DOCKER_IMAGE}".`,
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

async function waitForFilesStable(filePaths) {
  const drawioFiles = [...new Set(filePaths.filter((p) => typeof p === 'string' && p.endsWith('.drawio')))];
  if (drawioFiles.length === 0) return;

  let previous = new Map();
  let stableSince = 0;
  const deadline = Date.now() + Math.max(WATCH_STABLE_MS * 10, 2000);

  while (Date.now() <= deadline) {
    const current = new Map(drawioFiles.map((p) => [p, statFingerprint(p)]));
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
  console.log(`Draw.io watch: watching ${CONTENT_ROOT} for .drawio/.mdx changes...`);

  let isRunning = false;
  let rerunRequested = false;
  let debounceTimer = null;
  const pendingChangedPaths = new Set();
  const ignoredPathsUntil = new Map();

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
      // In watch mode, keep old hashed assets around to reduce hot-reload races.
      const result = generateDrawioDiagrams({ pruneStale: false, syncDev: true });

      const changedDrawioPaths = new Set(changedPaths.filter((p) => p.endsWith('.drawio')));
      if (changedDrawioPaths.size > 0) {
        const mdxDependents = new Set();
        for (const ref of result.refs) {
          if (changedDrawioPaths.has(ref.sourcePath)) {
            mdxDependents.add(ref.mdxFilePath);
          }
        }

        for (const mdxPath of mdxDependents) {
          if (!existsSync(mdxPath)) continue;
          try {
            touchFile(mdxPath);
            ignoredPathsUntil.set(mdxPath, Date.now() + WATCH_IGNORE_MS);
            console.log(`Draw.io watch: touched dependent MDX ${path.relative(CONTENT_ROOT, mdxPath)}`);
          } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            console.error(`Draw.io watch: failed to touch ${mdxPath}: ${message}`);
          }
        }
      }
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
      console.log(`Draw.io watch: change detected (${reason}), regenerating...`);
      void runGeneration();
    }, WATCH_DEBOUNCE_MS);
  };

  void runGeneration();

  const watcher = fsWatch(
    CONTENT_ROOT,
    { recursive: true },
    (_eventType, filename) => {
      const name = typeof filename === 'string' ? filename : '';
      if (!name) {
        queueRun('content change');
        return;
      }
      if (!name.endsWith('.drawio') && !name.endsWith('.mdx')) return;
      const absolutePath = path.join(CONTENT_ROOT, name);
      const ignoreUntil = ignoredPathsUntil.get(absolutePath) ?? 0;
      if (ignoreUntil > Date.now()) {
        return;
      }
      if (ignoreUntil > 0) {
        ignoredPathsUntil.delete(absolutePath);
      }
      pendingChangedPaths.add(absolutePath);
      queueRun(name);
    },
  );

  watcher.on('error', (error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`Draw.io watch error: ${message}`);
  });

  const shutdown = () => {
    if (debounceTimer) clearTimeout(debounceTimer);
    watcher.close();
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
    generateDrawioDiagrams({ pruneStale: true, syncDev: false });
  } catch (error) {
    logFatal(error);
    process.exit(1);
  }
}

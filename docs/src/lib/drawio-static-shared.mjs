import { createHash } from 'node:crypto';

export const DRAWIO_STATIC_HASH_VERSION = 'v1';
export const DRAWIO_LIGHT_URL_PREFIX = '/kubos/diagrams/drawio/light/';
export const DRAWIO_DARK_URL_PREFIX = '/kubos/diagrams/drawio/dark/';
export const DRAWIO_DEV_ALIAS_VERSION = 'v1';

/**
 * @param {string | null | undefined} meta
 * @returns {Record<string, string | true>}
 */
export function parseFenceMeta(meta) {
  const out = {};
  const input = (meta ?? '').trim();
  if (!input) return out;

  let idx = 0;
  const isWhitespace = (ch) => /\s/.test(ch);

  while (idx < input.length) {
    while (idx < input.length && isWhitespace(input[idx])) idx += 1;
    if (idx >= input.length) break;

    const keyStart = idx;
    while (idx < input.length && /[A-Za-z0-9_-]/.test(input[idx])) idx += 1;
    if (idx === keyStart) {
      throw new Error(`Invalid fence metadata near "${input.slice(idx, idx + 20)}"`);
    }

    const key = input.slice(keyStart, idx);

    while (idx < input.length && isWhitespace(input[idx])) idx += 1;
    if (input[idx] !== '=') {
      out[key] = true;
      continue;
    }

    idx += 1;
    while (idx < input.length && isWhitespace(input[idx])) idx += 1;
    if (idx >= input.length) {
      throw new Error(`Missing value for "${key}" in fence metadata`);
    }

    const quote = input[idx];
    if (quote === '"' || quote === "'") {
      idx += 1;
      let value = '';
      let closed = false;
      while (idx < input.length) {
        const ch = input[idx];
        if (ch === '\\' && idx + 1 < input.length) {
          value += input[idx + 1];
          idx += 2;
          continue;
        }
        if (ch === quote) {
          idx += 1;
          closed = true;
          break;
        }
        value += ch;
        idx += 1;
      }
      if (!closed) {
        throw new Error(`Unterminated quoted value for "${key}" in fence metadata`);
      }
      out[key] = value;
      continue;
    }

    const valueStart = idx;
    while (idx < input.length && !isWhitespace(input[idx])) idx += 1;
    out[key] = input.slice(valueStart, idx);
  }

  return out;
}

/**
 * @param {string | null | undefined} meta
 */
export function parseDrawioFenceMeta(meta) {
  const attrs = parseFenceMeta(meta);
  const src = attrs.src;
  if (typeof src !== 'string' || !src.trim()) {
    throw new Error('`drawio` fence requires src="...drawio"');
  }

  const page = attrs.page;
  if (typeof page === 'string' && page.trim()) {
    throw new Error('`drawio` fence `page=` is not supported yet (v1 supports single-page .drawio files only)');
  }

  return {
    src: src.trim(),
    alt: typeof attrs.alt === 'string' ? attrs.alt : 'Draw.io diagram',
    className: typeof attrs.className === 'string' ? attrs.className : undefined,
    width: typeof attrs.width === 'string' ? attrs.width : undefined,
    height: typeof attrs.height === 'string' ? attrs.height : undefined,
  };
}

/**
 * @param {Buffer | string} source
 */
export function drawioDiagramHash(source) {
  const hash = createHash('sha256');
  hash.update(`codex-drawio-static:${DRAWIO_STATIC_HASH_VERSION}\nformat=svg\n`);
  hash.update(source);
  return hash.digest('hex').slice(0, 16);
}

/**
 * @param {string} hash
 */
export function drawioLightDiagramUrl(hash) {
  return `${DRAWIO_LIGHT_URL_PREFIX}${hash}.svg`;
}

/**
 * @param {string} hash
 */
export function drawioDarkDiagramUrl(hash) {
  return `${DRAWIO_DARK_URL_PREFIX}${hash}.svg`;
}

/**
 * @param {string} sourcePath
 */
export function drawioDevDiagramKey(sourcePath) {
  return createHash('sha256')
    .update(`codex-drawio-dev-alias:${DRAWIO_DEV_ALIAS_VERSION}\n`)
    .update(String(sourcePath).replaceAll('\\', '/'))
    .digest('hex')
    .slice(0, 16);
}

/**
 * @param {string} sourcePath
 */
export function drawioLightDevDiagramUrl(sourcePath) {
  return `${DRAWIO_LIGHT_URL_PREFIX}dev/${drawioDevDiagramKey(sourcePath)}.svg`;
}

/**
 * @param {string} sourcePath
 */
export function drawioDarkDevDiagramUrl(sourcePath) {
  return `${DRAWIO_DARK_URL_PREFIX}dev/${drawioDevDiagramKey(sourcePath)}.svg`;
}

/**
 * @param {string} src
 */
export function isDrawioLightDiagramUrl(src) {
  return src.startsWith(DRAWIO_LIGHT_URL_PREFIX) && src.endsWith('.svg');
}

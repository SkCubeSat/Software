import { readFileSync } from 'node:fs';
import path from 'node:path';
import {
  drawioDiagramHash,
  drawioLightDevDiagramUrl,
  drawioLightDiagramUrl,
  parseDrawioFenceMeta,
} from './drawio-static-shared.mjs';

type MdastNode = {
  type: string;
  children?: MdastNode[];
  lang?: string | null;
  meta?: string | null;
  value?: string;
  url?: string;
  alt?: string;
  title?: string | null;
};

type VFileLike = {
  path?: string;
};

function isDrawioBlock(node: MdastNode): node is MdastNode & { meta?: string | null } {
  return node.type === 'code' && node.lang?.toLowerCase() === 'drawio';
}

function visit(node: MdastNode, file: VFileLike) {
  if (!node.children) return;
  if (!file.path) {
    throw new Error('Draw.io MDX processing requires the source file path');
  }

  for (let i = 0; i < node.children.length; i += 1) {
    const child = node.children[i];

    if (isDrawioBlock(child)) {
      let parsed;
      try {
        parsed = parseDrawioFenceMeta(child.meta);
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        throw new Error(`${file.path}: ${message}`);
      }

      const resolvedPath = path.resolve(path.dirname(file.path), parsed.src);
      let source: Buffer;
      try {
        source = readFileSync(resolvedPath);
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        throw new Error(
          `${file.path}: failed to read Draw.io source "${parsed.src}" (${resolvedPath}): ${message}`,
        );
      }

      const hash = drawioDiagramHash(source);
      node.children[i] = {
        type: 'image',
        url: drawioLightDiagramUrl(hash),
        alt: parsed.alt,
        title: null,
      };
      continue;
    }

    visit(child, file);
  }
}

export function remarkDrawioStaticImages() {
  return (tree: MdastNode, file: VFileLike) => {
    const isDev = process.env.NODE_ENV !== 'production';
    if (!file.path) {
      throw new Error('Draw.io MDX processing requires the source file path');
    }
    const filePath = file.path;
    // pass mode through closure to keep visit() simple and deterministic
    const visitWithMode = (node: MdastNode) => {
      if (!node.children) return;

      for (let i = 0; i < node.children.length; i += 1) {
        const child = node.children[i];

        if (isDrawioBlock(child)) {
          let parsed;
          try {
            parsed = parseDrawioFenceMeta(child.meta);
          } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            throw new Error(`${filePath}: ${message}`);
          }

          const resolvedPath = path.resolve(path.dirname(filePath), parsed.src);
          let source: Buffer;
          try {
            source = readFileSync(resolvedPath);
          } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            throw new Error(
              `${filePath}: failed to read Draw.io source "${parsed.src}" (${resolvedPath}): ${message}`,
            );
          }

          const url = isDev
            ? drawioLightDevDiagramUrl(resolvedPath)
            : drawioLightDiagramUrl(drawioDiagramHash(source));

          node.children[i] = {
            type: 'image',
            url,
            alt: parsed.alt,
            title: null,
          };
          continue;
        }

        visitWithMode(child);
      }
    };
    visitWithMode(tree);
  };
}

# RADSAT Software Docs (Fumadocs)

This `docs/` folder contains the documentation site built with Fumadocs + Next.js (static export), replacing the previous Sphinx setup.

## Local Development (Bun)

```bash
cd docs
bun install
bun run dev
```

Open `http://localhost:3000/docs`.

## Static Build (GitHub Pages)

```bash
cd docs
bun install
bun run build
```

The generated static site is written to `docs/out/`.

Note: the build also renders PlantUML diagrams (used by migrated KubOS pages) into static SVG assets.
For local builds you need Java and Graphviz (`dot`) installed. The GitHub Pages workflow installs them automatically.

## Content

Documentation pages are stored in `docs/content/docs/`.

Generated PlantUML SVGs are written to `docs/public/kubos/diagrams/` during the docs build.

The service workflow docs are also kept at the repo root as:

- `TESTING.MD`
- `CROSSCOMPILING.MD`

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
For local builds you need Java and Graphviz (`dot`) installed for PlantUML and Docker installed for Draw.io exports.
The GitHub Pages workflow installs PlantUML prerequisites and uses Docker for Draw.io.

## Content

Documentation pages are stored in `docs/content/docs/`.

Generated PlantUML SVGs are written to `docs/public/kubos/diagrams/` during the docs build.
Generated Draw.io SVGs are written to `docs/public/kubos/diagrams/drawio/` during the docs build.

## Draw.io Diagrams in MDX

Reference a `.drawio` file from MDX using a fenced `drawio` block:

````mdx
```drawio src="./diagrams/system-architecture.drawio" alt="System architecture"
```
````

Notes:

- `src` is required and resolves relative to the current `.mdx` file.
- `alt` is recommended for accessibility.
- `page=` is not supported yet (v1 supports single-page `.drawio` files only).
- The build generates static light/dark SVG variants and swaps them automatically in the site theme.

The service workflow docs are also kept at the repo root as:

- `TESTING.MD`
- `CROSSCOMPILING.MD`

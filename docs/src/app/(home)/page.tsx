import Link from 'next/link';

export default function HomePage() {
  return (
    <main className="mx-auto flex w-full max-w-5xl flex-1 flex-col justify-center gap-8 px-6 py-20">
      <p className="text-sm uppercase tracking-[0.18em] text-fd-muted-foreground">RADSAT Software</p>
      <h1 className="max-w-4xl text-4xl font-semibold tracking-tight sm:text-5xl">
        Welcome to RADSAT-SK2 Software Documentation
      </h1>
      <p className="max-w-3xl text-base leading-7 text-fd-muted-foreground">
        RADSAT-SK2 (Radiation Satellite Saskatchewan 2) is the second cube satellite designed at
        the University of Saskatchewan.
      </p>
      <div className="flex flex-wrap gap-3">
        <Link
          href="/docs"
          className="inline-flex items-center rounded-md border border-fd-border bg-fd-card px-4 py-2 text-sm font-medium transition hover:bg-fd-accent"
        >
          Open Documentation
        </Link>
        <a
          href="https://skcubesat.ca/"
          target="_blank"
          rel="noreferrer noopener"
          className="inline-flex items-center rounded-md border border-fd-border px-4 py-2 text-sm font-medium text-fd-muted-foreground transition hover:bg-fd-accent hover:text-fd-foreground"
        >
          Project Website
        </a>
        <a
          href="https://github.com/SkCubeSat"
          target="_blank"
          rel="noreferrer noopener"
          className="inline-flex items-center rounded-md border border-fd-border px-4 py-2 text-sm font-medium text-fd-muted-foreground transition hover:bg-fd-accent hover:text-fd-foreground"
        >
          SkCubeSat GitHub
        </a>
      </div>
      <div className="grid gap-4 pt-2 sm:grid-cols-2">
        <div className="rounded-xl border border-fd-border bg-fd-card p-4">
          <h2 className="text-sm font-semibold">Get Started</h2>
          <p className="mt-2 text-sm text-fd-muted-foreground">
            Clone the repository, set up GitHub SSH access, and review contribution notes.
          </p>
          <Link href="/docs/getting-started" className="mt-3 inline-block text-sm underline">
            Open Getting Started
          </Link>
        </div>
        <div className="rounded-xl border border-fd-border bg-fd-card p-4">
          <h2 className="text-sm font-semibold">Service Workflows</h2>
          <p className="mt-2 text-sm text-fd-muted-foreground">
            Testing procedures, cross-compiling for KubOS, and docs deployment notes.
          </p>
          <Link href="/docs" className="mt-3 inline-block text-sm underline">
            Browse Docs
          </Link>
        </div>
      </div>
      <p className="text-sm text-fd-muted-foreground">This project is under active development.</p>
    </main>
  );
}

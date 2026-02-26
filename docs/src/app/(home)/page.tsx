import Link from 'next/link';

export default function HomePage() {
  return (
    <main className="mx-auto flex w-full max-w-4xl flex-1 items-center px-5 py-12 sm:px-6 sm:py-20">
      <section className="vp-home-hero w-full rounded-2xl p-8 sm:p-12">
        <div className="relative z-10 space-y-6">
          <p className="text-xs font-semibold uppercase tracking-[0.22em] text-fd-muted-foreground">
            RADSAT Software
          </p>

          <h1 className="text-5xl font-semibold tracking-tight sm:text-7xl">
            <span className="vp-title-gradient">Just Lock IN</span>
          </h1>

          <p className="max-w-2xl text-base leading-7 text-fd-muted-foreground sm:text-lg">
            RADSAT-SK2 software documentation and operational guides.
          </p>

          <div className="flex flex-wrap gap-3">
            <Link
              href="/docs"
              className="inline-flex items-center rounded-md bg-fd-primary px-4 py-2.5 text-sm font-semibold text-fd-primary-foreground transition hover:opacity-90"
            >
              Open Docs
            </Link>
            <Link
              href="/docs/setup/getting-started"
              className="inline-flex items-center rounded-md border border-fd-border bg-fd-card px-4 py-2.5 text-sm font-medium transition hover:bg-fd-accent"
            >
              Getting Started
            </Link>
          </div>
        </div>
      </section>
    </main>
  );
}

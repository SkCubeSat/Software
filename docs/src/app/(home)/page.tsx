import Link from 'next/link';

export default function HomePage() {
  return (
    <main className="mx-auto flex w-full max-w-4xl flex-1 items-center px-5 py-12 sm:px-6 sm:py-20">
      <section className="vp-home-hero w-full rounded-3xl p-6 sm:p-8 lg:p-10">
        <div className="relative z-10 space-y-6">
          <p className="text-xs font-semibold uppercase tracking-[0.22em] text-fd-muted-foreground">
            RADSAT Software
          </p>

          <div className="vp-lockline">
            <span className="vp-lockbox" aria-hidden="true">
              <svg viewBox="0 0 20 20" fill="none">
                <path d="M4.5 10.5L8.3 14.2L15.5 6.8" />
              </svg>
            </span>

            <div className="min-w-0">
              <p className="vp-lockline-label">Todo</p>
              <h1 className="vp-lockline-title">LOCK IN</h1>
            </div>

            <span className="vp-lockline-badge" aria-hidden="true">
              Active
            </span>
          </div>

          <p className="max-w-2xl text-base leading-7 text-fd-muted-foreground sm:text-lg">
            RADSAT-SK2 software documentation and operational guides with a cleaner,
            more focused landing experience.
          </p>

          <div className="flex flex-wrap gap-3">
            <Link href="/docs" className="vp-home-cta vp-home-cta-primary">
              Open Docs
            </Link>
            <Link href="/docs/setup/getting-started" className="vp-home-cta vp-home-cta-secondary">
              Getting Started
            </Link>
          </div>
        </div>
      </section>
    </main>
  );
}

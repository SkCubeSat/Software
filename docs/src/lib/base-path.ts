const repoName = process.env.GITHUB_REPOSITORY?.split('/')[1] ?? '';
const ghPagesBasePath =
  process.env.GITHUB_ACTIONS === 'true' && repoName ? `/${repoName}` : '';
const RAW_BASE_PATH = process.env.NEXT_PUBLIC_BASE_PATH ?? ghPagesBasePath;

export const DOCS_BASE_PATH =
  RAW_BASE_PATH && RAW_BASE_PATH !== '/' ? RAW_BASE_PATH.replace(/\/+$/, '') : '';

export function withBasePath(url: string): string {
  if (!url.startsWith('/') || url.startsWith('//')) {
    return url;
  }

  if (!DOCS_BASE_PATH) {
    return url;
  }

  if (url === DOCS_BASE_PATH || url.startsWith(`${DOCS_BASE_PATH}/`)) {
    return url;
  }

  return `${DOCS_BASE_PATH}${url}`;
}

export function normalizeLegacyKubosUrl(url: string): string {
  if (!url.startsWith('/') || url.startsWith('//')) {
    return url;
  }

  // Legacy migrated docs sometimes refer to `/kubos/...` docs routes, but this app mounts docs pages under `/docs`.
  // Keep static assets under `/kubos/images` and `/kubos/diagrams` at the site root.
  if (
    url.startsWith('/kubos/') &&
    !url.startsWith('/kubos/images/') &&
    !url.startsWith('/kubos/diagrams/')
  ) {
    return withBasePath(`/docs${url}`);
  }

  return withBasePath(url);
}

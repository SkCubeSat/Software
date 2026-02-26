const RAW_BASE_PATH = process.env.NEXT_PUBLIC_BASE_PATH ?? '';

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

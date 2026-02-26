import { createMDX } from 'fumadocs-mdx/next';

const withMDX = createMDX();
const isGitHubPagesBuild = process.env.GITHUB_ACTIONS === 'true';
const repoName = process.env.GITHUB_REPOSITORY?.split('/')[1] ?? '';
const basePath = isGitHubPagesBuild && repoName ? `/${repoName}` : undefined;

/** @type {import('next').NextConfig} */
const config = {
  output: 'export',
  reactStrictMode: true,
  trailingSlash: true,
  env: {
    NEXT_PUBLIC_BASE_PATH: basePath ?? '',
  },
  images: {
    unoptimized: true,
  },
  basePath,
  assetPrefix: basePath,
};

export default withMDX(config);

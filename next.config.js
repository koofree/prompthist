/** @type {import('next').NextConfig} */
const nextConfig = {
  // Configure for Tauri static export
  output: 'export',

  // Disable server-side features for static export
  trailingSlash: true,

  // Configure asset prefix for Tauri
  assetPrefix: '.',

  // Optimize images for static export
  images: {
    unoptimized: true,
  },

  // Configure build output
  distDir: 'out',

  // Disable x-powered-by header
  poweredByHeader: false,

  // Configure for desktop app (no server-side rendering)
  reactStrictMode: true,

  // ESLint configuration - use our flat config
  eslint: {
    // Disable ESLint during builds since we run it separately
    ignoreDuringBuilds: true,
  },

  // Configure server external packages
  serverExternalPackages: ['@tauri-apps/api'],

  // Configure webpack for Tauri compatibility
  webpack: (config, { isServer }) => {
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        path: false,
        crypto: false,
      };
    }
    return config;
  },
};

export default nextConfig;

import type { NextConfig } from "next";

const isProd = process.env.NODE_ENV === "production";

const nextConfig: NextConfig = {
  output: "export",
  images: { unoptimized: true },
  // For GitHub Pages deployment under /gic/
  basePath: isProd ? "/gic" : "",
  assetPrefix: isProd ? "/gic/" : "",
};

export default nextConfig;

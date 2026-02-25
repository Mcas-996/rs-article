## Why

当前 `web` 分支只用于在 Cloudflare Pages 上部署运行。这是独立的部署分支，不需要合并回 main。

## What Changes

- 添加 Rust 到 WASM 的编译配置
- 创建 HTML/JS 入口加载 WASM 模块
- 添加 Cloudflare Pages 配置 (wrangler.toml)
- 添加 _redirects 实现 SPA 路由
- 添加构建流程

## Capabilities

### New Capabilities
- `cloudflare-pages-deploy`: 配置并启用 Cloudflare Pages 部署

### Modified Capabilities
- (无 - 这是一个独立的部署分支，不影响主项目)

## Impact

- wasm32-unknown-unknown 目标构建配置
- Web 资源文件 (HTML, JS, _redirects, wrangler.toml)
- GitHub Actions 部署工作流

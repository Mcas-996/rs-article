## Context

`web` 分支是一个独立的部署分支，目标是在 Cloudflare Pages 上运行。rs-grammar 原本是本地 TUI 应用，使用 harper-core、ratatui、arboard、crossterm。

## Goals / Non-Goals

**Goals:**
- 让 rs-grammar 能在 Cloudflare Pages 上运行
- 通过 GitHub Actions 实现自动部署

**Non-Goals:**
- 不修改核心语法检查逻辑
- 不支持其他云平台 (AWS, Vercel 等)
- 不需要完整的 Web UI（TUI 跑不起来就显示简单提示）

## Decisions

1. **WASM 编译目标**: 使用 `wasm32-unknown-unknown`
2. **不引入框架**: 直接用 wasm-bindgen，不使用 yew/leptos
3. **部署工具**: 使用 Wrangler (Cloudflare 官方 CLI)
4. **SPA 路由**: 使用 `_redirects` 文件

## Risks / Trade-offs

- **WASM 体积** → 开启 opt-level = "z", lto = true 优化
- **终端 API 不可用** → 用 feature flag 条件编译，web 入口单独处理
- **构建超时** → 确保在 20 分钟内完成

## 实现步骤

1. 添加 wasm-bindgen、web-sys 依赖
2. 创建 src/web.rs 作为 WASM 入口
3. 创建 index.html 加载 WASM
4. 创建 _redirects、wrangler.toml
5. 创建 GitHub Actions 工作流

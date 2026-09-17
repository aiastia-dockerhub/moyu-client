# moyu-client — 墨语桌面客户端

Tauri v2 薄壳客户端：窗口加载服务器上部署的墨语站点，前端/后端全在服务器，本地不装任何依赖。Rust 编译全部在 GitHub Actions 云端完成。

**站点地址不出现在仓库里**：编译时由 CI 从仓库 secret `MOYU_URL` 注入二进制。设置位置：仓库 → Settings → Secrets and variables → Actions。

## 平台矩阵

| 产物 | 平台 |
|---|---|
| `moyu-macos-arm64` | macOS Apple Silicon (DMG) |
| `moyu-macos-intel` | macOS Intel (DMG) |
| `moyu-windows-x64` | Windows (NSIS 安装器 + MSI) |
| `moyu-linux-x64` | Linux (AppImage + deb) |

## 重新出包

推送 main 或手动 `gh workflow run` 触发；打 tag `v*` 额外自动建 Release。

```bash
git push && gh run watch -R aiastia-dockerhub/moyu-client   # 看进度
gh run download -R aiastia-dockerhub/moyu-client --dir ~/Desktop/moyu-out
```

## 未签名应用

- macOS：gh 下载的 DMG 无隔离属性可直接开；浏览器下载的右键 → 打开，或 `xattr -cr /Applications/Moyu.app`
- Windows：SmartScreen 点「更多信息 → 仍要运行」

## 结构

- `src-tauri/src/lib.rs` — 从 `MOYU_URL` 编译期常量创建主窗口，改窗口尺寸在这里
- `src-tauri/icons/app-icon.png` — 图标源文件（1024），CI 用 `tauri icon` 生成全平台图标
- `src-tauri/gen_icon.swift` — 图标源文件生成脚本（macOS 自带 swift）
- `.github/workflows/build.yml` — 四平台云编译

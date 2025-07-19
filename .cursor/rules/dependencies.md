# 依存関係とAPI使用例

## Frontend依存関係

### 主要パッケージ
- `react` ^18.3.1 - UIライブラリ
- `react-dom` ^18.3.1 - DOM操作
- `@tauri-apps/api` ^2 - Tauri API
- `@tauri-apps/plugin-opener` ^2 - システム統合

### 開発依存関係
- `typescript` ~5.6.2 - 型チェック
- `vite` ^6.0.3 - ビルドツール
- `@vitejs/plugin-react` ^4.3.4 - React対応

## Tauri API使用例

### Core API
```typescript
import { invoke } from "@tauri-apps/api/core";

// コマンド呼び出し
const result = await invoke("greet", { name: "World" });
```

### Plugin - Opener
```typescript
import { open } from "@tauri-apps/plugin-opener";

// 外部URLを開く
await open("https://example.com");

// ファイルを開く
await open("/path/to/file.txt");
```

## Rust依存関係

### 主要クレート
- `tauri` 2 - フレームワーク本体
- `tauri-plugin-opener` 2 - システム統合
- `serde` 1 - JSON シリアライゼーション
- `serde_json` 1 - JSON操作

### 使用例

#### Serde シリアライゼーション
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    version: String,
}

#[tauri::command]
fn get_config() -> Config {
    Config {
        name: "reportify".to_string(),
        version: "0.1.0".to_string(),
    }
}
```

## Viteプラグイン設定

### React プラグイン
```typescript
// vite.config.ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  // Tauri固有の設定
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
});
```

## 新しい依存関係追加時のチェックリスト

### Frontend
1. `yarn add <package>` でインストール
2. TypeScript型定義が含まれているか確認
3. Vite設定での追加設定が必要か確認

### Backend
1. `Cargo.toml` に依存関係追加
2. `features` が必要な場合は指定
3. `cargo build` でコンパイル確認

## セキュリティ考慮事項

### 信頼できるパッケージの選択
- NPM Auditの実行: `yarn audit`
- Cargo Auditの実行: `cargo audit`

### バージョン固定
- 重要な依存関係はexact versionで固定
- セキュリティアップデートは定期的に確認

## 更新履歴
- 2025-07-19: 初期テンプレート作成
# トラブルシューティングガイド

## Tauriビルドエラー

### 一般的な問題

#### "tauri command not found"
**原因**: Tauri CLIがインストールされていない
**解決策**:
```bash
yarn add --dev @tauri-apps/cli
# または
npm install --save-dev @tauri-apps/cli
```

#### Rustコンパイルエラー
**原因**: 依存関係の不整合
**解決策**:
```bash
cd src-tauri
cargo clean
cargo build
```

## Rust-JS間通信エラー

### "command not found" エラー
**原因**: コマンドがinvoke_handlerに登録されていない
**確認箇所**: `src-tauri/src/lib.rs`の`invoke_handler`
```rust
.invoke_handler(tauri::generate_handler![your_command])
```

### JSON シリアライゼーションエラー
**原因**: Rustの型がJSONに変換できない
**解決策**: SerdeのDeriveマクロを追加
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    field: String,
}
```

## 開発環境構築時の問題

### ポート1420が既に使用されている
**解決策**:
```bash
# プロセスを確認
lsof -i :1420
# プロセスを終了
kill -9 <PID>
```

### Yarn/NPM依存関係の問題
**解決策**:
```bash
# node_modulesを削除して再インストール
rm -rf node_modules yarn.lock
yarn install
```

## パフォーマンス問題

### アプリ起動が遅い
**確認項目**:
- Rustのrelease buildを使用しているか
- 不要なTauriプラグインが含まれていないか

### メモリ使用量が多い
**確認項目**:
- React DevToolsでのメモリリーク
- Rustサイドでの不適切なメモリ管理

## 更新履歴
- 2025-07-19: 初期テンプレート作成
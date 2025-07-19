# セキュリティガイドライン

## Tauriアプリ特有のセキュリティ考慮事項

### CSP (Content Security Policy) 設定

現在の設定: `tauri.conf.json`
```json
{
  "app": {
    "security": {
      "csp": null
    }
  }
}
```

**推奨設定:**
```json
{
  "app": {
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:;"
    }
  }
}
```

### コマンドレベルのセキュリティ

#### 入力検証
```rust
#[tauri::command]
fn secure_command(input: &str) -> Result<String, String> {
    // 入力検証
    if input.is_empty() || input.len() > 1000 {
        return Err("Invalid input".to_string());
    }
    
    // サニタイゼーション
    let sanitized = input.trim();
    
    // 処理実行
    Ok(format!("Processed: {}", sanitized))
}
```

#### 認証が必要なコマンド
```rust
use tauri::State;

struct AuthState {
    is_authenticated: bool,
}

#[tauri::command]
fn protected_command(
    state: State<AuthState>
) -> Result<String, String> {
    if !state.is_authenticated {
        return Err("Unauthorized".to_string());
    }
    
    // 認証済みユーザーのみ実行
    Ok("Protected data".to_string())
}
```

## 機密情報の扱い方

### 環境変数の使用
```rust
// Rustサイド
fn get_api_key() -> String {
    std::env::var("API_KEY")
        .expect("API_KEY environment variable not set")
}
```

### フロントエンドでの機密情報
```typescript
// ❌ 危険: フロントエンドに直接書かない
const API_KEY = "secret-key";

// ✅ 安全: Tauriコマンド経由で取得
const getApiKey = async () => {
  return await invoke("get_api_key");
};
```

### 設定ファイルの暗号化
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EncryptedConfig {
    encrypted_data: String,
}

#[tauri::command]
fn load_secure_config() -> Result<String, String> {
    // 設定ファイルを読み込み、復号化
    // 実装は要件に応じて
    Ok("decrypted_config".to_string())
}
```

## ファイルアクセス制御

### 許可されたディレクトリのみアクセス
```rust
use std::path::Path;

fn is_safe_path(path: &str) -> bool {
    let path = Path::new(path);
    
    // ディレクトリトラバーサル攻撃を防ぐ
    if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return false;
    }
    
    // 許可されたディレクトリのみ
    path.starts_with("./data") || path.starts_with("./config")
}

#[tauri::command]
fn read_file(path: &str) -> Result<String, String> {
    if !is_safe_path(path) {
        return Err("Access denied".to_string());
    }
    
    // ファイル読み込み処理
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
}
```

## ネットワーク通信

### HTTPS強制
```typescript
// ✅ HTTPS URLのみ使用
const API_BASE = "https://api.example.com";

// ❌ HTTP は避ける
const API_BASE = "http://api.example.com";
```

### 証明書検証
```rust
// HTTPクライアント設定例
use reqwest::Client;

fn create_secure_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .danger_accept_invalid_certs(false) // 証明書検証を有効化
        .build()
}
```

## ユーザー入力のサニタイゼーション

### XSS対策
```typescript
// HTMLエスケープ
const escapeHtml = (text: string): string => {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
};

// 使用例
const userInput = "<script>alert('xss')</script>";
const safeOutput = escapeHtml(userInput);
```

### SQLインジェクション対策（将来のDB使用時）
```rust
// ❌ 危険: 文字列結合
let query = format!("SELECT * FROM users WHERE name = '{}'", user_name);

// ✅ 安全: パラメータ化クエリ
let query = "SELECT * FROM users WHERE name = ?";
// プリペアードステートメントを使用
```

## ログ出力時の注意事項

### 機密情報をログに出力しない
```rust
// ❌ 危険
println!("API Key: {}", api_key);

// ✅ 安全
println!("API Key loaded successfully");

// ✅ 開発時のみ
#[cfg(debug_assertions)]
println!("Debug: API Key: {}", api_key);
```

## 更新とメンテナンス

### 定期的なセキュリティチェック
```bash
# npm audit
yarn audit

# Rust dependencies audit
cargo audit

# 依存関係の更新
yarn upgrade
cargo update
```

### セキュリティヘッダー
```rust
// レスポンスヘッダーの設定（Web APIを提供する場合）
response.headers_mut().insert(
    "X-Content-Type-Options",
    "nosniff".parse().unwrap(),
);
response.headers_mut().insert(
    "X-Frame-Options", 
    "DENY".parse().unwrap(),
);
```

## 更新履歴
- 2025-07-19: 初期テンプレート作成
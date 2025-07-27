# Reportify

レポート作成を効率化するTauriアプリケーション

## プロジェクト概要

Reportifyは、React（TypeScript）フロントエンドとRustバックエンドを組み合わせたTauri v2アプリケーションです。クリーンアーキテクチャパターンを採用し、効率的なレポート作成をサポートします。

### 技術スタック

#### フロントエンド

- **React 18** + **TypeScript** - UIフレームワーク
- **Vite** - 高速ビルドツール
- **Material-UI** - UIコンポーネントライブラリ
- **TanStack Router** - ルーティング
- **Emotion** - CSS-in-JS

#### バックエンド

- **Rust** - 高性能バックエンド
- **Tauri v2** - デスクトップアプリフレームワーク
- **SeaORM** - データベースORM
- **SQLite** - 軽量データベース
- **Tokio** - 非同期ランタイム

### 主な機能

- テーマ管理（ライト/ダークモード切り替え）
- 設定データの永続化
- アクティビティカレンダー表示
- レスポンシブなユーザーインターフェース

## 開発の進め方

### 前提条件

- Node.js (18以上)
- Yarn
- Rust (最新安定版)
- SQLite

### セットアップ

```bash
# リポジトリのクローン
git clone <repository-url>
cd reportify.tauri

# フロントエンド依存関係のインストール
yarn install

# 開発サーバーの起動
yarn tauri dev
```

### 開発コマンド

#### フロントエンド開発

```bash
# 開発サーバー起動（Vite、ポート1420）
yarn dev

# プロダクションビルド（TypeScript + Vite）
yarn build

# プロダクションプレビュー
yarn preview

# TypeScript型チェック
yarn typecheck

# ESLint実行
yarn lint
yarn lint:fix

# Prettier実行
yarn format
yarn format:check

# パッケージ管理
yarn add <package>              # 依存関係追加
yarn add -D <package>           # 開発依存関係追加
yarn remove <package>           # パッケージ削除
yarn upgrade                    # 全パッケージ更新
yarn upgrade <package>          # 特定パッケージ更新
```

#### Tauri開発

```bash
# Tauri開発モード（Rustビルド + アプリ起動）
yarn tauri dev

# プロダクションビルド
yarn tauri build

# Tauri CLIコマンド
yarn tauri
```

#### Rust開発

```bash
cd src-tauri

# コード整形
make format

# Clippyリンター実行
make lint

# フォーマットチェック
make check-format

# コンパイルチェック
make check

# 全品質チェック実行
make ci

# 開発用クイックチェック
make dev

# テスト実行
make test

# ビルド
make build

# クリーンアップ
make clean

# パッケージ管理
cargo add <crate>               # 依存関係追加
cargo add --dev <crate>         # 開発依存関係追加
cargo add --build <crate>       # ビルド依存関係追加
cargo remove <crate>            # 依存関係削除
cargo update                    # Cargo.lock更新
cargo update <crate>            # 特定クレート更新
cargo search <crate>            # クレート検索
```

#### データベース開発

```bash
cd src-tauri

# sea-orm-cliのインストール（初回のみ）
cargo install sea-orm-cli

# 新しいマイグレーションファイルを生成
sea-orm-cli migrate generate <migration_name>

# マイグレーションを実行
sea-orm-cli migrate up

# マイグレーションを1つ戻す
sea-orm-cli migrate down

# マイグレーション状態を確認
sea-orm-cli migrate status

# エンティティを生成（スキーマから）
sea-orm-cli generate entity -o src/infrastructure/database/entities --database-url sqlite://reportify.db
```

### プロジェクト構造

```text
reportify.tauri/
├── src/                   # React フロントエンド
│   ├── components/        # UIコンポーネント
│   ├── contexts/          # React Context
│   ├── features/          # 機能別コンポーネント
│   ├── hooks/             # カスタムフック
│   ├── models/            # TypeScript型定義
│   └── routes/            # ページコンポーネント
│
├── src-tauri/             # Rust バックエンド
│   ├── src/
│   │   ├── application/   # アプリケーション層
│   │   ├── domain/        # ドメイン層
│   │   ├── infrastructure/# インフラ層
│   │   └── presentation/  # プレゼンテーション層
│   ├── icons/             # アプリアイコン
│   └── tauri.conf.json    # Tauri設定
│
├── dist/                  # ビルド出力
└── package.json          # Node.js依存関係
```

### アーキテクチャ

#### クリーンアーキテクチャ

Rustバックエンドはクリーンアーキテクチャパターンを採用：

- **Domain**: ビジネスロジックとエンティティ
- **Application**: ユースケースとアプリケーションサービス
- **Infrastructure**: データベースアクセスと外部サービス
- **Presentation**: Tauriコマンドとインターフェース

#### フロントエンド・バックエンド通信

- フロントエンドから`@tauri-apps/api/core`の`invoke()`でRust関数を呼び出し
- 例: `await invoke("get_theme")` → Rustの`get_theme()`関数を実行
- Rust関数は`lib.rs`の`invoke_handler`に登録が必要

### 開発ガイドライン

1. **コード品質**: 各コミット前に`make ci`（Rust）と`yarn lint`（フロントエンド）を実行
2. **型安全性**: TypeScriptとRustの型を活用し、実行時エラーを防止
3. **テスト**: 新機能には適切なテストを追加
4. **ドキュメント**: APIや複雑な機能には文書化を行う

### ビルドと配布

```bash
# プロダクションビルド
yarn tauri build

# 出力先: src-tauri/target/release/bundle/
```

## 推奨IDE設定

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## ライセンス

MIT License

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Frontend Development
- `yarn dev` - Start development server (Vite on port 1420)
- `yarn build` - Build frontend for production (TypeScript compilation + Vite build)
- `yarn preview` - Preview production build

### Tauri Development
- `yarn tauri dev` - Start Tauri development mode (builds Rust backend + launches app)
- `yarn tauri build` - Build production Tauri application
- `yarn tauri` - Access Tauri CLI commands

### Individual Commands
- `tsc` - TypeScript type checking
- `vite build` - Build frontend assets only

## Architecture Overview

This is a Tauri v2 application combining React (TypeScript) frontend with Rust backend:

### Frontend (React + TypeScript + Vite)
- **Entry point**: `src/main.tsx`
- **Main component**: `src/App.tsx` 
- **Build tool**: Vite with React plugin
- **Port**: Development server runs on port 1420 (fixed for Tauri integration)

### Backend (Rust + Tauri)
- **Entry point**: `src-tauri/src/main.rs` → calls `src-tauri/src/lib.rs::run()`
- **Tauri commands**: Defined in `lib.rs` with `#[tauri::command]` macro
- **Current commands**: `greet(name: &str) -> String`
- **Plugins**: Uses `tauri-plugin-opener` for system integration

### Configuration
- **Tauri config**: `src-tauri/tauri.conf.json`
  - App identifier: `net.noncore.reportify`
  - Window: 800x600 default size
  - Build commands: `yarn dev`/`yarn build`
- **Frontend dist**: Built to `dist/` and served from `../dist` relative to Tauri

### Communication Pattern
- Frontend calls Rust functions via `invoke()` from `@tauri-apps/api/core`
- Example: `await invoke("greet", { name })` calls Rust `greet()` function
- Rust functions must be registered in `invoke_handler` in `lib.rs`

## Project Structure Notes
- Frontend code in `src/`
- Rust backend in `src-tauri/src/`
- Vite ignores `src-tauri/` directory during frontend development
- Icons and assets for app bundling in `src-tauri/icons/`

## 📚 ドキュメント自動更新システム

このプロジェクトでは、開発中に得られた知識を体系的に管理し、既存ドキュメントに反映させるシステムを採用しています。

### 参照すべきドキュメント

作業開始時に必ず以下のドキュメントを確認してください：

- `README.md` - プロジェクトの基本説明とセットアップ手順
- `CLAUDE.md` - Claude Code開発ガイダンス（本ファイル）
- `package.json` - Frontend依存関係とスクリプト
- `src-tauri/Cargo.toml` - Rust依存関係
- `src-tauri/tauri.conf.json` - Tauriアプリケーション設定

### 更新ルール

#### 提案タイミング
以下の状況で、ドキュメント更新を提案してください：

1. **エラーや問題を解決した時**
2. **効率的な実装パターンを発見した時**
3. **新しいAPI/ライブラリの使用方法を確立した時**
4. **既存ドキュメントの情報が古い/不正確だと判明した時**
5. **頻繁に参照される情報を発見した時**
6. **コードレビューの修正を終わらせた時**

#### 提案フォーマット
💡 ドキュメント更新の提案： [状況の説明]
【更新内容】 [具体的な追加/修正内容]
【更新候補】
[ファイルパス1] - [理由]
[ファイルパス2] - [理由]
新規ファイル作成 - [理由]
どこに追加しますか？（番号を選択 or skip）

#### 承認プロセス
1. ユーザーが更新先を選択
2. 実際の更新内容をプレビュー表示
3. ユーザーが最終承認（yes/edit/no）
4. 承認後、ファイルを更新

### 既存ドキュメントとの連携

- 既存の記載形式やスタイルを踏襲すること
- 関連する既存内容がある場合は参照を明記すること
- 日付（YYYY-MM-DD形式）を含めて更新履歴を残すこと

### 重要な制約

1. **ユーザーの承認なしにファイルを更新しない**
2. **既存の内容を削除・変更せず、追加のみ行う**
3. **機密情報（APIキー、パスワード等）は記録しない**
4. **プロジェクトの慣習やスタイルガイドに従う**

### ドキュメントの分割管理

CLAUDE.mdが肥大化することを防ぐため、以下の基準で適切にファイルを分割してください：

- **100行を超えた場合**: 関連する内容を別ファイルに分離することを提案
- **推奨される分割方法**:
  - `.cursor/rules/update-system.md` - 更新システムのルール
  - `.cursor/rules/project-specific.md` - プロジェクト固有の設定
  - `.cursor/rules/references.md` - 参照すべきドキュメントのリスト
- **CLAUDE.mdには概要とリンクのみ残す**: 詳細は個別ファイルへ
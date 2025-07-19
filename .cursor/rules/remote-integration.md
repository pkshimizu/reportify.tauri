# リモートリポジトリ連携

## Git操作のベストプラクティス

### ブランチ戦略
- `main` - 本番環境用（安定版）
- `develop` - 開発統合用
- `feature/*` - 機能開発用
- `hotfix/*` - 緊急修正用

### コミットメッセージ規約
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメント
- `style`: フォーマット
- `refactor`: リファクタリング
- `test`: テスト
- `chore`: その他

**例:**
```
feat(frontend): add user authentication
fix(tauri): resolve window sizing issue
docs(readme): update installation guide
```

## PR/MRテンプレート

### Pull Request テンプレート
```markdown
## 概要
<!-- 変更内容の概要を記載 -->

## 変更内容
- [ ] Frontend変更
- [ ] Backend変更
- [ ] ドキュメント更新
- [ ] テスト追加

## テスト
- [ ] 手動テスト実施
- [ ] 自動テストパス
- [ ] ビルド確認

## チェックリスト
- [ ] TypeScript型チェックパス
- [ ] Rustコンパイル成功
- [ ] Lintエラーなし
- [ ] 関連ドキュメント更新
```

## CI/CD設定

### GitHub Actions推奨設定
```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install dependencies
        run: yarn install
      - name: TypeScript check
        run: yarn tsc
      - name: Build
        run: yarn build
      - name: Test Tauri
        run: yarn tauri build
```

### リリース戦略
1. `develop` ブランチで開発
2. `main` へのマージでリリースタグ作成
3. 自動ビルド＆デプロイ

## リモート操作コマンド

### 基本的な操作
```bash
# リモート確認
git remote -v

# プッシュ
git push origin feature/new-feature

# プルリクエスト作成後のマージ
git checkout main
git pull origin main
git branch -d feature/new-feature
```

### 競合解決
```bash
# リベース実行
git rebase main

# 競合解決後
git add .
git rebase --continue

# 強制プッシュ（注意して使用）
git push --force-with-lease origin feature/branch
```

## セキュリティ考慮事項

### 機密情報の管理
- `.env` ファイルは `.gitignore` に追加
- APIキーやトークンはコミットしない
- GitHub Secretsを活用

### アクセス権限
- リポジトリへの適切なアクセス権限設定
- ブランチ保護ルールの設定
- レビュー必須化

## バックアップ戦略

### 定期的な操作
- ローカルバックアップ: `git bundle create backup.bundle --all`
- タグによるバージョン管理
- 重要なブランチの保護

## 更新履歴
- 2025-07-19: 初期テンプレート作成
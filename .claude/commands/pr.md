---
description: Create GitHub Pull Request from feature branch
allowed-tools:
  - Bash(git branch:*)
  - Bash(git status:*)
  - Bash(git diff:*)
  - Bash(git log:*)
  - Bash(git push:*)
  - Bash(gh pr create:*)
  - Bash(gh pr view:*)
---

# pr command

GitHubのプルリクエストを作成してください。以下のルールに従ってください：

## Pull Request Rules

1. **feature/*ブランチのみ対応**
   - 現在のブランチ名を確認する
   - `feature/*`パターンでない場合はエラーメッセージを表示して終了する
   - エラーメッセージ: "プルリクエストは feature/* ブランチからのみ作成できます"

2. **ブランチ名からIssue IDを取得**
   - ブランチ名のパターン: `feature/{issue id}-{issue title}`
   - 例: `feature/123-add-new-feature` → Issue ID: 123
   - Issue IDが取得できない場合は、対応Issueは無視する

3. **コミット履歴を確認して概要と詳細を生成**
   - `git log main..HEAD --oneline` でブランチのコミット履歴を確認する
   - `git diff main...HEAD` でブランチの全変更内容を確認する
   - コミットメッセージと変更内容から、プルリクエストの概要（1行）と詳細を生成する

4. **プルリクエストテンプレートに従う**
   - `.github/pull_request_template.md` の形式に従う
   - 概要: 一行程度の短い文章でプルリクエストの概要を記述
   - 対応Issue: Issue IDが取得できた場合のみ記載
     - 形式: `- fixes: https://github.com/pkshimizu/reportify.tauri/issues/{issue id}`
   - 詳細: 具体的に追加・変更した内容を箇条書きで記述

5. **プルリクエストを作成**
   - リモートブランチが存在しない場合は `git push -u origin {branch name}` で push
   - `gh pr create` コマンドでプルリクエストを作成
   - base ブランチは `main` を使用
   - 作成後、プルリクエストのURLを表示する

## 手順

1. 現在のブランチ名を確認し、`feature/*`パターンであることを検証
2. ブランチ名から Issue ID を抽出（可能な場合）
3. `git log` と `git diff` でブランチの変更内容を確認
4. プルリクエストの概要と詳細を生成
5. リモートブランチの存在確認と push（必要な場合）
6. `gh pr create` でプルリクエストを作成
7. 作成されたプルリクエストのURLを表示

## 注意事項

- プルリクエストの内容は日本語で記述する
- コミットメッセージに基づいて、わかりやすく簡潔に記述する
- 技術的な詳細は箇条書きで整理する

---
description: Git commit with Japanese message following project rules
allowed-tools:
  - Bash(npm run fix)
  - Bash(git add:*)
  - Bash(git commit:*)
  - Bash(git status:*)
  - Bash(git diff:*)
  - Bash(git log:*)
---

Git commit を実行してください。以下のルールに従ってください：

## Git Commit Rules

1. **コミットメッセージは日本語で書くこと**
   - コミットメッセージは日本語で記述する
   - 変更内容を分かりやすく簡潔に説明する
   - コミットメッセージに、claudeで記述したことを載せない

2. **コミット前にlintを実行すること**
   - `yarn fix` を実行してエラーがないことを確認する
   - リンタエラーがある場合は修正してからコミットする

3. **差分を確認して簡潔なメッセージにすること**
   - `git status` と `git diff` で変更内容を確認する
   - 変更の目的と内容を簡潔にまとめたメッセージを作成する
   - 複数の変更がある場合は、主要な変更にフォーカスする

## 手順

1. `yarn fix` を実行してlintエラーがないことを確認
2. `git status` と `git diff` を並列実行して変更内容を確認
3. 変更内容に基づいて簡潔な日本語のコミットメッセージを作成
4. `git add .` でファイルをステージング
5. `git commit` でコミット実行
6. `git status` でコミット結果を確認
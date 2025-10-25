# Database Migration

## 概要

このディレクトリにはSeaORMを使用したデータベースマイグレーションファイルが含まれています。
アプリケーション起動時に自動的にマイグレーションが実行されますが、手動で実行することも可能です。

## Makefileコマンド（推奨）

親ディレクトリ（src-tauri）から以下のコマンドを実行できます：

- 新しいマイグレーションファイルを生成

    ```sh
    make migrate-generate
    ```

- すべての保留中のマイグレーションを適用

    ```sh
    make migrate-up
    ```

- 最後に適用したマイグレーションをロールバック

    ```sh
    make migrate-down
    ```

- すべてのテーブルを削除して再度マイグレーションを実行

    ```sh
    make migrate-fresh
    ```

## Running Migrator CLI

- Generate a new migration file

    ```sh
    cargo run -- generate MIGRATION_NAME
    ```

- Apply all pending migrations

    ```sh
    cargo run
    ```

    ```sh
    cargo run -- up
    ```

- Apply first 10 pending migrations

    ```sh
    cargo run -- up -n 10
    ```

- Rollback last applied migrations

    ```sh
    cargo run -- down
    ```

- Rollback last 10 applied migrations

    ```sh
    cargo run -- down -n 10
    ```

- Drop all tables from the database, then reapply all migrations

    ```sh
    cargo run -- fresh
    ```

- Rollback all applied migrations, then reapply all migrations

    ```sh
    cargo run -- refresh
    ```

- Rollback all applied migrations

    ```sh
    cargo run -- reset
    ```

- Check the status of all migrations

    ```sh
    cargo run -- status
    ```

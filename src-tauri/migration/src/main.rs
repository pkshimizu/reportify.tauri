use sea_orm_migration::prelude::*;
use std::path::PathBuf;

#[async_std::main]
async fn main() {
    // DATABASE_URL環境変数が設定されていない場合は、アプリケーションのデフォルトURLを設定
    if std::env::var("DATABASE_URL").is_err() {
        if let Ok(db_url) = get_default_database_url() {
            std::env::set_var("DATABASE_URL", db_url);
        } else {
            eprintln!("Failed to get database URL");
            std::process::exit(1);
        }
    }

    cli::run_cli(migration::Migrator).await;
}

/// デフォルトのデータベースURLを取得する
/// database.rsと同じロジックを使用
fn get_default_database_url() -> Result<String, String> {
    let app_name = "reportify";

    // アプリケーションデータフォルダのパスを取得
    let app_data_dir =
        dirs::data_dir().ok_or_else(|| "Failed to get application data directory".to_string())?;

    // アプリケーション専用フォルダを作成
    let app_dir = app_data_dir.join(app_name);
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create application directory: {e}"))?;

    // データベースファイルのパス
    let db_path: PathBuf = app_dir.join(format!("{app_name}.db"));

    Ok(format!("sqlite://{}?mode=rwc", db_path.display()))
}

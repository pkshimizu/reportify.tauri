use migration::{Migrator, MigratorTrait};
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::path::PathBuf;

static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

/// データベースファイルのパスを取得する
fn get_database_path() -> Result<PathBuf, String> {
    let app_name = "reportify";

    // アプリケーションデータフォルダのパスを取得
    let app_data_dir =
        dirs::data_dir().ok_or_else(|| "Failed to get application data directory".to_string())?;

    // アプリケーション専用フォルダを作成
    let app_dir = app_data_dir.join(app_name);
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create application directory: {e}"))?;

    // データベースファイルのパス
    let db_path = app_dir.join(format!("{app_name}.db"));

    Ok(db_path)
}

/// データベース接続URL文字列を取得する（マイグレーション用）
///
/// # Errors
/// データベースパスの取得に失敗した場合にエラーを返す
pub fn get_database_url() -> Result<String, String> {
    let db_path = get_database_path()?;
    Ok(format!("sqlite://{}?mode=rwc", db_path.display()))
}

/// データベース接続を初期化する
pub async fn initialize_database() -> Result<(), DbErr> {
    let db_url = get_database_url().map_err(DbErr::Custom)?;

    log::info!("Initializing database at: {db_url}");

    let db = Database::connect(&db_url).await?;

    // マイグレーションを実行
    log::info!("Running database migrations...");
    Migrator::up(&db, None).await?;
    log::info!("Database migrations completed");

    DB_CONNECTION
        .set(db)
        .map_err(|_| DbErr::Custom("Database already initialized".to_string()))?;

    log::info!("Database initialized successfully");

    Ok(())
}

/// データベース接続を取得する
///
/// # Panics
/// データベースが初期化されていない場合にパニックする
pub fn get_connection() -> &'static DatabaseConnection {
    DB_CONNECTION
        .get()
        .expect("Database not initialized. Call initialize_database() first.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_database_path() {
        let path = get_database_path().unwrap();
        assert!(path.to_string_lossy().contains("reportify"));
        assert!(path.to_string_lossy().ends_with("reportify.db"));
    }
}

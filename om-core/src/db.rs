//! This module facilitates interactions with the underlying SQLite database.

use const_format::concatcp;
use sqlx::{FromRow, Pool, migrate::MigrateDatabase, sqlite::*};

use crate::media::*;

#[derive(Debug)]
pub struct Db {
    pool: Pool<Sqlite>,
}

impl Db {
    pub async fn new_in_memory() -> Result<Db, sqlx::Error> {
        Self::new_at_url("sqlite::memory:").await
    }

    pub async fn new_at_url<'a>(url: &'a str) -> Result<Db, sqlx::Error> {
        if !Sqlite::database_exists(url).await.unwrap() {
            Sqlite::create_database(url).await.unwrap();
        }

        let pool = SqlitePoolOptions::new().connect(url).await?;
        sqlx::query(include_str!("../migrations/20251123060115_init.sql"))
            .execute(&pool)
            .await?;

        Ok(Db { pool })
    }

    pub async fn disconnect(&self) {
        self.pool.close().await
    }

    pub async fn add<'a, T: SQLiteCompat<'a>>(&self, entity: T) -> Result<i64, sqlx::Error> {
        let id = entity
            .insert()
            .execute(&self.pool)
            .await?
            .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_all<'a, T: SQLiteCompat<'a>>(&self) -> Result<Vec<T>, sqlx::Error> {
        self.get("").await
    }

    pub async fn get<'a, T: SQLiteCompat<'a>>(
        &self,
        trailing: &str,
    ) -> Result<Vec<T>, sqlx::Error> {
        let query = format!("SELECT * FROM {} {}", T::TABLE_NAME, trailing);
        sqlx::query_as(&query)
            .bind(T::TABLE_NAME)
            .fetch_all(&self.pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{db::Db, media::*};

    #[tokio::test]
    async fn use_memory_db() {
        let db = Db::new_in_memory().await.unwrap();

        let artist = db
            .add(Artist {
                name: "Aquilus".to_string(),
                description: Some("Solo Orchestral Atmospheric Black Metal Project".to_string()),
                location: Some("Australia".to_string()),
                ..Default::default()
            })
            .await
            .unwrap() as u32;

        let track = db
            .add(Track {
                title: "Nihil".to_string(),
                artist: artist,
                ..Default::default()
            })
            .await
            .unwrap();

        let tracks = db.get_all::<Track>().await.unwrap();
        println!("Current tracks: {:?}", tracks);
        let artists = db.get_all::<Artist>().await.unwrap();
        println!("Current artists: {:?}", artists);

        db.disconnect().await;
    }

    #[tokio::test]
    async fn use_file_db() {
        let db = Db::new_at_url("sqlite:test.db").await.unwrap();

        let artist = db
            .add(Artist {
                name: "Aquilus".to_string(),
                description: Some("Solo Orchestral Atmospheric Black Metal Project".to_string()),
                location: Some("Australia".to_string()),
                ..Default::default()
            })
            .await
            .unwrap() as u32;

        let track = db
            .add(Track {
                title: "Nihil".to_string(),
                artist: artist,
                ..Default::default()
            })
            .await
            .unwrap();

        let tracks = db.get_all::<Track>().await.unwrap();
        println!("Current tracks: {:?}", tracks);
        let artists = db.get_all::<Artist>().await.unwrap();
        println!("Current artists: {:?}", artists);

        db.disconnect().await;

        // std::fs::remove_file(Path::new("test.db")).unwrap();
        // std::fs::remove_file(Path::new("test.db-wal")).unwrap();
        // std::fs::remove_file(Path::new("test.db-shm")).unwrap();
    }
}

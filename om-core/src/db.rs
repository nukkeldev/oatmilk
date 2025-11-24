//! This module facilitates interactions with the underlying SQLite database.

use sqlx::{Pool, migrate::MigrateDatabase, sqlite::*};

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
        Ok(Db { pool })
    }

    pub async fn try_setup(&mut self) -> Result<(), sqlx::Error> {
        sqlx::query(include_str!("../migrations/20251123060115_init.sql"))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.pool.close().await
    }

    pub async fn add<T: Insertable<Sqlite>>(&mut self, entity: T) -> Result<i64, sqlx::Error> {
        let id = entity
            .insert()
            .execute(&self.pool)
            .await?
            .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_songs(&mut self) -> Result<Vec<Song>, sqlx::Error> {
        sqlx::query_as::<_, Song>(
            r#"
            SELECT * FROM songs;
        "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_artists(&mut self) -> Result<Vec<Artist>, sqlx::Error> {
        sqlx::query_as::<_, Artist>(
            r#"
            SELECT * FROM artists;
        "#,
        )
        .fetch_all(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{db::Db, media::*};

    #[tokio::test]
    async fn use_memory_db() {
        let mut db = Db::new_in_memory().await.unwrap();
        db.try_setup().await.unwrap();

        let artist = db
            .add(Artist {
                name: "Aquilus".to_string(),
                description: Some("Solo Orchestral Atmospheric Black Metal Project".to_string()),
                location: Some("Australia".to_string()),
                ..Default::default()
            })
            .await
            .unwrap() as u32;

        let song = db
            .add(Song {
                title: "Nihil".to_string(),
                artist: artist,
                ..Default::default()
            })
            .await
            .unwrap();

        let songs = db.get_songs().await.unwrap();
        println!("Current songs: {:?}", songs);
        let artists = db.get_artists().await.unwrap();
        println!("Current artists: {:?}", artists);

        db.disconnect().await;
    }

    #[tokio::test]
    async fn use_file_db() {
        let mut db = Db::new_at_url("sqlite:test.db").await.unwrap();
        db.try_setup().await.unwrap();

        let artist = db
            .add(Artist {
                name: "Aquilus".to_string(),
                description: Some("Solo Orchestral Atmospheric Black Metal Project".to_string()),
                location: Some("Australia".to_string()),
                ..Default::default()
            })
            .await
            .unwrap() as u32;

        let song = db
            .add(Song {
                title: "Nihil".to_string(),
                artist: artist,
                ..Default::default()
            })
            .await
            .unwrap();

        let songs = db.get_songs().await.unwrap();
        println!("Current songs: {:?}", songs);
        let artists = db.get_artists().await.unwrap();
        println!("Current artists: {:?}", artists);

        db.disconnect().await;

        // std::fs::remove_file(Path::new("test.db")).unwrap();
        // std::fs::remove_file(Path::new("test.db-wal")).unwrap();
        // std::fs::remove_file(Path::new("test.db-shm")).unwrap();
    }
}

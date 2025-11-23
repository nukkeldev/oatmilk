//! This module facilitates interactions with the underlying SQLite database.

use sqlx::{Pool, sqlite::*};

use crate::media::{NewSong, Song};

#[derive(Debug)]
pub struct Db {
    pool: Pool<Sqlite>,
}

impl Db {
    pub async fn pool<'a>(options: SqlitePoolOptions, url: &'a str) -> Result<Db, sqlx::Error> {
        let pool = options.connect(url).await?;
        Ok(Db { pool })
    }

    pub async fn try_setup(&mut self) -> Result<(), sqlx::Error> {
        sqlx::query(include_str!("../migrations/20251123060115_init.sql"))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn add_song(&mut self, song: NewSong) -> Result<i64, sqlx::Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO Songs (title, album, author) VALUES (?, ?, ?)
        "#,
            song.title,
            song.album,
            song.author,
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn get_songs(&mut self) -> Result<Vec<Song>, sqlx::Error> {
        sqlx::query_as!(
            Song,
            r#"
            SELECT * FROM Songs;
        "#
        )
        .fetch_all(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::{db::Db, media::NewSong};

    #[tokio::test]
    async fn connect_to_db() {
        _ = Db::pool(SqlitePoolOptions::new(), "sqlite::memory:")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn use_db() {
        let mut db = Db::pool(SqlitePoolOptions::new(), "sqlite::memory:")
            .await
            .unwrap();
        db.try_setup().await.unwrap();

        let id = db
            .add_song(NewSong {
                title: "".to_string(),
                album: None,
                author: None,
            })
            .await
            .unwrap();

        println!("Added song with ID#{}", id);

        let songs = db.get_songs().await.unwrap();
        println!("Current songs: {:?}", songs);
    }
}

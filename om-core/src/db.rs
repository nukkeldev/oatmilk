//! This module facilitates interactions with the underlying SQLite database.

use sqlx::{Pool, sqlite::*};

use crate::media::Song;

#[derive(Debug)]
pub struct Db {
    pool: Pool<Sqlite>,
}

impl Db {
    pub async fn new_in_memory() -> Result<Db, sqlx::Error> {
        Self::new_at_url("sqlite::memory:").await
    }

    pub async fn new_at_url<'a>(url: &'a str) -> Result<Db, sqlx::Error> {
        let pool = SqlitePoolOptions::new().connect(url).await?;
        Ok(Db { pool })
    }

    pub async fn try_setup(&mut self) -> Result<(), sqlx::Error> {
        sqlx::query(include_str!("../migrations/20251123060115_init.sql"))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn add_song(&mut self, song: Song) -> Result<i64, sqlx::Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO Songs 
                (title, parent_type, parent_id, author, duration, description) 
            VALUES (?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(song.title)
        .bind(song.parent.r#type as i64)
        .bind(song.parent.id)
        .bind(song.author)
        .bind(song.duration)
        // .bind(song.tags)
        .bind(song.description)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn get_songs(&mut self) -> Result<Vec<Song>, sqlx::Error> {
        sqlx::query_as::<_, Song>(
            r#"
            SELECT * FROM Songs;
        "#,
        )
        .fetch_all(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{db::Db, media::Song};

    #[tokio::test]
    async fn use_db() {
        let mut db = Db::new_in_memory().await.unwrap();
        db.try_setup().await.unwrap();

        let id = db
            .add_song(Song {
                title: "".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();

        println!("Added song with ID#{}", id);

        let songs = db.get_songs().await.unwrap();
        println!("Current songs: {:?}", songs);
    }
}

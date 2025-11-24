//! Here we define the various required media structures. Definitions here must match those
//! in the DBML model (and thus the SQLx migration). When inserting new rows, auto-computed
//! fields are ignored (hence why Default is implemented).
//!
//! All collated structures are identified by a unique, auto-incremented unsigned 32-bit INTEGER.
//! Unforunately, that means we can only store 4 billion structures of each type.

use chrono::{DateTime, Utc};
use sqlx::{Database, Sqlite, query::Query};

pub type ID = u32;

// -- Traits -- //

pub trait Insertable<DB: Database> {
    fn insert<'a>(self) -> Query<'a, DB, <DB as Database>::Arguments<'a>>;
}

// -- Structures -- //

#[derive(Debug, Default, sqlx::FromRow)]
pub struct Song {
    pub id: ID,

    pub title: String,

    pub collection: Option<ID>,
    pub artist: ID,

    pub cover_of: Option<ID>,

    pub released: Option<DateTime<Utc>>,

    pub duration: Option<f32>,
    pub description: Option<String>,

    // TODO: Define strict rating system.
    /// A composite, subjective rating for the song.
    /// Possibly further elaborated by the `review`.
    pub rating: Option<f32>,
    pub review: Option<String>,
}

impl Insertable<Sqlite> for Song {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO songs 
                (
                    title, collection, artist, cover_of,
                    released, duration, description, rating,
                    review
                )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        )
        .bind(self.title)
        .bind(self.collection)
        .bind(self.artist)
        .bind(self.cover_of)
        .bind(self.duration)
        .bind(self.description)
        .bind(self.rating)
        .bind(self.review)
    }
}

#[derive(Debug, Default, sqlx::FromRow)]
pub struct Collection {
    pub id: ID,

    pub title: String,
    pub description: Option<String>,

    pub released: Option<DateTime<Utc>>,

    pub artist: ID,
}

impl Insertable<Sqlite> for Collection {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO collections 
                (title, description, released, artist)
            VALUES ($1, $2, $3, $4)
        "#,
        )
        .bind(self.title)
        .bind(self.description)
        .bind(self.released)
        .bind(self.artist)
    }
}

#[derive(Debug, Default, sqlx::FromRow)]
pub struct Artist {
    pub id: ID,

    pub name: String,
    pub description: Option<String>,

    pub location: Option<String>,

    pub year_founded: Option<DateTime<Utc>>,
    pub year_ended: Option<DateTime<Utc>>,
}

impl Insertable<Sqlite> for Artist {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO artists 
                (name, description, location, year_founded, year_ended)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        )
        .bind(self.name)
        .bind(self.description)
        .bind(self.location)
        .bind(self.year_founded)
        .bind(self.year_ended)
    }
}

#[derive(Debug, Default, sqlx::FromRow)]
pub struct Person {
    pub id: ID,

    pub name: String,
    pub description: Option<String>,

    pub birth: Option<DateTime<Utc>>,
    pub birthplace: Option<String>,

    pub death: Option<DateTime<Utc>>,
    pub deathplace: Option<String>,
}

impl Insertable<Sqlite> for Person {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO artists 
                (name, description, birth, birthplace, death, deathplace)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        )
        .bind(self.name)
        .bind(self.description)
        .bind(self.birth)
        .bind(self.birthplace)
        .bind(self.death)
        .bind(self.deathplace)
    }
}

#[derive(Debug, Default, sqlx::FromRow)]
pub struct Tag {
    pub id: ID,

    pub name: String,
    pub description: Option<String>,
}

impl Insertable<Sqlite> for Tag {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO tags
                (name, description)
            VALUES ($1, $2)
        "#,
        )
        .bind(self.name)
        .bind(self.description)
    }
}

// -- Relations -- //

#[derive(Debug, Default, sqlx::FromRow)]
pub struct PersonToArtist {
    pub person_id: ID,
    pub artist_id: ID,

    pub time_start: Option<DateTime<Utc>>,
    pub time_end: Option<DateTime<Utc>>,

    pub role: Option<String>,
}

impl Insertable<Sqlite> for PersonToArtist {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO persons_to_artists
                (person_id, artist_id, time_start, time_end, role)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        )
        .bind(self.person_id)
        .bind(self.artist_id)
        .bind(self.time_start)
        .bind(self.time_end)
        .bind(self.role)
    }
}

#[derive(Debug, Default, sqlx::FromRow)]
pub struct SongToTag {
    pub song_id: ID,
    pub tag_id: ID,
}

impl Insertable<Sqlite> for SongToTag {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO songs_to_tags
                (song_id, tag_id)
            VALUES ($1, $2)
        "#,
        )
        .bind(self.song_id)
        .bind(self.tag_id)
    }
}

#[derive(Debug, Default, sqlx::FromRow)]
pub struct ArtistToTag {
    pub artist_id: ID,
    pub tag_id: ID,
}

impl Insertable<Sqlite> for ArtistToTag {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO songs_to_tags
                (artist_id, tag_id)
            VALUES ($1, $2)
        "#,
        )
        .bind(self.artist_id)
        .bind(self.tag_id)
    }
}

#[derive(Debug, Default, sqlx::FromRow)]
pub struct TagToParentTag {
    pub tag_id: ID,
    pub parent_id: ID,
}

impl Insertable<Sqlite> for TagToParentTag {
    fn insert<'a>(self) -> Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>> {
        sqlx::query(
            r#"
            INSERT INTO tags_to_parent_tags
                (tag_id, parent_id)
            VALUES ($1, $2)
        "#,
        )
        .bind(self.tag_id)
        .bind(self.parent_id)
    }
}

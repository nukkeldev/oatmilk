//! Here we define the various required media structures. Definitions here must match those
//! in the DBML model (and thus the SQLx migration). When inserting new rows, auto-computed
//! fields are ignored (hence why Default is implemented).
//!
//! All collated structures are identified by a unique, auto-incremented unsigned 32-bit INTEGER.
//! Unforunately, that means we can only store 4 billion structures of each type.

use chrono::{DateTime, Utc};
use om_core_proc::SQLiteCompat;
use sqlx::{
    FromRow, Sqlite,
    query::{Query, QueryAs},
    sqlite::SqliteRow,
};

pub type ID = u32;
pub type SqliteQuery<'a> = Query<'a, Sqlite, <sqlx::Sqlite as sqlx::Database>::Arguments<'a>>;
pub type SqliteQueryAs<'a, T> =
    QueryAs<'a, Sqlite, T, <sqlx::Sqlite as sqlx::Database>::Arguments<'a>>;

// -- Traits -- //

pub trait SQLiteCompat<'a>: Sized + Send + Unpin + for<'r> FromRow<'r, SqliteRow> {
    const TABLE_NAME: &'static str;

    /// Inserts `self`` into the table.
    fn insert(self) -> SqliteQuery<'a>;
}

// -- Structures -- //

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "tracks")]
pub struct Track {
    pub id: ID,

    pub title: String,

    pub collection: Option<ID>,
    pub artist: ID,

    pub cover_of: Option<ID>,

    pub released: Option<DateTime<Utc>>,

    pub duration: Option<u32>,
    pub description: Option<String>,

    // TODO: Define strict rating system.
    /// A composite, subjective rating for the track.
    /// Possibly further elaborated by the `review`.
    pub rating: Option<f32>,
    pub review: Option<String>,
}

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "collections")]
pub struct Collection {
    pub id: ID,

    pub title: String,
    pub description: Option<String>,

    pub released: Option<DateTime<Utc>>,

    pub artist: ID,
}

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "artists")]
pub struct Artist {
    pub id: ID,

    pub name: String,
    pub description: Option<String>,

    pub location: Option<String>,

    pub year_founded: Option<DateTime<Utc>>,
    pub year_ended: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "people")]
pub struct Person {
    pub id: ID,

    pub name: String,
    pub description: Option<String>,

    pub birth: Option<DateTime<Utc>>,
    pub birthplace: Option<String>,

    pub death: Option<DateTime<Utc>>,
    pub deathplace: Option<String>,
}

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "tags")]
pub struct Tag {
    pub id: ID,

    pub name: String,
    pub description: Option<String>,
}

// -- Relations -- //

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "persons_to_artists")]
pub struct PersonToArtist {
    pub person_id: ID,
    pub artist_id: ID,

    pub time_start: Option<DateTime<Utc>>,
    pub time_end: Option<DateTime<Utc>>,

    pub role: Option<String>,
}

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "tracks_to_tags")]
pub struct TrackToTag {
    pub track_id: ID,
    pub tag_id: ID,
}

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "artists_to_tags")]
pub struct ArtistToTag {
    pub artist_id: ID,
    pub tag_id: ID,
}

#[derive(Debug, Default, sqlx::FromRow, SQLiteCompat)]
#[sqlite_compat(table_name = "tags_to_parent_tags")]
pub struct TagToParentTag {
    pub tag_id: ID,
    pub parent_id: ID,
}

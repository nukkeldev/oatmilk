//! Here we define the various required media structures. Definitions here must match those
//! in the SQLx migration. When inserting new rows, auto-computed fields are ignored (hence
//! why Default is implemented).

/// A song entry.
#[derive(Debug, Default, sqlx::FromRow)]
pub struct Song {
    /// A unique auto-incremented ID for the song.
    /// Should be used as a unique reference to this entry.
    pub id: i64,

    /// The title of the song.
    pub title: String,
    /// The parent under which this work falls under.
    #[sqlx(flatten)]
    pub parent: Parent,
    /// The ID of the author.
    pub author: i64,

    /// How long the song is.
    pub duration: Option<f32>,

    /// Tag IDs to associate with the song.
    #[sqlx(json)]
    pub tags: Vec<i64>,
    /// A description of the song.
    pub description: Option<String>,
}

#[derive(Debug, Default, num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[repr(i64)]
pub enum ParentType {
    #[default]
    None = 0,
    Album = 1,
}

#[derive(Debug, sqlx::FromRow, Default)]
pub struct Parent {
    #[sqlx(try_from = "i64", rename = "parent_type")]
    pub r#type: ParentType,
    #[sqlx(rename = "parent_id")]
    pub id: Option<i64>,
}

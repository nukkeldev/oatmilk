/// A song entry.
#[derive(Debug, sqlx::FromRow)]
pub struct Song {
    /// An unique identifier.
    pub id: i64,
    /// The title of the song.
    pub title: String,
    // TODO: This doesn't fit singles, compilations, etc.
    /// An ID to the album the song is associated with.
    pub album: Option<i64>,
    /// An ID to the author of the song.
    pub author: Option<i64>,
}

/// A new song.
#[derive(Debug)]
pub struct NewSong {
    /// The title of the song.
    pub title: String,
    /// An ID to the album the song is associated with.
    pub album: Option<i64>,
    /// An ID to the author of the song.
    pub author: Option<i64>,
}

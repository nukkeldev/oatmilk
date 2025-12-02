CREATE TABLE IF NOT EXISTS tracks (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  collection INTEGER,
  artist INTEGER NOT NULL,
  cover_of INTEGER,
  released INTEGER,
  duration INTEGER,
  description TEXT,
  rating REAL,
  review TEXT,
  FOREIGN KEY(collection) REFERENCES collections(id),
  FOREIGN KEY(artist) REFERENCES artists(id),
  FOREIGN KEY(cover_of) REFERENCES tracks(id)
);
CREATE TABLE IF NOT EXISTS collections (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  released INTEGER,
  artist INTEGER NOT NULL,
  FOREIGN KEY(artist) REFERENCES artists(id)
);
CREATE TABLE IF NOT EXISTS artists (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  location TEXT,
  year_founded INTEGER,
  year_ended INTEGER
);
CREATE TABLE IF NOT EXISTS persons_to_artists (
  person_id INTEGER NOT NULL,
  artist_id INTEGER NOT NULL,
  time_start INTEGER,
  time_end INTEGER,
  role TEXT,
  FOREIGN KEY(person_id) REFERENCES people(id),
  FOREIGN KEY(artist_id) REFERENCES artists(id)
);
CREATE TABLE IF NOT EXISTS people (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  birth INTEGER,
  birthplace TEXT,
  death INTEGER,
  deathplace TEXT
);
CREATE TABLE IF NOT EXISTS tracks_to_tags (
  track_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  FOREIGN KEY(track_id) REFERENCES tracks(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
);
CREATE TABLE IF NOT EXISTS artists_to_tags (
  artist_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  FOREIGN KEY(artist_id) REFERENCES artists(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
);
CREATE TABLE IF NOT EXISTS tags_to_parent_tags (
  tag_id INTEGER NOT NULL UNIQUE,
  parent_id INTEGER NOT NULL,
  FOREIGN KEY(tag_id) REFERENCES tags(id),
  FOREIGN KEY(parent_id) REFERENCES tags(id)
);
CREATE TABLE IF NOT EXISTS tags (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE,
  description TEXT
);


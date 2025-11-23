CREATE TABLE songs (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  collection INTEGER,
  artist INTEGER,
  cover_of INTEGER,
  released INTEGER,
  duration REAL,
  description TEXT,
  rating REAL,
  review TEXT,
  FOREIGN KEY(collection) REFERENCES collections(id),
  FOREIGN KEY(artist) REFERENCES artists(id),
  FOREIGN KEY(cover_of) REFERENCES songs(id)
);
CREATE TABLE collections (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  released INTEGER,
  artist INTEGER NOT NULL,
  FOREIGN KEY(artist) REFERENCES artists(id)
);
CREATE TABLE artists (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  location TEXT,
  year_founded INTEGER,
  year_ended INTEGER
);
CREATE TABLE person_to_artist (
  person_id INTEGER NOT NULL,
  artist_id INTEGER NOT NULL,
  time_start INTEGER,
  time_end INTEGER,
  role TEXT,
  FOREIGN KEY(person_id) REFERENCES people(id),
  FOREIGN KEY(artist_id) REFERENCES artists(id)
);
CREATE TABLE people (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  birth INTEGER,
  birthplace TEXT,
  death INTEGER,
  deathplace TEXT
);
CREATE TABLE song_to_tag (
  song_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  FOREIGN KEY(song_id) REFERENCES songs(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
);
CREATE TABLE artist_to_tag (
  artist_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  FOREIGN KEY(artist_id) REFERENCES artists(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
);
CREATE TABLE tag_to_parent_tag (
  tag_id INTEGER NOT NULL UNIQUE,
  parent_id INTEGER NOT NULL,
  FOREIGN KEY(tag_id) REFERENCES tags(id),
  FOREIGN KEY(parent_id) REFERENCES tags(id)
);
CREATE TABLE tags (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE,
  description TEXT
);


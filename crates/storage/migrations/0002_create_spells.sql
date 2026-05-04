-- The core table for spells.
CREATE TABLE spells (
    id               BLOB    PRIMARY KEY,
    name             TEXT    NOT NULL,
    school           TEXT    NOT NULL,
    level            TEXT    NOT NULL,
    source           TEXT,
    casting_time     TEXT    NOT NULL,
    ritual           INTEGER NOT NULL CHECK (ritual        IN (0, 1)) DEFAULT 0,
    concentration    INTEGER NOT NULL CHECK (concentration IN (0, 1)) DEFAULT 0,
    verbal           INTEGER NOT NULL CHECK (verbal        IN (0, 1)) DEFAULT 0,
    somatic          INTEGER NOT NULL CHECK (somatic       IN (0, 1)) DEFAULT 0,
    material         INTEGER NOT NULL CHECK (material      IN (0, 1)) DEFAULT 0,

    -- This is inline JSON, because of its complex structure essentially warranting
    -- its own table. However, there is no point in creating a separate table
    -- given that spell materials by themselves are never of interest
    -- and are always fetched as part of the spell itself.
    materials        TEXT,
    duration         TEXT    NOT NULL,
    range            TEXT    NOT NULL,
    area             TEXT    NOT NULL,

    -- This is inline JSON, because of the inconsistent structure of a spell shape.
    shape            TEXT    NOT NULL,
    description      TEXT    NOT NULL,
    at_higher_levels TEXT,
    flavor_text      TEXT,
    attribution      TEXT
);

-- The core table for tags, containing the identifier and value of a given tag.
-- Entity specific tables like spell_tags reference this table.
--
-- This table exists to make tag searching easier across entities,
-- and for providing easier tag auto-complete in forms.
CREATE TABLE tags (
    id    BLOB PRIMARY KEY,
    value TEXT NOT NULL UNIQUE
);

CREATE TABLE spell_aliases (
    spell_id BLOB NOT NULL REFERENCES spells(id) ON DELETE CASCADE,
    alias    TEXT NOT NULL,
    PRIMARY KEY (spell_id, alias)
);

CREATE TABLE spell_tags (
    spell_id BLOB NOT NULL REFERENCES spells(id) ON DELETE CASCADE,
    tag_id   BLOB NOT NULL REFERENCES tags(id)   ON DELETE CASCADE,
    PRIMARY KEY (spell_id, tag_id)
);

CREATE TABLE spell_classes (
    spell_id BLOB NOT NULL REFERENCES spells(id) ON DELETE CASCADE,
    class    TEXT NOT NULL,
    PRIMARY KEY (spell_id, class)
);

CREATE TABLE spell_images (
    -- The id functions as the path as well, since it is just {data_dir}/{id}.
    id       BLOB PRIMARY KEY,
    spell_id BLOB NOT NULL REFERENCES spells(id) ON DELETE CASCADE
);

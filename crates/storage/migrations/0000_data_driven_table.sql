-- The generic entity-attribute-value table used to contain data-driven option values.
--
-- Instead of creating a bunch of separate data-driven tables with similar structure,
-- like spell_schools, spell_levels and the like, this single table contains the same
-- underlying structure, while being able to contain all the different variants.
--
-- The main trade-off is that FK constraints cannot enforce the correct variants,
-- however given that we own the application layer, this is a minor concern.
CREATE TABLE options (
    -- While SQLite does not support a native UUID type, storing it as a BLOB,
    -- rather than a TEXT uses around half the bytes, although less human readable.
    id BLOB PRIMARY KEY,

    -- This defines the variant of entry, and is what most queries will filter on
    -- to find the entries relating only to that specific subset of options.
    variant TEXT NOT NULL,

    -- The user-facing value of the option.
    value TEXT NOT NULL,

    -- The sort order is not table-wide, but variant-wide.
    -- In other words, each variant has its own "internal" sort order in this column.
    sort_order INTEGER NOT NULL,

    -- Whether the entry should be protected from deletion,
    -- when there exists no references to the entry.
    protected INTEGER NOT NULL CHECK (protected IN (0, 1)) DEFAULT 0,

    -- In theory, the same value is allowed across different variants in this table.
    -- However, within the same variant, the entries should be unique.
    UNIQUE(variant, value)
);

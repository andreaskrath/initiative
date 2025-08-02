CREATE TABLE IF NOT EXISTS spells (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    school magic_school NOT NULL,
    level spell_level NOT NULL,
    verbal boolean NOT NULL,
    somatic boolean NOT NULL,
    material text,
    material_consumed boolean NOT NULL,
    ritual boolean NOT NULL,
    concentration boolean NOT NULL,
    casting_time text NOT NULL,
    duration text NOT NULL,
    range text NOT NULL,
    area text NOT NULL,
    shape text,
    description text NOT NULL,
    at_higher_levels text
);

CREATE INDEX IF NOT EXISTS idx_spells_school ON spells(school);
CREATE INDEX IF NOT EXISTS idx_spells_level ON spells(level);

CREATE TABLE IF NOT EXISTS spell_classes (
    spell_id uuid NOT NULL REFERENCES spells(id) ON DELETE CASCADE,
    class class NOT NULL,
    PRIMARY KEY (spell_id, class)
);
CREATE INDEX IF NOT EXISTS idx_spell_classes_class ON spell_classes(class);

CREATE OR REPLACE VIEW v_spells AS
SELECT
    spells.*,
    ARRAY (
        SELECT class
        FROM spell_classes
        WHERE spell_id = spells.id
        ORDER BY class
    ) as classes
FROM spells;

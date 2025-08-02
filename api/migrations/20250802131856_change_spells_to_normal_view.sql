DROP MATERIALIZED VIEW IF EXISTS mv_spells;

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

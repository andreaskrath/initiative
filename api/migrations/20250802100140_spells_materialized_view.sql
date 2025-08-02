CREATE MATERIALIZED VIEW mv_spells AS
SELECT
    spells.*,
    ARRAY (
        SELECT class
        FROM spell_classes
        WHERE spell_id = spells.id
        ORDER BY class
    ) as classes
FROM spells;

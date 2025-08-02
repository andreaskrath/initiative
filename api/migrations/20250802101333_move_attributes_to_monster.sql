DROP TABLE IF EXISTS monster_attributes;

ALTER TABLE monsters
    ADD COLUMN IF NOT EXISTS strength smallint NOT NULL,
    ADD COLUMN IF NOT EXISTS dexterity smallint NOT NULL,
    ADD COLUMN IF NOT EXISTS constitution smallint NOT NULL,
    ADD COLUMN IF NOT EXISTS intelligence smallint NOT NULL,
    ADD COLUMN IF NOT EXISTS wisdom smallint NOT NULL,
    ADD COLUMN IF NOT EXISTS charisma smallint NOT NULL;

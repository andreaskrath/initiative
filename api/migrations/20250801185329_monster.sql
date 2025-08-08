CREATE TABLE IF NOT EXISTS monsters (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    challenge_rating real NOT NULL,
    xp smallint NOT NULL,
    proficiency_bonus smallint NOT NULL,
    size size NOT NULL,
    monster_type monster_type NOT NULL,
    species text,
    alignment alignment NOT NULL,
    strength smallint NOT NULL,
    dexterity smallint NOT NULL,
    constitution smallint NOT NULL,
    intelligence smallint NOT NULL,
    wisdom smallint NOT NULL,
    charisma smallint NOT NULL,
    hit_points smallint NOT NULL,
    rollable_hit_points text NOT NULL,
    armor_class smallint NOT NULL,
    armor_type text NOT NULL,
    passive_perception smallint NOT NULL,
    available_legendary_actions smallint
);

CREATE INDEX IF NOT EXISTS idx_monsters_challenge_rating ON monsters(challenge_rating);
CREATE INDEX IF NOT EXISTS idx_monsters_size ON monsters(size);
CREATE INDEX IF NOT EXISTS idx_monsters_monster_type ON monsters(monster_type);

CREATE TABLE IF NOT EXISTS monster_spellcasting (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    level smallint NOT NULL,
    attribute attribute NOT NULL,
    dc smallint NOT NULL,
    attack_bonus smallint NOT NULL,
    PRIMARY KEY (monster_id)
);

CREATE TABLE IF NOT EXISTS monster_spell_slots (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    level spell_level NOT NULL,
    slots smallint NOT NULL,
    PRIMARY KEY (monster_id, level)
);

CREATE TABLE IF NOT EXISTS monster_spells (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    spell_id uuid NOT NULL REFERENCES spells(id) ON DELETE CASCADE,
    PRIMARY KEY (monster_id, spell_id)
);

CREATE TABLE IF NOT EXISTS monster_saving_throws (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    attribute attribute NOT NULL,
    modifier smallint NOT NULL,
    PRIMARY KEY (monster_id, attribute)
);

CREATE TABLE IF NOT EXISTS monster_damage_resistances (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    damage_type damage_type NOT NULL,
    PRIMARY KEY (monster_id, damage_type)
);

CREATE TABLE IF NOT EXISTS monster_damage_immunities (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    damage_type damage_type NOT NULL,
    PRIMARY KEY (monster_id, damage_type)
);

CREATE TABLE IF NOT EXISTS monster_condition_immunities (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    condition condition NOT NULL,
    PRIMARY KEY (monster_id, condition)
);

CREATE TABLE IF NOT EXISTS monster_visions (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    sight sight NOT NULL,
    range smallint NOT NULL,
    PRIMARY KEY (monster_id, sight)
);

CREATE TABLE IF NOT EXISTS monster_speeds (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    movement movement NOT NULL,
    distance smallint NOT NULL,
    PRIMARY KEY (monster_id, movement)
);

CREATE TABLE IF NOT EXISTS monster_languages (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    language language,
    PRIMARY KEY (monster_id, language)
);

CREATE TABLE IF NOT EXISTS monster_skills (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    skill skill NOT NULL,
    modifier smallint NOT NULL,
    PRIMARY KEY (monster_id, skill)
);

CREATE TABLE IF NOT EXISTS monster_traits (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (monster_id, name)
);

CREATE TABLE IF NOT EXISTS monster_regular_actions (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (monster_id, name)
);

CREATE TABLE IF NOT EXISTS monster_melee_attacks (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    hit_bonus smallint NOT NULL,
    reach smallint NOT NULL,
    one_handed_attack text,
    two_handed_attack text,
    damage_type damage_type NOT NULL,
    PRIMARY KEY (monster_id, name),
    CHECK (one_handed_attack IS NOT NULL OR two_handed_attack IS NOT NULL)
);

CREATE TABLE IF NOT EXISTS monster_ranged_attacks (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    hit_bonus smallint NOT NULL,
    normal_range smallint NOT NULL,
    long_range smallint NOT NULL,
    attack text NOT NULL,
    damage_type damage_type NOT NULL,
    PRIMARY KEY (monster_id, name)
);

CREATE TABLE IF NOT EXISTS monster_recharge_actions (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    recharge_dice text NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (monster_id, name)
);

CREATE TABLE IF NOT EXISTS monster_bonus_actions (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (monster_id, name)
);

CREATE TABLE IF NOT EXISTS monster_reactions (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (monster_id, name)
);

CREATE TABLE IF NOT EXISTS monster_legendary_actions (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    cost smallint NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (monster_id, name)
);

CREATE TABLE IF NOT EXISTS monster_lair_actions (
    monster_id uuid NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    name text NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (monster_id, name)
);

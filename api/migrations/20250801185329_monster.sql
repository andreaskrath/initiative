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
    armor_type text,
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
    recharge text NOT NULL,
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

CREATE OR REPLACE VIEW v_monsters AS
SELECT
    monsters.*,
    -- Bonus Actions
    (
        SELECT json_agg(json_build_object('name', monster_bonus_actions.name, 'description', monster_bonus_actions.description))
        FROM monster_bonus_actions
        WHERE monster_bonus_actions.monster_id = monsters.id
    ) AS bonus_actions,
    -- Condition Immunities
    ARRAY(
        SELECT monster_condition_immunities.condition
        FROM monster_condition_immunities
        WHERE monster_condition_immunities.monster_id = monsters.id
        ORDER BY monster_condition_immunities.condition) AS condition_immunities,
    -- Damage Immunities
    ARRAY(
        SELECT monster_damage_immunities.damage_type
        FROM monster_damage_immunities
        WHERE monster_damage_immunities.monster_id = monsters.id
        ORDER BY monster_damage_immunities.damage_type) AS damage_immunities,
    -- Damage Resistances
    ARRAY(
        SELECT monster_damage_resistances.damage_type
        FROM monster_damage_resistances
        WHERE monster_damage_resistances.monster_id = monsters.id
        ORDER BY monster_damage_resistances.damage_type) AS damage_resistances,
    -- Lair Actions
    (
        SELECT json_agg(json_build_object('name', monster_lair_actions.name, 'description', monster_lair_actions.description))
        FROM monster_lair_actions
        WHERE monster_lair_actions.monster_id = monsters.id
    ) AS lair_actions,
    -- Languages
    ARRAY(
        SELECT monster_languages.language
        FROM monster_languages
        WHERE monster_languages.monster_id = monsters.id
        ORDER BY monster_languages.language) AS languages,
    -- Legendary Actions
    (
        SELECT json_agg(json_build_object(
                'name', monster_legendary_actions.name,
                'cost', monster_legendary_actions.cost,
                'description', monster_legendary_actions.description
        )) FROM monster_legendary_actions
        WHERE monster_legendary_actions.monster_id = monsters.id
    ) AS legendary_actions,
    -- Melee Attacks
    (
        SELECT json_agg(json_build_object(
                'name', monster_melee_attacks.name,
                'hit_bonus', monster_melee_attacks.hit_bonus,
                'reach', monster_melee_attacks.reach,
                'one_handed_attack', monster_melee_attacks.one_handed_attack,
                'two_handed_attack', monster_melee_attacks.two_handed_attack,
                'damage_type', monster_melee_attacks.damage_type
        )) FROM monster_melee_attacks
        WHERE monster_melee_attacks.monster_id = monsters.id
    ) AS melee_attack_actions,
    -- Ranged Attacks
    (
        SELECT json_agg(json_build_object(
                'name', monster_ranged_attacks.name,
                'hit_bonus', monster_ranged_attacks.hit_bonus,
                'normal_range', monster_ranged_attacks.normal_range,
                'long_range', monster_ranged_attacks.long_range,
                'attack', monster_ranged_attacks.attack,
                'damage_type', monster_ranged_attacks.damage_type
        )) FROM monster_ranged_attacks
        WHERE monster_ranged_attacks.monster_id = monsters.id
    ) AS ranged_attack_actions,
    -- Reactions
    (
        SELECT json_agg(json_build_object('name', monster_reactions.name, 'description', monster_reactions.description))
        FROM monster_reactions
        WHERE monster_reactions.monster_id = monsters.id
    ) AS reactions,
    -- Recharge Actions
    (
        SELECT json_agg(json_build_object('name', monster_recharge_actions.name, 'recharge', monster_recharge_actions.recharge,'description', monster_recharge_actions.description))
        FROM monster_recharge_actions
        WHERE monster_recharge_actions.monster_id = monsters.id
    ) AS recharge_actions,
    -- Regular Actions
    (
        SELECT json_agg(json_build_object('name', monster_regular_actions.name, 'description', monster_regular_actions.description))
        FROM monster_regular_actions
        WHERE monster_regular_actions.monster_id = monsters.id
    ) AS regular_actions,
    -- Saving Throws
    (
        SELECT json_agg(json_build_object('attribute', monster_saving_throws.attribute, 'modifier', monster_saving_throws.modifier))
        FROM monster_saving_throws
        WHERE monster_saving_throws.monster_id = monsters.id
    ) AS saving_throws,
    -- Skills
    (
        SELECT json_agg(json_build_object('skill', monster_skills.skill, 'modifier', monster_skills.modifier))
        FROM monster_skills
        WHERE monster_skills.monster_id = monsters.id
    ) AS skills,
    -- Speeds
    (
        SELECT json_agg(json_build_object('movement', monster_speeds.movement, 'distance', monster_speeds.distance))
        FROM monster_speeds
        WHERE monster_speeds.monster_id = monsters.id
    ) AS speeds,
    -- Spell Slots
    (
        SELECT json_agg(json_build_object(
                'level', monster_spell_slots.level,
                'slots', monster_spell_slots.slots
        )) FROM monster_spell_slots
        WHERE monster_spell_slots.monster_id = monsters.id
    ) AS spell_slots,
    -- Spellcasting
    COALESCE(
        (
            SELECT json_agg(json_build_object(
                    'level', monster_spellcasting.level,
                    'attribute', monster_spellcasting.attribute,
                    'dc', monster_spellcasting.dc,
                    'attack_bonus', monster_spellcasting.attack_bonus
            )) FROM monster_spellcasting
            WHERE monster_spellcasting.monster_id = monsters.id
        ),
        'null'::json
    ) AS spellcasting,
    -- Spells
    (
        SELECT json_agg(to_json(v_spells.*))
        FROM v_spells
        JOIN monster_spells ON v_spells.id = monster_spells.spell_id
        WHERE monster_spells.monster_id = monsters.id
    ) AS spells,
    -- Traits
    (
        SELECT json_agg(json_build_object('name', monster_traits.name, 'description', monster_traits.description))
        FROM monster_traits
        WHERE monster_traits.monster_id = monsters.id
    ) AS traits,
    -- Visions
    (
        SELECT json_agg(json_build_object('sight', monster_visions.sight, 'range', monster_visions.range))
        FROM monster_visions
        WHERE monster_visions.monster_id = monsters.id
    ) AS visions
FROM monsters;

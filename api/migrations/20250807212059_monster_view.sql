CREATE OR REPLACE VIEW v_monsters AS
select
    monsters.*,
    -- bonus actions
    (
        select json_agg(json_build_object('name', monster_bonus_actions.name, 'description', monster_bonus_actions.description))
        from monster_bonus_actions
        where monster_bonus_actions.monster_id = monsters.id
    ) as bonus_actions,
    -- condition immunities
    ARRAY(
        select monster_condition_immunities.condition
        from monster_condition_immunities
        where monster_condition_immunities.monster_id = monsters.id
        order by monster_condition_immunities.condition) AS condition_immunities,
    -- damage immunities
    ARRAY(
        select monster_damage_immunities.damage_type
        from monster_damage_immunities
        where monster_damage_immunities.monster_id = monsters.id
        order by monster_damage_immunities.damage_type) AS damage_immunities,
    -- damage resistances
    ARRAY(
        select monster_damage_resistances.damage_type
        from monster_damage_resistances
        where monster_damage_resistances.monster_id = monsters.id
        order by monster_damage_resistances.damage_type) AS damage_resistances,
    -- lair actions
    (
        select json_agg(json_build_object('name', monster_lair_actions.name, 'description', monster_lair_actions.description))
        from monster_lair_actions
        where monster_lair_actions.monster_id = monsters.id
    ) as lair_actions,
    -- languages
    ARRAY(
        select monster_languages.language
        from monster_languages
        where monster_languages.monster_id = monsters.id
        order by monster_languages.language) AS languages,
    -- legendary actions
    (
        select json_agg(json_build_object(
                'name', monster_legendary_actions.name,
                'cost', monster_legendary_actions.cost,
                'description', monster_legendary_actions.description
        )) from monster_legendary_actions
        where monster_legendary_actions.monster_id = monsters.id
    ) as legendary_actions,
    -- melee attacks
    (
        select json_agg(json_build_object(
                'name', monster_melee_attacks.name,
                'hit_bonus', monster_melee_attacks.hit_bonus,
                'reach', monster_melee_attacks.reach,
                'one_handed_attack', monster_melee_attacks.one_handed_attack,
                'two_handed_attack', monster_melee_attacks.two_handed_attack,
                'damage_type', monster_melee_attacks.damage_type
        )) from monster_melee_attacks
        where monster_melee_attacks.monster_id = monsters.id
    ) as melee_attack_actions,
    -- ranged attacks
    (
        select json_agg(json_build_object(
                'name', monster_ranged_attacks.name,
                'hit_bonus', monster_ranged_attacks.hit_bonus,
                'normal_range', monster_ranged_attacks.normal_range,
                'long_range', monster_ranged_attacks.long_range,
                'attack', monster_ranged_attacks.attack,
                'damage_type', monster_ranged_attacks.damage_type
        )) from monster_ranged_attacks
        where monster_ranged_attacks.monster_id = monsters.id
    ) as ranged_attack_actions,
    -- reactions
    (
        select json_agg(json_build_object('name', monster_reactions.name, 'description', monster_reactions.description))
        from monster_reactions
        where monster_reactions.monster_id = monsters.id
    ) as reactions,
    -- recharge actions
    (
        select json_agg(json_build_object('name', monster_recharge_actions.name, 'recharge', monster_recharge_actions.recharge_dice ,'description', monster_recharge_actions.description))
        from monster_recharge_actions
        where monster_recharge_actions.monster_id = monsters.id
    ) as recharge_actions,
    -- regular actions
    (
        select json_agg(json_build_object('name', monster_regular_actions.name, 'description', monster_regular_actions.description))
        from monster_regular_actions
        where monster_regular_actions.monster_id = monsters.id
    ) as regular_actions,
    -- saving throws
    (
        select json_agg(json_build_object('attribute', monster_saving_throws.attribute, 'modifier', monster_saving_throws.modifier))
        from monster_saving_throws
        where monster_saving_throws.monster_id = monsters.id
    ) as saving_throws,
    -- skills
    (
        select json_agg(json_build_object('skill', monster_skills.skill, 'modifier', monster_skills.modifier))
        from monster_skills
        where monster_skills.monster_id = monsters.id
    ) as skills,
    -- speeds
    (
        select json_agg(json_build_object('movement', monster_speeds.movement_type, 'distance', monster_speeds.speed))
        from monster_speeds
        where monster_speeds.monster_id = monsters.id
    ) as speeds,
    -- spell slots
    (
        SELECT json_agg(json_build_object(
                'level', monster_spell_slots.level,
                'slots', monster_spell_slots.slots
        )) FROM monster_spell_slots
        WHERE monster_spell_slots.monster_id = monsters.id
    ) as spell_slots,
    -- spellcasting
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
    ) as spellcasting,
    -- spells
    (
        SELECT json_agg(to_json(v_spells.*))
        FROM v_spells
        JOIN monster_spells ON v_spells.id = monster_spells.spell_id
        WHERE monster_spells.monster_id = monsters.id
    ) as spells,
    -- traits
    (
        select json_agg(json_build_object('name', monster_traits.name, 'description', monster_traits.description))
        from monster_traits
        where monster_traits.monster_id = monsters.id
    ) as traits,
    -- visions
    (
        select json_agg(json_build_object('sight', monster_visions.sight, 'range', monster_visions.range))
        from monster_visions
        where monster_visions.monster_id = monsters.id
    ) as visions
from monsters;

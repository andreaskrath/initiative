use crate::types::Monster;
use sqlx::{PgPool, QueryBuilder, query_as};
use uuid::Uuid;

pub struct MonsterRepository {
    pool: PgPool,
}

impl MonsterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, monster: Monster) -> Result<Uuid, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        let monster_id: sqlx::types::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO MONSTERS (
                name,
                challenge_rating,
                xp,
                proficiency_bonus,
                size,
                monster_type,
                species,
                alignment,
                strength,
                dexterity,
                constitution,
                intelligence,
                wisdom,
                charisma,
                hit_points,
                rollable_hit_points,
                armor_class,
                armor_type,
                passive_perception,
                available_legendary_actions
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20)
            RETURNING id;
            "#,
        )
        .bind(monster.name)
        .bind(monster.challenge_rating)
        .bind(monster.xp)
        .bind(monster.proficiency_bonus)
        .bind(monster.size)
        .bind(monster.monster_type)
        .bind(monster.species)
        .bind(monster.alignment)
        .bind(monster.strength)
        .bind(monster.dexterity)
        .bind(monster.constitution)
        .bind(monster.intelligence)
        .bind(monster.wisdom)
        .bind(monster.charisma)
        .bind(monster.hit_points)
        .bind(monster.rollable_hit_points)
        .bind(monster.armor_class)
        .bind(monster.armor_type)
        .bind(monster.passive_perception)
        .bind(monster.available_legendary_actions)
        .fetch_one(&mut *transaction)
        .await?;

        if !monster.bonus_actions.is_empty() {
            let mut bonus_actions = Vec::with_capacity(monster.bonus_actions.len());
            for bonus_action in monster.bonus_actions {
                bonus_actions.push((monster_id, bonus_action));
            }

            QueryBuilder::new("INSERT INTO monster_bonus_actions (monster_id, name, description)")
                .push_values(bonus_actions, |mut b, (monster_id, bonus_action)| {
                    b.push_bind(monster_id)
                        .push_bind(bonus_action.name)
                        .push_bind(bonus_action.description);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.condition_immunities.is_empty() {
            let mut condition_immunities = Vec::with_capacity(monster.condition_immunities.len());
            for condition_immunity in monster.condition_immunities {
                condition_immunities.push((monster_id, condition_immunity));
            }

            QueryBuilder::new("INSERT INTO monster_condition_immunities (monster_id, condition)")
                .push_values(condition_immunities, |mut b, (monster_id, condition)| {
                    b.push_bind(monster_id).push_bind(condition);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.damage_immunities.is_empty() {
            let mut damage_immunities = Vec::with_capacity(monster.damage_immunities.len());
            for damage_type in monster.damage_immunities {
                damage_immunities.push((monster_id, damage_type));
            }

            QueryBuilder::new("INSERT INTO monster_damage_immunities (monster_id, damage_type)")
                .push_values(damage_immunities, |mut b, (monster_id, damage_type)| {
                    b.push_bind(monster_id).push_bind(damage_type);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.damage_resistances.is_empty() {
            let mut damage_resistances = Vec::with_capacity(monster.damage_resistances.len());
            for damage_type in monster.damage_resistances {
                damage_resistances.push((monster_id, damage_type));
            }

            QueryBuilder::new("INSERT INTO monster_damage_resistances (monster_id, damage_type)")
                .push_values(damage_resistances, |mut b, (monster_id, damage_type)| {
                    b.push_bind(monster_id).push_bind(damage_type);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.lair_actions.is_empty() {
            let mut lair_actions = Vec::with_capacity(monster.lair_actions.len());
            for action in monster.lair_actions {
                lair_actions.push((monster_id, action));
            }

            QueryBuilder::new("INSERT INTO monster_lair_actions (monster_id, name, description)")
                .push_values(lair_actions, |mut b, (monster_id, action)| {
                    b.push_bind(monster_id)
                        .push_bind(action.name)
                        .push_bind(action.description);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.languages.is_empty() {
            let mut languages = Vec::with_capacity(monster.languages.len());
            for language in monster.languages {
                languages.push((monster_id, language));
            }

            QueryBuilder::new("INSERT INTO monster_languages (monster_id, language)")
                .push_values(languages, |mut b, (monster_id, language)| {
                    b.push_bind(monster_id).push_bind(language);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.legendary_actions.is_empty() {
            let mut legendary_actions = Vec::with_capacity(monster.legendary_actions.len());
            for action in monster.legendary_actions {
                legendary_actions.push((monster_id, action));
            }

            QueryBuilder::new(
                "INSERT INTO monster_legendary_actions (monster_id, name, cost, description)",
            )
            .push_values(legendary_actions, |mut b, (monster_id, action)| {
                b.push_bind(monster_id)
                    .push_bind(action.name)
                    .push_bind(action.cost)
                    .push_bind(action.description);
            })
            .build()
            .execute(&mut *transaction)
            .await?;
        }

        if !monster.melee_attack_actions.is_empty() {
            let mut melee_attacks = Vec::with_capacity(monster.melee_attack_actions.len());
            for attack in monster.melee_attack_actions {
                melee_attacks.push((monster_id, attack));
            }

            QueryBuilder::new("INSERT INTO monster_melee_attacks (monster_id, name, hit_bonus, reach, one_handed_attack, two_handed_attack, damage_type)")
                .push_values(melee_attacks, |mut b, (monster_id, attack)| {
                    b.push_bind(monster_id)
                        .push_bind(attack.name)
                        .push_bind(attack.hit_bonus)
                        .push_bind(attack.reach)
                        .push_bind(attack.one_handed_attack)
                        .push_bind(attack.two_handed_attack)
                        .push_bind(attack.damage_type);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.ranged_attack_actions.is_empty() {
            let mut ranged_attacks = Vec::with_capacity(monster.ranged_attack_actions.len());
            for attack in monster.ranged_attack_actions {
                ranged_attacks.push((monster_id, attack));
            }

            QueryBuilder::new("INSERT INTO monster_ranged_attacks (monster_id, name, hit_bonus, normal_range, long_range, attack, damage_type)")
                .push_values(ranged_attacks, |mut b, (monster_id, attack)| {
                    b.push_bind(monster_id)
                        .push_bind(attack.name)
                        .push_bind(attack.hit_bonus)
                        .push_bind(attack.normal_range)
                        .push_bind(attack.long_range)
                        .push_bind(attack.attack)
                        .push_bind(attack.damage_type);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.reactions.is_empty() {
            let mut reactions = Vec::with_capacity(monster.reactions.len());
            for reaction in monster.reactions {
                reactions.push((monster_id, reaction));
            }

            QueryBuilder::new("INSERT INTO monster_reactions (monster_id, name, description)")
                .push_values(reactions, |mut b, (monster_id, reaction)| {
                    b.push_bind(monster_id)
                        .push_bind(reaction.name)
                        .push_bind(reaction.description);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.recharge_actions.is_empty() {
            let mut recharge_actions = Vec::with_capacity(monster.recharge_actions.len());
            for action in monster.recharge_actions {
                recharge_actions.push((monster_id, action));
            }

            QueryBuilder::new(
                "INSERT INTO monster_recharge_actions (monster_id, name, recharge, description)",
            )
            .push_values(recharge_actions, |mut b, (monster_id, action)| {
                b.push_bind(monster_id)
                    .push_bind(action.name)
                    .push_bind(action.recharge)
                    .push_bind(action.description);
            })
            .build()
            .execute(&mut *transaction)
            .await?;
        }

        if !monster.saving_throws.is_empty() {
            let mut saving_throws = Vec::with_capacity(monster.saving_throws.len());
            for throw in monster.saving_throws {
                saving_throws.push((monster_id, throw));
            }

            QueryBuilder::new(
                "INSERT INTO monster_saving_throws (monster_id, attribute, modifier)",
            )
            .push_values(saving_throws, |mut b, (monster_id, throw)| {
                b.push_bind(monster_id)
                    .push_bind(throw.attribute)
                    .push_bind(throw.modifier);
            })
            .build()
            .execute(&mut *transaction)
            .await?;
        }

        if !monster.skills.is_empty() {
            let mut skills = Vec::with_capacity(monster.skills.len());
            for skill in monster.skills {
                skills.push((monster_id, skill));
            }

            QueryBuilder::new("INSERT INTO monster_skills (monster_id, skill, modifier)")
                .push_values(skills, |mut b, (monster_id, skill)| {
                    b.push_bind(monster_id)
                        .push_bind(skill.skill)
                        .push_bind(skill.modifier);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.speeds.is_empty() {
            let mut speeds = Vec::with_capacity(monster.speeds.len());
            for speed in monster.speeds {
                speeds.push((monster_id, speed));
            }

            QueryBuilder::new("INSERT INTO monster_speeds (monster_id, movement, distance)")
                .push_values(speeds, |mut b, (monster_id, speed)| {
                    b.push_bind(monster_id)
                        .push_bind(speed.movement)
                        .push_bind(speed.distance);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if let Some(spellcasting) = monster.spellcasting
            && !spellcasting.spell_slots.is_empty()
        {
            let mut spell_slots = Vec::with_capacity(spellcasting.spell_slots.len());
            for slot in spellcasting.spell_slots {
                spell_slots.push((monster_id, slot));
            }

            QueryBuilder::new("INSERT INTO monster_spell_slots (monster_id, level, slots)")
                .push_values(spell_slots, |mut b, (monster_id, slot)| {
                    b.push_bind(monster_id)
                        .push_bind(slot.level)
                        .push_bind(slot.slots);
                })
                .build()
                .execute(&mut *transaction)
                .await?;

            let mut spells = Vec::with_capacity(spellcasting.spells.len());
            for spell_id in spellcasting.spells {
                spells.push((monster_id, spell_id));
            }

            QueryBuilder::new("INSERT INTO monster_spells (monster_id, spell_id)")
                .push_values(spells, |mut b, (monster_id, spell_id)| {
                    b.push_bind(monster_id).push_bind(spell_id.id);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.traits.is_empty() {
            let mut traits = Vec::with_capacity(monster.traits.len());
            for trait_item in monster.traits {
                traits.push((monster_id, trait_item));
            }

            QueryBuilder::new("INSERT INTO monster_traits (monster_id, name, description)")
                .push_values(traits, |mut b, (monster_id, trait_item)| {
                    b.push_bind(monster_id)
                        .push_bind(trait_item.name)
                        .push_bind(trait_item.description);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        if !monster.visions.is_empty() {
            let mut visions = Vec::with_capacity(monster.visions.len());
            for vision in monster.visions {
                visions.push((monster_id, vision));
            }

            QueryBuilder::new("INSERT INTO monster_visions (monster_id, sight, range)")
                .push_values(visions, |mut b, (monster_id, vision)| {
                    b.push_bind(monster_id)
                        .push_bind(vision.sight)
                        .push_bind(vision.range);
                })
                .build()
                .execute(&mut *transaction)
                .await?;
        }

        transaction.commit().await?;

        Ok(monster_id)
    }

    pub async fn get_all(&self) -> Result<Box<[Monster]>, sqlx::Error> {
        let monsters: Vec<Monster> = query_as("SELECT * FROM v_monsters;")
            .fetch_all(&self.pool)
            .await?;

        Ok(monsters.into_boxed_slice())
    }
}

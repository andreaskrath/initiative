use crate::types::Monster;
use sqlx::{PgPool, query_as, types::Json};
use uuid::Uuid;

pub struct MonsterRepository {
    pool: PgPool,
}

impl MonsterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, monster: Monster) -> Result<Uuid, sqlx::Error> {
        let monster_id: sqlx::types::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO monsters (
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
                available_legendary_actions,
                bonus_actions,
                condition_immunities,
                damage_immunities,
                damage_resistances,
                lair_actions,
                languages,
                legendary_actions,
                melee_attack_actions,
                ranged_attack_actions,
                reactions,
                recharge_actions,
                regular_actions,
                saving_throws,
                skills,
                speeds,
                traits,
                visions,
                spellcasting
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35,$36,$37,$38)
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
                .bind(Json(monster.bonus_actions))
                .bind(monster.condition_immunities)
                .bind(monster.damage_immunities)
                .bind(monster.damage_resistances)
                .bind(Json(monster.lair_actions))
                .bind(monster.languages)
                .bind(Json(monster.legendary_actions))
                .bind(Json(monster.melee_attack_actions))
                .bind(Json(monster.ranged_attack_actions))
                .bind(Json(monster.reactions))
                .bind(Json(monster.recharge_actions))
                .bind(Json(monster.regular_actions))
                .bind(Json(monster.saving_throws))
                .bind(Json(monster.skills))
                .bind(Json(monster.speeds))
                .bind(Json(monster.traits))
                .bind(Json(monster.visions))
                .bind(Json(monster.spellcasting))
        .fetch_one(&self.pool)
        .await?;

        Ok(monster_id)
    }

    pub async fn get_all(&self) -> Result<Box<[Monster]>, sqlx::Error> {
        let monsters: Vec<Monster> = query_as("SELECT * FROM monsters;")
            .fetch_all(&self.pool)
            .await?;

        Ok(monsters.into_boxed_slice())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Monster, sqlx::Error> {
        let monster: Monster = query_as("SELECT * FROM monsters WHERE id = $1;")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(monster)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM monsters WHERE id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

use sqlx::{PgPool, QueryBuilder, query_as};
use uuid::Uuid;

use super::types::Spell;

pub struct SpellRepository {
    pool: PgPool,
}

impl SpellRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, spell: Spell) -> Result<Uuid, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        let spell_id: sqlx::types::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO spells (
                name,
                school,
                level,
                verbal,
                somatic,
                material,
                material_consumed,
                ritual,
                concentration,
                casting_time,
                duration,
                range,
                area,
                shape,
                description,
                at_higher_levels
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING id;
            "#,
        )
        .bind(spell.name)
        .bind(spell.school)
        .bind(spell.level)
        .bind(spell.verbal)
        .bind(spell.somatic)
        .bind(spell.material)
        .bind(spell.material_consumed)
        .bind(spell.ritual)
        .bind(spell.concentration)
        .bind(spell.casting_time)
        .bind(spell.duration)
        .bind(spell.range)
        .bind(spell.area)
        .bind(spell.shape)
        .bind(spell.description)
        .bind(spell.at_higher_levels)
        .fetch_one(&mut *transaction)
        .await?;

        let mut relations = Vec::with_capacity(spell.classes.len());
        for class in spell.classes {
            relations.push((spell_id, class));
        }

        let mut class_restriction_query =
            QueryBuilder::new("INSERT INTO spell_classes (spell_id, class)");

        class_restriction_query.push_values(relations, |mut b, (spell_id, class)| {
            b.push_bind(spell_id).push_bind(class);
        });

        class_restriction_query
            .build()
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(spell_id)
    }

    pub async fn get_all(&self) -> Result<Box<[Spell]>, sqlx::Error> {
        let spells: Vec<Spell> = query_as("SELECT * FROM mv_spells;")
            .fetch_all(&self.pool)
            .await?;

        Ok(spells.into_boxed_slice())
    }
}

use sqlx::{PgPool, QueryBuilder, query_as};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::types::Spell;

pub struct SpellRepository {
    pool: PgPool,
}

impl SpellRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, spell), fields(spell_name = %spell.name))]
    pub async fn create(&self, spell: Spell) -> Result<Uuid, sqlx::Error> {
        debug!("Starting transaction to insert spell into database");
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

        debug!("Committing transaction");
        transaction.commit().await?;

        debug!("Successfully inserted spell into database with id: {}", spell_id);
        Ok(spell_id)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self) -> Result<Box<[Spell]>, sqlx::Error> {
        debug!("Querying all spells from database");
        let spells: Vec<Spell> = query_as("SELECT * FROM v_spells;")
            .fetch_all(&self.pool)
            .await?;

        debug!("Successfully retrieved {} spells from database", spells.len());
        Ok(spells.into_boxed_slice())
    }

    #[instrument(skip(self))]
    pub async fn get_by_id(&self, id: Uuid) -> Result<Spell, sqlx::Error> {
        debug!("Querying spell from database");
        let spell: Spell = query_as("SELECT * FROM v_spells WHERE id = $1;")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        debug!("Successfully retrieved spell: {}", spell.name);
        Ok(spell)
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        debug!("Deleting spell from database");
        sqlx::query("DELETE FROM spells WHERE id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await?;

        debug!("Successfully deleted spell from database");
        Ok(())
    }
}

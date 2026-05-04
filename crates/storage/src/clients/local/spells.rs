use crate::Error;
use crate::clients::local::Local;
use crate::models::spell::NewSpell;
use crate::repositories::spells::Spells;
use crate::repositories::spells::SpellsRepository;

use uuid::Uuid;

#[async_trait::async_trait]
impl Spells for Local {
    async fn create(&self, new_spell: NewSpell) -> Result<(), Error> {
        for image in &new_spell.images {
            let path = self.images_dir.clone().join(image.id.to_string());

            // Periodic maintenance will run and clean up any images left-over by failing in this step.
            //
            // This approach is simpler than having to in-line clean up at each point the database
            // transaction can fail.
            if let Err(err) = tokio::fs::write(path, &image.bytes).await {
                tracing::error!("failed to create image file: {err}");
                return Err(Error::NotFound);
            }
        }

        let Ok(shape) = serde_json::to_string(&new_spell.shape) else {
            tracing::error!("failed to JSON serialize spell shape");
            return Err(Error::Decode);
        };

        let Ok(materials) = serde_json::to_string(&new_spell.materials) else {
            tracing::error!("failed to JSON serialize spell materials");
            return Err(Error::Decode);
        };

        let Ok(mut transaction) = self.pool.begin().await else {
            tracing::error!("failed to begin transaction for creating spell");
            return Err(Error::Connection);
        };

        let insert_spell_query = r#"
            INSERT INTO spells
            (
                id,
                name,
                school,
                level,
                source,
                casting_time,
                ritual,
                concentration,
                verbal,
                somatic,
                material,
                materials,
                duration,
                range,
                area,
                shape,
                description,
                at_higher_levels,
                flavor_text,
                attribution
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20);
        "#;
        let insert_spell_result = sqlx::query(insert_spell_query)
            .bind(new_spell.id)
            .bind(new_spell.name)
            .bind(new_spell.school)
            .bind(new_spell.level)
            .bind(new_spell.source)
            .bind(new_spell.casting_time)
            .bind(new_spell.ritual)
            .bind(new_spell.concentration)
            .bind(new_spell.verbal)
            .bind(new_spell.somatic)
            .bind(new_spell.material)
            .bind(materials)
            .bind(new_spell.duration)
            .bind(new_spell.range)
            .bind(new_spell.area)
            .bind(shape)
            .bind(new_spell.description)
            .bind(new_spell.at_higher_levels)
            .bind(new_spell.flavor_text)
            .bind(new_spell.attribution)
            .execute(&mut *transaction)
            .await;

        if let Err(err) = insert_spell_result {
            tracing::error!("failed to insert spell in spells table: {err}");
            return Err(Error::Query);
        }

        for alias in new_spell.aliases {
            let insert_alias_query = r#"
                INSERT INTO spell_aliases (
                    spell_id, alias
                ) VALUES ($1,$2);
            "#;
            let insert_alias_result = sqlx::query(insert_alias_query)
                .bind(new_spell.id)
                .bind(alias)
                .execute(&mut *transaction)
                .await;

            if let Err(err) = insert_alias_result {
                tracing::error!("failed to insert alias in spell_aliases table: {err}");
                return Err(Error::Query);
            }
        }

        for class in new_spell.classes {
            let insert_class_query = r#"
                INSERT INTO spell_classes (
                    spell_id, class
                ) VALUES ($1,$2);
            "#;
            let insert_class_result = sqlx::query(insert_class_query)
                .bind(new_spell.id)
                .bind(class.to_string())
                .execute(&mut *transaction)
                .await;

            if let Err(err) = insert_class_result {
                tracing::error!("failed to insert class in spell_classes table: {err}");
                return Err(Error::Query);
            }
        }

        for tag in new_spell.tags {
            let initial_tag_id = Uuid::new_v4();

            // This query contains a no-op update to get Sqlite to return the id on a conflict.
            let insert_tag_query = r#"
                INSERT INTO tags (
                    id, value
                ) VALUES ($1,$2)
                ON CONFLICT (value)
                DO UPDATE SET value = value
                RETURNING id;
            "#;
            let insert_tag_result = sqlx::query_scalar(insert_tag_query)
                .bind(initial_tag_id)
                .bind(tag)
                .fetch_one(&mut *transaction)
                .await;
            let actual_tag_id: Uuid = match insert_tag_result {
                Ok(actual_tag_id) => actual_tag_id,
                Err(err) => {
                    tracing::error!("failed to insert tag in tags table: {err}");
                    return Err(Error::Query);
                }
            };

            let link_tag_query = r#"
                INSERT INTO spell_tags (
                    spell_id, tag_id
                ) VALUES ($1,$2);
            "#;
            let link_tag_result = sqlx::query(link_tag_query)
                .bind(new_spell.id)
                .bind(actual_tag_id)
                .execute(&mut *transaction)
                .await;
            if let Err(err) = link_tag_result {
                tracing::error!("failed to link tag in spell_tags table: {err}");
                return Err(Error::Query);
            }
        }

        for image in new_spell.images {
            let insert_image_query = r#"
                INSERT INTO spell_images (
                    id, spell_id
                ) VALUES ($1,$2);
            "#;
            let insert_image_result = sqlx::query(insert_image_query)
                .bind(image.id)
                .bind(new_spell.id)
                .execute(&mut *transaction)
                .await;

            if let Err(err) = insert_image_result {
                tracing::error!("failed to insert image in spell_images table: {err}");
                return Err(Error::Query);
            }
        }

        if let Err(err) = transaction.commit().await {
            tracing::error!("failed to commit spell creation transaction: {err}");
            return Err(Error::Connection);
        }

        Ok(())
    }
}

impl SpellsRepository for Local {
    fn spells(&self) -> &dyn Spells {
        self
    }
}

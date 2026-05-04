use crate::Error;
use crate::clients::local::Local;
use crate::repositories::options::Options;
use crate::repositories::options::OptionsRepository;
use crate::repositories::options::Variant;

impl OptionsRepository for Local {
    fn options(&self) -> &dyn Options {
        self
    }
}

#[async_trait::async_trait]
impl Options for Local {
    async fn list_options(&self, variant: Variant) -> Result<Box<[String]>, Error> {
        let query = r#"
            SELECT value
            FROM options
            WHERE variant = $1
            ORDER BY sort_order;
        "#;

        let options = sqlx::query_scalar(query)
            .bind(variant)
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!("fetched {} '{:?}' options", options.len(), variant);

        Ok(options.into_boxed_slice())
    }
}

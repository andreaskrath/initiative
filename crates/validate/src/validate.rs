mod error;
mod rules;

use gpui::SharedString;

use error::ValidationError;
use rules::Rule;

#[derive(Debug, Default)]
pub struct InputValidator {
    field: SharedString,
    rules: Vec<Rule>,
}

impl InputValidator {
    pub fn min(mut self, length: usize) -> Self {
        self.rules.push(Rule::Minimum(length));
        self
    }

    pub fn max(mut self, length: usize) -> Self {
        self.rules.push(Rule::Maximum(length));
        self
    }

    pub fn between(mut self, min: usize, max: usize) -> Self {
        self.rules.push(Rule::Between(min, max));
        self
    }

    pub fn validate(&self, input: SharedString) -> Result<(), ValidationError> {
        for rule in &self.rules {
            self.check_rule(rule, &input)?;
        }

        Ok(())
    }

    fn check_rule(&self, rule: &Rule, input: &SharedString) -> Result<(), ValidationError> {
        let length = input.len();

        match rule {
            Rule::Minimum(min) if length < *min => {
                Err(ValidationError::TooShort(self.field.clone(), *min))
            }
            Rule::Maximum(max) if length > *max => {
                Err(ValidationError::TooLong(self.field.clone(), *max))
            }
            Rule::Between(min, max) if length < *min || length > *max => {
                Err(ValidationError::Between(self.field.clone(), *min, *max))
            }
            Rule::Minimum(_) | Rule::Maximum(_) | Rule::Between(_, _) => Ok(()),
            rule => unreachable!("invalid rule found for InputValidator: {rule:?}"),
        }
    }
}

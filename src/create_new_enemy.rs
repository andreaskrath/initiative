use crate::{
    dice::{d10, d12, d20, d4, d6, d8},
    enemy::Enemy,
    message::Message,
};
use iced::{
    widget::{button, column, row, text, text_input},
    Color, Element, Task,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum CreateNewEnemyError {
    #[error("Could not be parse '{0}' as a valid initiative value or modifier.")]
    InvalidInitiative(String),

    #[error("Could not parse '{0}' as flat nor rollable hit point value.")]
    HitPointsFormat(String),

    #[error("Both '+' and '-' are defined in the hit points; use only one or neither.")]
    BonusHitPointsFormat,

    #[error("Could not parse '{0}' as a valid dice count value.")]
    DiceCount(String),

    #[error("Could not parse '{0}' as a valid bonus hit point value.")]
    BonusHitPointsValue(String),

    #[error("Could not parse '{0}' as a valid dice head value.")]
    DiceHead(String),
}

#[derive(Debug, Clone)]
pub enum Action {
    Name(String),
    InitiativeMod(String),
    HitPoints(String),
    Submit,
    SetError(CreateNewEnemyError),
}

pub struct CreateNewEnemy {
    name: String,
    initiative_mod: String,
    hp: String,
    error: Option<CreateNewEnemyError>,
}

impl CreateNewEnemy {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            initiative_mod: String::new(),
            hp: String::new(),
            error: None,
        }
    }

    pub fn update(&mut self, msg: Action) -> Task<Message> {
        match msg {
            Action::Name(s) => {
                self.name = s;

                Task::none()
            }
            Action::InitiativeMod(s) => {
                self.initiative_mod = s;

                Task::none()
            }
            Action::HitPoints(s) => {
                self.hp = s;

                Task::none()
            }
            Action::Submit => Task::perform(
                submit_new_enemy(
                    self.name.clone(),
                    self.initiative_mod.clone(),
                    self.hp.clone(),
                ),
                Message::SubmitNewEnemy,
            ),
            Action::SetError(error) => {
                self.error = Some(error);

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Action> {
        let name_label = text("Name:");
        let name_input = text_input("", &self.name).on_input(Action::Name);

        let initiative_label = text("Initiative Modifier:");
        let initiative_input = text_input("", &self.initiative_mod).on_input(Action::InitiativeMod);

        let hp_label = text("Hit Points:");
        let hp_input = text_input("", &self.hp).on_input(Action::HitPoints);

        let submit = button("Submit").on_press(Action::Submit);

        let error = match &self.error {
            Some(err) => text(err.to_string()).color(Color::from_rgb8(255, 0, 0)),
            None => text(""),
        };

        column![
            row![name_label, name_input],
            row![initiative_label, initiative_input],
            row![hp_label, hp_input],
            row![submit],
            row![error],
        ]
        .into()
    }
}

async fn submit_new_enemy(
    name: String,
    initiative: String,
    hp: String,
) -> Result<Enemy, CreateNewEnemyError> {
    let initiative = parse_initiative(initiative)?;
    let max_hp = parse_hit_points(hp)?;
    Ok(Enemy::new(name, initiative, max_hp))
}

fn parse_initiative(initiative_mod: String) -> Result<i8, CreateNewEnemyError> {
    let parsed = initiative_mod
        .parse::<i8>()
        .map_err(|_| CreateNewEnemyError::InvalidInitiative(initiative_mod.clone()))?;
    if initiative_mod
        .chars()
        .next()
        .is_some_and(|c| c == '-' || c == '+')
    {
        // Safe cast cause 1-20 range is always valid i8.
        Ok(d20(1) as i8 + parsed)
    } else {
        Ok(parsed)
    }
}

fn parse_hit_points(hp: String) -> Result<u16, CreateNewEnemyError> {
    // This handles the straight hit points being supplied.
    if let Ok(hit_points) = hp.parse::<u16>() {
        return Ok(hit_points);
    }

    // Everything below here handles hit point calculator being supplied.

    let (count, rest) = match hp.split_once('d') {
        Some((a, b)) => {
            let count_first = a.chars().next().is_some_and(|c| c.is_ascii_digit());
            let head_first = b.chars().next().is_some_and(|c| c.is_ascii_digit());

            match (count_first, head_first) {
                (true, true) => (a, b),

                // Neither of these scenarios are valid; but better error information is provided
                // further down in the function.
                (true, false) | (false, true) => (a, b),

                (false, false) => return Err(CreateNewEnemyError::HitPointsFormat(hp.clone())),
            }
        }
        None => return Err(CreateNewEnemyError::HitPointsFormat(hp.clone())),
    };

    let (dice, bonus) = match (rest.split_once('+'), rest.split_once('-')) {
        (Some(_), Some(_)) => return Err(CreateNewEnemyError::BonusHitPointsFormat),

        // There is no bonus defined.
        (None, None) => (rest, String::from("0")),

        // Bonus is negative.
        (None, Some((a, b))) => (a, String::from("-") + b),

        // Bonus is positive.
        (Some((a, b)), None) => (a, String::from(b)),
    };

    let count = count
        .parse()
        .map_err(|_| CreateNewEnemyError::DiceCount(count.to_string()))?;
    let bonus = bonus
        .parse::<i16>()
        .map_err(|_| CreateNewEnemyError::BonusHitPointsValue(bonus))?;

    let result = match dice.trim() {
        "4" => d4(count) + bonus,
        "6" => d6(count) + bonus,
        "8" => d8(count) + bonus,
        "10" => d10(count) + bonus,
        "12" => d12(count) + bonus,
        "20" => d20(count) + bonus,
        unknown => return Err(CreateNewEnemyError::DiceHead(unknown.to_string())),
    };

    // Ensures that cases like a rat (1d4 - 1) always end up being at least 1 hit points.
    Ok(result.max(1) as u16)
}

#[cfg(test)]
mod parse_initiative {
    use super::{parse_initiative, CreateNewEnemyError};

    #[test]
    fn plus_prefix() {
        let initiative = String::from("+2");
        let actual = parse_initiative(initiative);

        assert!(actual.is_ok());
    }

    #[test]
    fn minus_prefix() {
        let initiative = String::from("-2");
        let actual = parse_initiative(initiative);

        assert!(actual.is_ok());
    }

    #[test]
    fn no_prefix() {
        let initiative = String::from("2");
        let actual = parse_initiative(initiative);

        assert!(actual.is_ok());
    }

    #[test]
    fn invalid_format() -> Result<(), CreateNewEnemyError> {
        let initiative = String::from("2d4");
        let expected = Err(CreateNewEnemyError::InvalidInitiative(initiative.clone()));
        let actual = parse_initiative(initiative);

        assert_eq!(actual, expected);
        Ok(())
    }
}

#[cfg(test)]
mod parse_hit_points {
    use super::{parse_hit_points, CreateNewEnemyError};

    #[test]
    fn flat_value() -> Result<(), CreateNewEnemyError> {
        let hp = String::from("65");
        let expected = 65;
        let actual = parse_hit_points(hp)?;

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn rollable_value_without_bonus() -> Result<(), CreateNewEnemyError> {
        let hp = String::from("2d10");
        let lower = 2;
        let upper = 20;
        let actual = parse_hit_points(hp)?;

        assert!(lower <= actual && actual <= upper);
        Ok(())
    }

    #[test]
    fn rollable_value_with_bonus() -> Result<(), CreateNewEnemyError> {
        let hp = String::from("2d10+8");
        let lower = 10;
        let upper = 28;
        let actual = parse_hit_points(hp)?;

        assert!(lower <= actual && actual <= upper);
        Ok(())
    }

    #[test]
    fn hit_points_format_err() {
        let hp = String::from("Hello, world!");
        let expected = Err(CreateNewEnemyError::HitPointsFormat(hp.clone()));
        let actual = parse_hit_points(hp);

        assert_eq!(actual, expected);
    }

    #[test]
    fn bonus_hit_point_format_err() {
        let hp = String::from("2d10+-5");
        let expected = Err(CreateNewEnemyError::BonusHitPointsFormat);
        let actual = parse_hit_points(hp);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dice_count_err() {
        let hp = String::from("ad10-5");
        let expected = Err(CreateNewEnemyError::DiceCount(String::from("a")));
        let actual = parse_hit_points(hp);

        assert_eq!(actual, expected);
    }

    #[test]
    fn bonus_hit_point_value_err() {
        let hp = String::from("2d10-a");
        let expected = Err(CreateNewEnemyError::BonusHitPointsValue(String::from("-a")));
        let actual = parse_hit_points(hp);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dice_head_err() {
        let hp = String::from("2d13");
        let expected = Err(CreateNewEnemyError::DiceHead(String::from("13")));
        let actual = parse_hit_points(hp);

        assert_eq!(actual, expected);
    }
}

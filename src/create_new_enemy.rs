use crate::{
    dice::{d10, d12, d20, d4, d6, d8},
    enemy::Enemy,
    message::Message,
};
use iced::{
    widget::{button, column, row, text, text_input},
    Color, Element, Task,
};
use rand::Rng;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum CreateNewEnemyError {
    #[error("The provided initiative value could not be parsed to a valid value.")]
    InvalidInitiative,
    #[error("The provided hit points value could not be parsed to a valid valie.")]
    InvalidHitPoints,
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
        .map_err(|_| CreateNewEnemyError::InvalidInitiative)?;
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

    let (count, rest) = hp
        .split_once('d')
        .ok_or(CreateNewEnemyError::InvalidHitPoints)?;

    let (dice, bonus) = match (rest.split_once('+'), rest.split_once('-')) {
        (None, None) | (Some(_), Some(_)) => return Err(CreateNewEnemyError::InvalidHitPoints),

        // Bonus is negative.
        (None, Some((a, b))) => (a, String::from("-") + b),

        // Bonus is positive.
        (Some((a, b)), None) => (a, String::from(b)),
    };

    //let (dice, bonus) = rest
    //    .split_once('+')
    //    .ok_or(CreateNewEnemyError::InvalidHitPoints)?;

    let count = count
        .parse()
        .map_err(|_| CreateNewEnemyError::InvalidHitPoints)?;
    let bonus = bonus
        .parse::<i16>()
        .map_err(|_| CreateNewEnemyError::InvalidHitPoints)?;

    let result = match dice.trim() {
        "4" => d4(count) + bonus,
        "6" => d6(count) + bonus,
        "8" => d8(count) + bonus,
        "10" => d10(count) + bonus,
        "12" => d12(count) + bonus,
        "20" => d20(count) + bonus,
        _ => return Err(CreateNewEnemyError::InvalidHitPoints),
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
        let expected = Err(CreateNewEnemyError::InvalidInitiative);
        let actual = parse_initiative(initiative);

        assert_eq!(actual, expected);
        Ok(())
    }
}

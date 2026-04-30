use crate::view::spell::form::loader::LoadMessage;
use components::image_field::error::ImageError;
use storage::Error;
use types::Class;
use types::ShapeKind;

use iced::widget::text_editor::Action;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    LoadMessage(LoadMessage),
    NameChanged(String),
    SchoolSelected(String),
    LevelSelected(String),
    ClassToggled(Class),
    TagChanged(String),
    TagSubmitted,
    TagRemoved(usize),
    CastingTimeSelected(String),
    RitualToggled,
    ConcentrationToggled,
    VerbalToggled,
    SomaticToggled,
    MaterialToggled,
    MaterialChanged(usize, String),
    MaterialWorthChanged(usize, String),
    MaterialConsumed(usize),
    DurationSelected(String),
    RangeSelected(String),
    AreaSelected(String),
    ShapeKindSelected(ShapeKind),
    ShapeLengthChanged(String),
    ShapeRadiusChanged(String),
    ShapeHeightChanged(String),
    ShapeWidthChanged(String),
    DescriptionChanged(Action),
    AtHigherLevelsChanged(Action),
    FlavorTextChanged(Action),
    AttributionChanged(String),
    ImagePasted,
    ImageLoaded(Result<Box<[u8]>, ImageError>),
    ImageRemoved(usize),
    ImagePickerOpened,
    ImageFileSelected(Option<PathBuf>),
    ImageFileLoaded(Result<Box<[u8]>, ImageError>),
}

#[derive(Debug, Clone)]
pub enum Effect {
    LoadFailed(Error),
}

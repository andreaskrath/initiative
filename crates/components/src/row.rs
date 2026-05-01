use style::layout::BODY_SPACING;
use widgets::Element;

use iced::Length;
use iced::widget::Column;
use iced::widget::Row;

/// A chunked row that wraps with `per_row` elements.
pub fn chunked<'a, Message: 'a>(
    mut items: Vec<Element<'a, Message>>,
    per_row: usize,
) -> Element<'a, Message> {
    // Pad to a multiple of per_row so rows are uniform width.
    let padding = calculate_padding(items.len(), per_row);
    for _ in 0..padding {
        let space = iced::widget::space().width(Length::FillPortion(1)).into();
        items.push(space);
    }

    let mut column = Column::with_capacity(items.len() / per_row).spacing(BODY_SPACING);
    while !items.is_empty() {
        let chunk = items.drain(..per_row.min(items.len()));
        let mut row = Row::with_capacity(per_row).spacing(BODY_SPACING);
        for item in chunk {
            row = row.push(item);
        }

        column = column.push(row);
    }

    column.into()
}

/// Calculate the number of padding elements necessary for `current` to become a multiple of `target`.
fn calculate_padding(current: usize, target: usize) -> usize {
    let remainder = current % target;

    // This branch exists for when `current` is already a multiple of `target`.
    //
    // For example:
    // - `current` = 10
    // - `target` = 5
    // - `remainder` = 0
    //
    // Now `target` - `remainder` ends up just being `target` and over-padding.
    if remainder == 0 {
        0
    } else {
        target - remainder
    }
}

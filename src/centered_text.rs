use bevy::{prelude::*, text::Text2dSize};

pub fn centered_text_bundle<S: ToString>(
    message: S,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
    position: Vec3,
) -> CenteredTextBundle {
    let text_style = TextStyle {
        font,
        font_size,
        color,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    CenteredTextBundle {
        text: Text::with_section(message.to_string(), text_style, text_alignment),
        transform: Transform::from_xyz(position.x, position.y, position.z),
        text_2d_size: Default::default(),
        global_transform: Default::default(),
        visibility: Default::default(),
    }
}

#[derive(Bundle)]
pub struct CenteredTextBundle {
    pub text: Text,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub text_2d_size: Text2dSize,
    pub visibility: Visibility,
}

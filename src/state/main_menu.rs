use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Color, Commands,
        Query, Res, TextBundle, With,
    },
    text::TextStyle,
    ui::{AlignItems, Interaction, JustifyContent, Size, Style, UiColor, UiRect, Val},
};
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(TextBundle::from_section(
        "Sample Text",
        TextStyle {
            font: asset_server.load("fonts/roboto.ttf"),
            font_size: 30.0,
            color: Color::WHITE,
        },
    ));
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.55, 0.55, 0.55).into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.35, 0.35, 0.35).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

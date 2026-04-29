use bevy::prelude::*;

use crate::demo::loot::MoneyText;

#[derive(Component)]
pub struct Hud;

pub fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            Name::new("HUD Root"),
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            Hud,
        ))
        .with_children(|parent| {
            // Top bar
            parent
                .spawn(Node {
                    width: percent(100),
                    height: px(50.0),
                    padding: UiRect::all(px(10.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Money: 0"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        MoneyText,
                    ));
                });

            // Bottom hint / debug
            parent.spawn((
                Text::new("P / ESC: Pause"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::all(px(10.0)),
                    ..default()
                },
            ));
        });
}

pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<Hud>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

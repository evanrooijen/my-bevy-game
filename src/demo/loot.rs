use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    demo::player::{Player, PlayerAssets},
};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Coin;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct GivesMoney(u32);

#[derive(Resource)]
pub struct Money(pub u32);

#[derive(Component)]
pub struct MoneyText;

pub fn coin(
    x: f32,
    y: f32,
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Name::new("Coin"),
        Coin,
        GivesMoney(1),
        Sprite::from_atlas_image(
            player_assets.coin.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_scale(Vec2::splat(2.0).extend(1.0)).with_translation(Vec3::new(x, y, 0.0)),
        // Physics
        Collider::ball(8.0),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
    )
}

pub fn spawn_coin(
    window: Single<&Window, With<PrimaryWindow>>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let mut rng = rand::rng();

    let size = window.size();
    let half_x = size.x * 0.5;
    let half_y = size.y * 0.5;

    // TODO: replace with your actual level bounds
    let x = rng.random_range(-half_x..half_x);
    let y = rng.random_range(-half_y..half_y);

    commands.spawn(coin(x, y, &player_assets, &mut texture_atlases));
}

fn pickup_money_system(
    mut commands: Commands,
    mut money: ResMut<Money>,
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform, &GivesMoney), With<Coin>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.xy();
    let player_radius = 16.0 * 8.0; // sprite scale-aware (adjust if needed)

    for (coin_entity, coin_transform, gives_money) in &coin_query {
        let coin_pos = coin_transform.translation.xy();

        let distance = player_pos.distance(coin_pos);

        // simple overlap check
        if distance < player_radius {
            money.0 += gives_money.0;
            println!("Money: {}", money.0);

            commands.entity(coin_entity).despawn();
        }
    }
}

fn update_money_ui(money: Res<Money>, mut query: Query<&mut Text, With<MoneyText>>) {
    for mut text in &mut query {
        *text = Text::new(format!("Money: {}", money.0));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Money(0)).add_systems(
        Update,
        (update_money_ui, pickup_money_system)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

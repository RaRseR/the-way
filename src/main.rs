use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 700.0;

const SPRITE_SHEET_PATH: &str = "assets.png";
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 32;

pub const BG_COLOR: (f32, f32, f32) = (0.5, 0.5, 0.9);

const PLAYER_SPEED: f32 = 2.0;

#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    GameInit,
    InMenu,
    InGame
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::srgb(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .add_systems(OnEnter(AppState::Loading), load_assets)
        .add_systems(OnEnter(AppState::InGame), (setup_camera, init_world))
        .add_systems(Update, (handle_player_input).run_if(in_state(AppState::InGame)))
        .run();
}

fn load_assets(
    mut texture_atlas_handle: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut current_state: ResMut<NextState<AppState>>
) {
    image_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(UVec2::new(TILE_WIDTH, TILE_HEIGHT), 7, 1, None, None);
    texture_atlas_handle.0 = Some(texture_atlas_layouts.add(layout));

    current_state.set(AppState::InGame);
}

fn setup_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2d);
}

fn init_world (
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>
) {
    let animation_indices = AnimationIndices { first: 0, last: 6 };

    commands.spawn(
        (
            Sprite::from_atlas_image(
                image_handle.0.clone().unwrap(),
                TextureAtlas { 
                    layout: texture_atlas.0.clone().unwrap(), 
                    index: animation_indices.first
                },
            ),
            Transform::from_scale(Vec3::splat(6.0)),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Player
        )
    );
}

fn handle_player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut().unwrap();

    let w_key = keyboard_input.pressed(KeyCode::KeyW);
    let a_key = keyboard_input.pressed(KeyCode::KeyA);
    let s_key = keyboard_input.pressed(KeyCode::KeyS);
    let d_key = keyboard_input.pressed(KeyCode::KeyD);

    let mut delta = Vec2::ZERO;

    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }

    delta = delta.normalize();
    if delta.is_finite() {
        println!("{delta}");
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
    }
}
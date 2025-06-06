use bevy::{prelude::*, window::PrimaryWindow};
use crate::{state::AppState, BG_COLOR, SPRITE_SHEET_PATH, TILE_HEIGHT, TILE_WIDTH};

#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub image: Option<Handle<Image>>,
    pub layout: Option<Handle<TextureAtlasLayout>>
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            image: None,
            layout: None
        }
    }
}

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(
                BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
            )))
            .insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(AppState::Loading), load_assets)
            .add_systems(Update, (update_cursor_position).run_if(in_state(AppState::InGame)));
    }
}

fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut current_state: ResMut<NextState<AppState>>
) {
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(UVec2::new(TILE_WIDTH, TILE_HEIGHT), 20, 1, None, None);
    handle.layout = Some(texture_atlas_layouts.add(layout));

    current_state.set(AppState::GameInit);
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_position.0 = None;
        return;
    }

    let (camera, camera_transform) = camera_query.single().unwrap();

    let window = window_query.single().unwrap();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        cursor_position.0 = Some(world_position);
    }
}

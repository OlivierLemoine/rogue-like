mod animator;
mod collider;
mod dungeon;
mod monster;
mod player;
mod rigidbody;

use animator::*;
use bevy::{asset::LoadState, prelude::*};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use collider::*;
use monster::*;
use player::*;
use rigidbody::*;

fn main() {
    App::new()
        // Builtins
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // Resources
        .init_resource::<SpriteHandles>()
        .add_state(AppState::LoadingGameSprites)
        // Startup
        .add_system_set(
            SystemSet::on_enter(AppState::LoadingGameSprites).with_system(load_textures),
        )
        .add_system_set(
            SystemSet::on_update(AppState::LoadingGameSprites).with_system(check_loading_textures),
        )
        .add_system_set(SystemSet::on_enter(AppState::RunningGame).with_system(setup))
        // Systems
        .add_system_set(Animator::system_set())
        .add_system_set(Player::system_set())
        .add_system_set(Collider::system_set())
        .add_system_set(RigidBody::system_set())
        .add_system_set(dungeon::Dungeon::system_set())
        .add_system(bevy::input::system::exit_on_esc_system)
        // Inspect
        .register_inspectable::<Player>()
        .register_inspectable::<Monster>()
        .register_inspectable::<Animator>()
        .register_inspectable::<Collider>()
        // Run
        .run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    LoadingGameSprites,
    RunningGame,
}

#[derive(Default)]
struct SpriteHandles {
    player: Vec<HandleUntyped>,
    slime: Vec<HandleUntyped>,
    terrain: Option<Handle<Image>>,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    *sprite_handles = SpriteHandles {
        player: asset_server
            .load_folder("RoguelikeDungeon/Sprites/Player/Sword/Defence0")
            .unwrap(),
        slime: asset_server
            .load_folder("RoguelikeDungeon/Sprites/Monsters/Slime/Variant0")
            .unwrap(),
        terrain: Some(asset_server.load("Dungeon/Terrain/Dungeon_Terrain_Tileset.png")),
    }
}

fn check_loading_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    let player =
        asset_server.get_group_load_state(sprite_handles.player.iter().map(|handle| handle.id));
    let slime =
        asset_server.get_group_load_state(sprite_handles.slime.iter().map(|handle| handle.id));

    match (player, slime) {
        (LoadState::Loaded, LoadState::Loaded) => state.set(AppState::RunningGame).unwrap(),
        _ => {}
    }
}

fn setup(
    mut commands: Commands,
    sprite_handles: ResMut<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // Terrain
    let terrain_handle = sprite_handles.terrain.clone().unwrap();
    let terrain_atlas = TextureAtlas::from_grid(terrain_handle, Vec2::new(16., 16.), 13, 8);
    let terrain_atlas_handle = texture_atlases.add(terrain_atlas);
    commands
        .spawn()
        .insert(dungeon::Dungeon::new(terrain_atlas_handle));

    // Player
    let mut player_atlas_builder = TextureAtlasBuilder::default();

    for handle in &sprite_handles.player {
        let texture = textures.get(handle).unwrap();
        player_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }
    let player_atlas = player_atlas_builder.finish(&mut textures).unwrap();
    let player_atlas_handle = texture_atlases.add(player_atlas);
    commands.spawn_bundle(PlayerBundle::new(player_atlas_handle));

    // Others
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(Text2dBundle::default());
}

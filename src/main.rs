mod ab_gen;
mod assets;
mod cell;
mod dfs_gen;
mod maze_gen;
mod player;
mod tex;

use crate::ab_gen::AbMazeGen;
use crate::assets::Assets;
use crate::dfs_gen::DfsMazeGen;
use crate::maze_gen::MazeGen;
use crate::player::Player;
use macroquad::{
    audio::{PlaySoundParams, load_sound, play_sound},
    prelude::*,
};
use quad_gif::GifAnimation;

const COLS: usize = 21;
const ROWS: usize = 21;
const GRAV: f32 = 0.01;
const ZOOM: f32 = 3.0;

async fn load_asset(file_name: &str) -> Texture2D {
    load_texture(file_name)
        .await
        .expect("Failed to load `{file_name}`")
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Ant Maze"),
        window_width: 1000,
        window_height: 1000,
        window_resizable: false,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let size = screen_width() as i32 / COLS as i32;

    set_pc_assets_folder("assets");
    let mut maze_gen = AbMazeGen::<COLS, ROWS>::new();
    let cells = maze_gen.generate();

    // Load textures
    let dirt_1_tex = load_asset("dirt_1.png").await;
    let dirt_2_tex = load_asset("dirt_2.png").await;
    let dirt_3_tex = load_asset("dirt_3.png").await;
    let grass_1_tex = load_asset("grass_1.png").await;
    let grass_2_tex = load_asset("grass_2.png").await;
    let grass_3_tex = load_asset("grass_3.png").await;
    let background_1_tex = load_asset("background_1.png").await;
    let background_2_tex = load_asset("background_2.png").await;
    let background_3_tex = load_asset("background_3.png").await;
    let mushroom_tex = load_asset("mushroom.png").await;
    let rock_tex = load_asset("rock.png").await;
    let bush_left_tex = load_asset("bush_left.png").await;
    let bush_right_tex = load_asset("bush_right.png").await;

    // Bundle the assets together with an Assets struct
    let assets = Assets::new(
        dirt_1_tex,
        dirt_2_tex,
        dirt_3_tex,
        grass_1_tex,
        grass_2_tex,
        grass_3_tex,
        background_1_tex,
        background_2_tex,
        background_3_tex,
        mushroom_tex,
        rock_tex,
        bush_left_tex,
        bush_right_tex,
    );

    let player_left_tex = load_asset("player_left.png").await;
    let player_right_tex = load_asset("player_right.png").await;
    let player_left_climb_tex = GifAnimation::load("player_left_climb.gif".to_string()).await;
    let player_right_climb_tex = GifAnimation::load("player_right_climb.gif".to_string()).await;

    let ambiance = load_sound("ambiance.wav")
        .await
        .expect("Failed to load `ambiance.wav`");

    let mut player = Player::<COLS, ROWS>::new(
        cells,
        size as f32,
        (COLS - 2) as f32 * size as f32,
        (ROWS - 2) as f32 * size as f32,
        player_left_tex.width() * 0.03,
        player_left_tex.height() * 0.03,
        GRAV,
    );

    let mut camera = Camera2D {
        target: player.pos,
        zoom: vec2(2.0 / screen_width() * ZOOM, 2.0 / screen_height() * ZOOM),

        ..Default::default()
    };

    play_sound(
        &ambiance,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );

    loop {
        player.update();
        camera.target = camera.target.lerp(player.pos, 0.01);

        set_camera(&camera);

        clear_background(DARKBROWN);

        maze_gen.draw(&assets, size);
        player.draw();

        next_frame().await;
    }
}

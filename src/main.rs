mod ab_gen;
mod assets;
mod cell;
mod dfs_gen;
mod maze_gen;
mod player;
mod tex;
mod timer;

use crate::ab_gen::AbMazeGen;
use crate::assets::Assets;
use crate::dfs_gen::DfsMazeGen;
use crate::maze_gen::MazeGen;
use crate::player::Player;
use crate::timer::Timer;
use macroquad::{
    audio::{PlaySoundParams, load_sound, play_sound},
    prelude::*,
};
use quad_gif::GifAnimation;

const COLS: usize = 21;
const ROWS: usize = 21;
const GRAV: f32 = 200.0;
const ZOOM: f32 = 3.0;

async fn load_asset(file_name: &str) -> Texture2D {
    load_texture(file_name)
        .await
        .expect("Failed to load `{file_name}`")
}

async fn load_gif(files_name: &str, frame_delay: f32) -> GifAnimation {
    let mut anim = GifAnimation::load(files_name.to_string()).await;

    for frame in &mut anim.frames {
        frame.delay = frame_delay;
    }

    anim
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
    let mut player_won = false;
    let mut player_moved = false;
    let mut dt: f32;
    let mut timer = Timer::new();

    set_pc_assets_folder("assets");
    let mut maze_gen = AbMazeGen::<COLS, ROWS>::new();
    let cells = maze_gen.generate();
    let anim_fps = 10;
    let anim_frame_delay = 1.0 / anim_fps as f32;

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
    let player_left_idle_climb_tex = load_asset("player_left_idle_climb.png").await;
    let player_right_idle_climb_tex = load_asset("player_right_idle_climb.png").await;
    let mut player_left_climb_anim = load_gif("player_left_climb.gif", anim_frame_delay).await;
    let mut player_right_climb_anim = load_gif("player_right_climb.gif", anim_frame_delay).await;
    let mut player_left_walk_anim = load_gif("player_left_walk.gif", anim_frame_delay).await;
    let mut player_right_walk_anim = load_gif("player_right_walk.gif", anim_frame_delay).await;

    let sky_tex = load_asset("sky.png").await;

    let ambiance_sound = load_sound("ambiance.wav")
        .await
        .expect("Failed to load `ambiance.wav`");
    let walking_sound = load_sound("walking.wav")
        .await
        .expect("Failed to load `walking.wav`");
    let jump_sound = load_sound("jump.wav")
        .await
        .expect("Failed to load `jump.wav`");
    let land_sound = load_sound("land.wav")
        .await
        .expect("Failed to load `land.wav`");

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

    let win_camera = Camera2D {
        target: vec2(
            (COLS as f32 * size as f32) / 2.0,
            (ROWS as f32 * size as f32) / 2.0,
        ),
        zoom: vec2(2.0 / screen_width() / 1.15, 2.0 / screen_height() / 1.15),

        ..Default::default()
    };

    play_sound(
        &ambiance_sound,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );

    let mut win_timer = 0.0;

    loop {
        dt = get_frame_time().min(0.05);

        if !get_keys_pressed().is_empty() {
            player_moved = true;
        }

        if !player_won {
            camera.target = camera.target.lerp(player.pos, 4.0 * dt);

            set_camera(&camera);
        } else {
            set_camera(&win_camera);
        }

        if player.pos.y < 0.0 && !player_won {
            win_timer += dt;

            if win_timer > 2.0 {
                player_won = true;
            }
        } else {
            win_timer = 0.0;
        }

        clear_background(DARKBROWN);
        draw_texture_ex(
            &sky_tex,
            -200.0,
            -300.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(1500.0, 300.0)),
                ..Default::default()
            },
        );

        player.update(dt);
        player.play_sounds(&walking_sound, &jump_sound, &land_sound);

        if player.pos.y > 0.0 && player_moved {
            timer.update();
        }

        maze_gen.draw(&assets, size);

        player.draw(
            &player_left_tex,
            &player_right_tex,
            &mut player_left_walk_anim,
            &mut player_right_walk_anim,
            &player_left_idle_climb_tex,
            &player_right_idle_climb_tex,
            &mut player_left_climb_anim,
            &mut player_right_climb_anim,
        );

        if player_won {
            draw_text("You Won!", 300.0, 500.0, 100.0, GREEN);
            draw_text("r to reset", 400.0, 550.0, 30.0, WHITE);

            if is_key_pressed(KeyCode::R) {
                maze_gen.reset();
                maze_gen.generate();
                player.pos.x = (COLS - 2) as f32 * size as f32;
                player.pos.y = (ROWS - 2) as f32 * size as f32;
                player.cells = maze_gen.cells;
                set_camera(&camera);
                camera.target = player.pos;
                player_won = false;
                timer.reset();
                player_moved = false;
            }
        }

        set_default_camera();

        if !player_moved {
            draw_text("Press any key to start!", 250.0, 450.0, 50.0, WHITE);
            draw_text(
                "Get to the surface as quickly as possible!",
                50.0,
                600.0,
                50.0,
                WHITE,
            );
        }

        if player.pos.y < 0.0 && !player_won {
            timer.blink_time();
            timer.update_best();
        } else {
            timer.draw_time();
        }

        if player_won && timer.time == timer.best_time {
            timer.wave_new_best();
            timer.wave_best();
        } else if !player_won || (player_won && timer.time != timer.best_time) {
            timer.draw_best();
        }

        next_frame().await;
    }
}

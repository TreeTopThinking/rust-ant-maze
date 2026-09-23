mod ab_gen;
mod assets;
mod cell;
mod dfs_gen;
mod menu;
mod tex;

use crate::ab_gen::AbMazeGen;
use crate::assets::Assets;
use crate::cell::Cell;
use crate::dfs_gen::DfsMazeGen;
use crate::menu::Menu;

use raylib::prelude::*;

pub const WIDTH: i32 = 1050;
pub const HEIGHT: i32 = 1050;
pub const COLS: usize = 21;
pub const ROWS: usize = 21;
pub const SIZE: i32 = WIDTH / COLS as i32;

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Maze").build();
    let menu_open = false;
    let menu = Menu::<WIDTH, HEIGHT>::new(50);

    let mut maze_gen = DfsMazeGen::<COLS, ROWS>::new();
    maze_gen.generate();

    // Load textures
    let dirt_1_tex = rl
        .load_texture(&thread, "assets/dirt_1.png")
        .expect("Failed to load dirt_1.png");
    let dirt_2_tex = rl
        .load_texture(&thread, "assets/dirt_2.png")
        .expect("Failed to load dirt_2.png");
    let dirt_3_tex = rl
        .load_texture(&thread, "assets/dirt_3.png")
        .expect("Failed to load dirt_3.png");
    let grass_1_tex = rl
        .load_texture(&thread, "assets/grass_1.png")
        .expect("Failed to load gras_1.png");
    let grass_2_tex = rl
        .load_texture(&thread, "assets/grass_2.png")
        .expect("Failed to load grass_2.png");
    let grass_3_tex = rl
        .load_texture(&thread, "assets/grass_3.png")
        .expect("Failed to load grass_3.png");
    let background_1_tex = rl
        .load_texture(&thread, "assets/background_1.png")
        .expect("Failed to load background.png");
    let background_2_tex = rl
        .load_texture(&thread, "assets/background_2.png")
        .expect("Failed to load background.png");
    let background_3_tex = rl
        .load_texture(&thread, "assets/background_3.png")
        .expect("Failed to load background.png");
    let mushroom_tex = rl
        .load_texture(&thread, "assets/mushroom.png")
        .expect("Failed to load mushroom.png");
    let rock_tex = rl
        .load_texture(&thread, "assets/rock.png")
        .expect("Failed to load rock.png");
    let bush_left_tex = rl
        .load_texture(&thread, "assets/bush_left.png")
        .expect("Failed to load bush_left.png");
    let bush_right_tex = rl
        .load_texture(&thread, "assets/bush_right.png")
        .expect("Failed to load bush_right.png");
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

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        maze_gen.draw(&mut d, &assets, SIZE);
    }
}

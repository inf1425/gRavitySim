use std::time::Instant;

use macroquad::{
    color::Color,
    color_u8,
    input::{
        is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed,
        is_mouse_button_released, mouse_position, mouse_wheel,
    },
    window::{screen_height, screen_width},
};
use rand::Rng;

use crate::{render, sim::update, GravityObject};
use nalgebra::Vector2;

pub const MOVE_SPEEDS: [f32; 4] = [1.0, 2.0, 5.0, 10.0];
pub const SIMULATION_SPEEDS: [u32; 7] = [1, 2, 5, 10, 50, 100, 500];
pub const ZOOM_LEVELS: [f32; 18] = [
    0.2, 0.3, 0.5, 0.8, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.666667, 8.0, 10.0, 25.0, 50.0, 100.0
];

pub struct RenderData {
    pub zoom: f32,
    pub screen_offset_x: f32,
    pub screen_offset_y: f32,
    pub last_screen_size_x: f32,
    pub last_screen_size_y: f32,
    pub frames: u32,
    pub total_secs: f64,
    pub mspf: f64,
    pub move_speed: f32,
    pub render_orbits: bool,
    pub following: Option<u128>,
    pub relative_orbits: bool,
}

pub struct GameData {
    pub objs: Vec<GravityObject>,
    pub mouse_start: Option<Vector2<f32>>,
    pub cur_mouse: Vector2<f32>,
    pub paused: bool,
    pub cur_sim_speed: u32,
    pub mspf: f64,
    pub frames: u32,
    pub total_secs: f64
}

impl RenderData {
    pub fn new() -> RenderData {
        RenderData {
            zoom: 1.0,
            screen_offset_x: 0.0,
            screen_offset_y: 0.0,
            last_screen_size_x: 0.0,
            last_screen_size_y: 0.0,
            frames: 0,
            total_secs: 0.0,
            mspf: 0.0,
            move_speed: 2.0,
            render_orbits: false,
            following: None,
            relative_orbits: false,
        }
    }
}

pub fn run_frame(render_data: &mut RenderData, game_data: &mut GameData) {
    let frame_start = Instant::now();
    take_input(render_data, game_data);
    if !game_data.paused {
        for _ in 0..game_data.cur_sim_speed {
            update(&mut game_data.objs);
        }
    }
    game_data.frames += 1;
    game_data.total_secs += frame_start.elapsed().as_secs_f64();
    if game_data.frames == 30 {
        game_data.mspf = (game_data.total_secs / (game_data.frames) as f64) * 1000.0;
        game_data.frames = 0;
        game_data.total_secs = 0.0;
    }
    render::render(render_data, game_data);
}

fn take_input(render_data: &mut RenderData, game_data: &mut GameData) {
    let d_zoom = -mouse_wheel().1.ceil() as i32; //delta zoom
    if d_zoom.abs() > 0 {
        let mut cur_zoom = 4;
        for (i, z_lvl) in ZOOM_LEVELS.iter().enumerate() {
            if *z_lvl == render_data.zoom {
                cur_zoom = i;
                break;
            }
        }

        let old_wx = (-render_data.screen_offset_x + render_data.last_screen_size_x * 0.5)
            * render_data.zoom;
        let old_wy = (-render_data.screen_offset_y + render_data.last_screen_size_y * 0.5)
            * render_data.zoom;
        if d_zoom == 1 && cur_zoom < ZOOM_LEVELS.len() - 1 {
            render_data.zoom = ZOOM_LEVELS[cur_zoom + 1];
        } else if d_zoom == -1 && cur_zoom > 0 {
            render_data.zoom = ZOOM_LEVELS[cur_zoom - 1];
        }
        render_data.screen_offset_x =
            (0.5 * render_data.last_screen_size_x) - old_wx / (render_data.zoom);
        render_data.screen_offset_y =
            (0.5 * render_data.last_screen_size_y) - old_wy / (render_data.zoom);
    }

    let (mx, my) = mouse_position();
    let mouse_pos_world: Vector2<f32> = render::translate_from_screen_coords(mx, my, render_data);
    game_data.cur_mouse = Vector2::<f32>::new(mouse_pos_world.x, mouse_pos_world.y); //set cur mouse pos for rendering line

    if is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
        game_data.mouse_start = Some(mouse_pos_world); //for drag n drop, take starting mouse pos
    }

    //for deleting objs, check if rmb down and delete
    if is_mouse_button_down(macroquad::input::MouseButton::Right) {
        let mut to_del: Option<usize> = None;
        let mp: Vector2<f64> = Vector2::new(mouse_pos_world.x as f64, mouse_pos_world.y as f64);

        for i in 1..game_data.objs.len() {
            let obj_i = &game_data.objs[i];
            if mp.metric_distance(&obj_i.pos) <= obj_i.radius as f64 {
                to_del = Some(i);
                break;
            }
        }

        if let Some(to_del) = to_del {
            game_data.objs.swap_remove(to_del);
        }
    }

    //for drag n drop, when released, spawn obj
    if is_mouse_button_released(macroquad::input::MouseButton::Left) {
        let dx: Vector2<f32> = game_data.mouse_start.unwrap() - mouse_pos_world;
        let dx: Vector2<f64> = Vector2::new(dx.x as f64, dx.y as f64);
        let mpos: Vector2<f32> = game_data.mouse_start.unwrap();
        let spawn_pos: Vector2<f64> = Vector2::new(mpos.x as f64, mpos.y as f64);
        let color = color_u8!(
            rand::random::<u8>().clamp(100, 220),
            rand::random::<u8>().clamp(100, 220),
            rand::random::<u8>().clamp(100, 220),
            255
        );
        let mass = rand::thread_rng().gen_range(10.0..500.0);

        game_data.objs.push(GravityObject::new(
            spawn_pos,
            dx * -0.05,
            ((mass + 500.0) / 75.0) as u8,
            mass,
            color,
            true,
        ));
        game_data.mouse_start = None;
    }

    //cycle through camera speeds on pressing E
    if is_key_pressed(macroquad::input::KeyCode::E) {
        let mut idx: Option<usize> = None;
        for (i, speed) in MOVE_SPEEDS.iter().enumerate() {
            if *speed == render_data.move_speed {
                idx = Some(i);
                break;
            }
        }

        render_data.move_speed =
            MOVE_SPEEDS[(idx.unwrap_or(MOVE_SPEEDS.len() - 1) + 1) % MOVE_SPEEDS.len()];
    }

    //cycle through simulation speeds on pressing T
    if is_key_pressed(macroquad::input::KeyCode::T) {
        let mut idx: Option<usize> = None;
        for (i, speed) in SIMULATION_SPEEDS.iter().enumerate() {
            if *speed == game_data.cur_sim_speed {
                idx = Some(i);
                break;
            }
        }

        game_data.cur_sim_speed =
            SIMULATION_SPEEDS[(idx.unwrap_or(SIMULATION_SPEEDS.len() - 1) + 1) % SIMULATION_SPEEDS.len()];
    }

    //toggle rendering orbits and pausing using Q and P
    if is_key_pressed(macroquad::input::KeyCode::Q) {
        render_data.render_orbits = !render_data.render_orbits;
    }
    if is_key_pressed(macroquad::input::KeyCode::P) {
        game_data.paused = !game_data.paused;
    }

    //movement using WASD
    if is_key_down(macroquad::input::KeyCode::W) {
        render_data.screen_offset_y += render_data.move_speed * 5.0;
    }
    if is_key_down(macroquad::input::KeyCode::S) {
        render_data.screen_offset_y -= render_data.move_speed * 5.0;
    }
    if is_key_down(macroquad::input::KeyCode::A) {
        render_data.screen_offset_x += render_data.move_speed * 5.0;
    }
    if is_key_down(macroquad::input::KeyCode::D) {
        render_data.screen_offset_x -= render_data.move_speed * 5.0;
    }

    //follow an object by pressing F
    if is_key_pressed(macroquad::input::KeyCode::F) {
        let mut to_follow: Option<u128> = None;
        let mp: Vector2<f64> = Vector2::new(mouse_pos_world.x as f64, mouse_pos_world.y as f64);

        for obj_i in game_data.objs.iter().skip(0) {
            if mp.metric_distance(&obj_i.pos) <= obj_i.radius as f64 {
                to_follow = Some(obj_i.id);
                break;
            }
        }

        if let Some(to_follow) = to_follow {
            render_data.following = Some(to_follow)
        } else {
            render_data.following = None
        }
    }

    //relative orbits toggle
    if is_key_pressed(macroquad::input::KeyCode::R) {
        render_data.relative_orbits = !render_data.relative_orbits;
    }

    //handle window size changing, recenter view
    let scr_width = screen_width();
    let scr_height = screen_height();

    if scr_width != render_data.last_screen_size_x || scr_height != render_data.last_screen_size_y {
        render_data.screen_offset_x +=
            (scr_width - render_data.last_screen_size_x) * 0.5 * render_data.zoom;
        render_data.screen_offset_y +=
            (scr_height - render_data.last_screen_size_y) * 0.5 * render_data.zoom;
        render_data.last_screen_size_x = scr_width;
        render_data.last_screen_size_y = scr_height;
    }
}

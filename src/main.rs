mod game;
mod render;
mod sim;

use macroquad::{color::Color, color_u8, window::next_frame};

use game::{GameData, RenderData};
use nalgebra::Vector2;
use sim::GravityObject;

#[macroquad::main("Gravity Sim")]
async fn main() {
    let mut game_data: GameData = GameData {
        objs: vec![
            GravityObject::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(0.0, 0.0),
                25,
                100000.0,
                color_u8!(255, 255, 255, 255),
                false,
            ),
            GravityObject::new(
                Vector2::new(0.0, -100.0),
                Vector2::new(10.0, 0.0),
                6,
                1.0,
                color_u8!(80, 80, 80, 255),
                true,
            ),
            GravityObject::new(
                Vector2::new(0.0, -200.0),
                Vector2::new(7.0, 0.0),
                9,
                1.0,
                color_u8!(217, 174, 56, 255),
                true,
            ),
            GravityObject::new(
                Vector2::new(0.0, -700.0),
                Vector2::new(3.7, 0.0),
                11,
                200.0,
                color_u8!(50, 84, 160, 255),
                true,
            ),
            GravityObject::new(
                Vector2::new(0.0, -660.0),
                Vector2::new(4.45, 0.0),
                2,
                1.0,
                color_u8!(230, 230, 230, 255),
                true,
            ),
            GravityObject::new(
                Vector2::new(0.0, -450.0),
                Vector2::new(2.0, 0.0),
                5,
                1.0,
                color_u8!(255, 255, 160, 255),
                true,
            ),
        ],

        mouse_start: None,
        cur_mouse: Vector2::<f32>::new(0.0, 0.0),
        paused: true,
        cur_sim_speed: 1,
        mspf: 0.0,
        frames: 0,
        total_secs: 0.0
    };

    let mut render_data = RenderData::new();

    loop {
        game::run_frame(&mut render_data, &mut game_data);
        next_frame().await
    }
}

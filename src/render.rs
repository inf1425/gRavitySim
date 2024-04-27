use std::time::Instant;

use macroquad::{
    color::Color,
    color_u8,
    shapes::{draw_circle, draw_line},
    text::draw_text,
    window::clear_background,
};

use crate::{game::MOVE_SPEEDS, GameData, GravityObject, RenderData};
use nalgebra::Vector2;

pub fn render(render_data: &mut RenderData, game_data: &GameData) {
    let frame_start = Instant::now();
    clear_background(color_u8![0, 0, 0, 0]);

    for i in &game_data.objs {
        render_obj(i, render_data);
        if let Some(follow) = render_data.following {
            if follow == i.id {
                render_data.screen_offset_x = (render_data.last_screen_size_x * 0.5 * render_data.zoom - i.pos.x as f32) / render_data.zoom;
                render_data.screen_offset_y = (render_data.last_screen_size_y * 0.5 * render_data.zoom - i.pos.y as f32) / render_data.zoom;
            }
        }
    }

    if let Some(x) = game_data.mouse_start {
        let render_start: Vector2<f32> = translate_to_screen_coords(x.x, x.y, render_data);
        let render_end: Vector2<f32> =
            translate_to_screen_coords(game_data.cur_mouse.x, game_data.cur_mouse.y, render_data);
        draw_line(
            render_start.x,
            render_start.y,
            render_end.x,
            render_end.y,
            2.0,
            color_u8!(255, 255, 255, 255),
        );
    }

    draw_text(
        &format!(
            "{:.2} mspf ({} objects) (X: {:.2}, Y:{:.2}, {:.2}%)",
            render_data.mspf,
            game_data.objs.len(),
            (-render_data.screen_offset_x + render_data.last_screen_size_x * 0.5) * render_data.zoom,
            (-render_data.screen_offset_y + render_data.last_screen_size_y * 0.5) * render_data.zoom,
            100.0 / render_data.zoom
        ),
        20.0,
        20.0,
        20.0,
        color_u8!(255, 255, 255, 255),
    );

    draw_text(
        "Camera Speed:",
        20.0,
        40.0,
        20.0,
        color_u8!(255, 255, 255, 255),
    );

    for i in (0..MOVE_SPEEDS.len()).zip(MOVE_SPEEDS) {
        draw_text(
            &format!("{:.0}x", i.1),
            140.0 + i.0 as f32 * 30.0,
            40.0,
            20.0,
            if i.1 == render_data.move_speed {
                color_u8!(150, 255, 150, 255)
            } else {
                color_u8!(255, 255, 255, 255)
            },
        );
    }

    render_data.total_secs += frame_start.elapsed().as_secs_f64();
    render_data.frames += 1;

    if render_data.frames == 30 {
        render_data.mspf = (render_data.total_secs / render_data.frames as f64) * 1000.0;
        render_data.frames = 0;
        render_data.total_secs = 0.0;
    }
}

pub fn render_obj(obj: &GravityObject, render_data: &RenderData) {
    if render_data.render_orbits && obj.is_dynamic {
        for j in 0..obj.old_positions.len() {
            let cur: &Vector2<f64> = obj.old_positions.get(j).unwrap();
            let next: &Vector2<f64> = obj.old_positions.get(j + 1).unwrap_or(&obj.pos);
            draw_line(
                cur.x as f32 / render_data.zoom + render_data.screen_offset_x,
                cur.y as f32 / render_data.zoom + render_data.screen_offset_y,
                next.x as f32 / render_data.zoom + render_data.screen_offset_x,
                next.y as f32 / render_data.zoom + render_data.screen_offset_y,
                (2.0 / render_data.zoom).clamp(1.0, f32::INFINITY),
                color_u8!(40, 40, 160, 255),
            );
        }
    }

    let render_pos: Vector2<f32> =
        translate_to_screen_coords(obj.pos.x as f32, obj.pos.y as f32, render_data);
    draw_circle(
        render_pos.x,
        render_pos.y,
        obj.radius as f32 / render_data.zoom,
        obj.color,
    );
}

pub fn translate_from_screen_coords(x: f32, y: f32, render_data: &RenderData) -> Vector2<f32> {
    let x = (-render_data.screen_offset_x + x) * render_data.zoom;
    let y = (-render_data.screen_offset_y + y) * render_data.zoom;
    Vector2::<f32>::new(x, y)
}

pub fn translate_to_screen_coords(x: f32, y: f32, render_data: &RenderData) -> Vector2<f32> {
    let x = x / render_data.zoom + render_data.screen_offset_x;
    let y = y / render_data.zoom + render_data.screen_offset_y;
    Vector2::<f32>::new(x, y)
}

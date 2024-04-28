use std::time::Instant;

use macroquad::{
    color::Color,
    color_u8,
    shapes::{draw_circle, draw_line},
    text::draw_text,
    window::clear_background,
};

use crate::{game::{MOVE_SPEEDS, SIMULATION_SPEEDS}, GameData, GravityObject, RenderData};
use nalgebra::Vector2;

pub fn render(render_data: &mut RenderData, game_data: &GameData) {
    let frame_start = Instant::now();
    clear_background(color_u8![0, 0, 0, 0]);

    let mut follow_found: Option<&GravityObject> = None;
    for i in &game_data.objs {
        if let Some(follow) = render_data.following {
            if follow == i.id {
                render_data.screen_offset_x =
                    (render_data.last_screen_size_x * 0.5 * render_data.zoom - i.pos.x as f32)
                        / render_data.zoom;
                render_data.screen_offset_y =
                    (render_data.last_screen_size_y * 0.5 * render_data.zoom - i.pos.y as f32)
                        / render_data.zoom;
                follow_found = Some(i);
            }
        }
    }

    if follow_found.is_none() {
        render_data.following = None;
    }

    for i in &game_data.objs {
        render_obj(i, render_data, game_data);
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
            "{:.2} mspf ({:.2} ms sim, {} objects) (X: {:.2}, Y:{:.2}, {:.2}%)",
            render_data.mspf + game_data.mspf,
            game_data.mspf,
            game_data.objs.len(),
            (-render_data.screen_offset_x + render_data.last_screen_size_x * 0.5)
                * render_data.zoom,
            (-render_data.screen_offset_y + render_data.last_screen_size_y * 0.5)
                * render_data.zoom,
            100.0 / render_data.zoom
        ),
        20.0,
        20.0,
        20.0,
        color_u8!(255, 255, 255, 255),
    );

    if game_data.paused {
        draw_text(
            "Paused",
            render_data.last_screen_size_x - 70.0,
            20.0,
            20.0,
            color_u8!(255, 255, 255, 255),
        );
    }

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

    if render_data.relative_orbits {
        draw_text(
            "(Relative Orbits)",
            160.0 + MOVE_SPEEDS.len() as f32 * 30.0,
            40.0,
            20.0,
            color_u8!(160, 160, 160, 160),
        );
    }

    draw_text(
        "Simulation Speed:",
        20.0,
        60.0,
        20.0,
        color_u8!(255, 255, 255, 255),
    );

    for i in (0..SIMULATION_SPEEDS.len()).zip(SIMULATION_SPEEDS) {
        draw_text(
            &format!("{:.0}x", i.1),
            (if i.0 < 3 { 175.0 } else { if i.0 == 6 { 153.0 }  else { 145.0 } }) + i.0 as f32 * (if i.0 < 3 { 30.0 } else { 40.0 }),
            60.0,
            20.0,
            if i.1 == game_data.cur_sim_speed {
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

pub fn render_obj(obj: &GravityObject, render_data: &RenderData, game_data: &GameData) {
    if render_data.render_orbits {
        render_orbits(obj, render_data, game_data);
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

pub fn render_orbits(obj: &GravityObject, render_data: &RenderData, game_data: &GameData) {
    let mut following_obj: Option<&GravityObject> = None;
    if render_data.relative_orbits && render_data.following.is_some() {
        for i in &game_data.objs {
            if render_data.following.unwrap() == i.id {
                following_obj = Some(&i);
                break;
            }
        }
    }

    for j in 0..obj.old_positions.len() {
        let cur: Vector2<f64>;
        let next: Vector2<f64>;

        match following_obj {
            Some(following_obj) => {
                cur = obj.old_positions.get(j).unwrap()
                    - following_obj.old_positions.get(j).unwrap()
                    + following_obj.pos;
                next = obj.old_positions.get(j + 1).unwrap_or(&obj.pos)
                    - following_obj
                        .old_positions
                        .get(j + 1)
                        .unwrap_or(&following_obj.pos)
                    + following_obj.pos;
            }
            None => {
                cur = *obj.old_positions.get(j).unwrap();
                next = *obj.old_positions.get(j + 1).unwrap_or(&obj.pos);
            }
        }

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

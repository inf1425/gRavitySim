use macroquad::color::Color;
use nalgebra::Vector2;
use std::collections::VecDeque;

const G_GRAV: f64 = 0.1;

pub struct GravityObject {
    pub pos: Vector2<f64>,
    pub vel: Vector2<f64>,
    pub radius: u8,
    pub mass: f64,
    pub color: Color,
    pub old_positions: VecDeque<Vector2<f64>>,
    pub is_dynamic: bool,
    pub id: u128,
}

impl GravityObject {
    pub fn new(
        pos: Vector2<f64>,
        vel: Vector2<f64>,
        radius: u8,
        mass: f64,
        color: Color,
        is_dynamic: bool,
    ) -> GravityObject {
        assert!(radius > 0);
        assert!(mass > 0.0);
        GravityObject {
            pos,
            vel,
            radius,
            mass,
            color,
            old_positions: VecDeque::new(),
            is_dynamic,
            id: rand::random::<u128>(),
        }
    }
}

pub fn update(objs: &mut Vec<GravityObject>) {
    let mut del = 0;
    let mut to_del = vec![];
    for (i, ob_i) in objs.iter().skip(1).enumerate() {
        if ob_i.pos.metric_distance(&objs[0].pos) < objs[0].radius as f64 {
            to_del.push(i + 1);
        }
    }
    for i in to_del {
        objs.swap_remove(i - del);
        del += 1;
    }

    for i in 0..objs.len() {
        for j in 0..objs.len() {
            if i == j {
                continue;
            }

            let obj = &objs[i];
            let obj_other = &objs[j];
            let del_pos: Vector2<f64> = obj_other.pos - obj.pos;

            let force: Vector2<f64> = (G_GRAV * del_pos.normalize() * obj.mass * obj_other.mass)
                / (del_pos.magnitude().powi(2));
            let accel: Vector2<f64> = force / obj.mass;

            if objs[i].is_dynamic {
                objs[i].vel += accel;
            }
        }
        let vel: Vector2<f64> = objs[i].vel;
        let old_pos: Vector2<f64> = objs[i].pos;

        if objs[i].is_dynamic {
            objs[i].pos = vel + old_pos;
        }
        objs[i].old_positions.push_back(old_pos);
        if objs[i].old_positions.len() > 2000 {
            objs[i].old_positions.pop_front();
        }
    }
}

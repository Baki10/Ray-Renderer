
use macroquad::{prelude::*};

use crate::Player;

pub static WIDTH: f32 = 9.0;
pub static HEIGHT: f32 = 9.0;
pub static SCREEN_WIDTH: f32 = 1600.0;
pub static SCREEN_HEIGHT: f32 = 900.0;

pub static SPEED: f32 = 0.8;
pub static SENSITIVITY: f32 = 50.0;
pub static VIEW_DISTANCE: f32 = 9.0;
pub static RIGHT_ANGLE: f32 = 3.1415/2.;

pub static MAP: [[&str; WIDTH as usize]; HEIGHT as usize] =            [["#","#","#","#","#","#","#","#","#"],
                                                                        ["#"," "," "," "," "," "," "," ","#"],
                                                                        ["#"," ","#","#"," "," "," "," ","#"],
                                                                        ["#"," ","#"," "," "," "," "," ","#"],
                                                                        ["#"," "," "," ","@"," ","#"," ","#"],
                                                                        ["#"," ","#"," "," "," "," "," ","#"],
                                                                        ["#"," ","#"," "," "," ","#"," ","#"],
                                                                        ["#"," ","#"," "," "," "," "," ","#"],
                                                                        ["#","#","#","#","#","#","#","#","#"]];


pub fn is_wall(position: &Vec2) -> (bool, Vec2) {
    let grid_x: i32 = position.x as i32;
    let grid_y: i32 = position.y as i32;

    for wall_x in 0..WIDTH as i32 {
        for wall_y in 0..HEIGHT as i32 {

            if MAP[wall_y as usize][wall_x as usize] == "#" {

                if wall_x == grid_x && wall_y == grid_y {
                    return (true, Vec2::new(wall_x as f32, wall_y as f32));
                }
            }
        }
    }

    return (false, Vec2::new(-1.,-1.));
}

pub fn update_key_input(player: &mut Player, mouse_lock: &mut bool, show_fps: &mut bool) {

    movement_keys(player);
    debug_key_input(player);

    if is_key_pressed(KeyCode::F) {
        *show_fps = !*show_fps;
    }

    if is_key_pressed(KeyCode::Escape) {
        *mouse_lock = !*mouse_lock;
        set_cursor_grab(*mouse_lock);
        show_mouse(!*mouse_lock);
    }
}

fn movement_keys(player: &mut Player) {

    let delta_time: f32 = get_frame_time();

    let ws_direction: f32 = (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32;
    let ad_direction: f32 = (is_key_down(KeyCode::D) as i32 - is_key_down(KeyCode::A) as i32) as f32;

    let mut new_position: Vec2 = Vec2::new(player.position.x, player.position.y);

    new_position.x += ws_direction * f32::cos(player.camera_angle) * SPEED*delta_time;
    new_position.y += ws_direction * f32::sin(player.camera_angle) * SPEED*delta_time;

    new_position.x += ad_direction * f32::cos(player.camera_angle + RIGHT_ANGLE) * SPEED*delta_time;
    new_position.y += ad_direction * f32::sin(player.camera_angle + RIGHT_ANGLE) * SPEED*delta_time;

    if !is_wall(&new_position).0 {
        player.position = new_position;
    }
}

pub fn update_mouse_input(delta_time: &f32, player: &mut Player) {
    let delta_x: f32 = mouse_delta_position().x;
    player.camera_angle -= delta_x*SENSITIVITY*(*delta_time);
}

fn debug_key_input(player: &mut Player) {
    if is_key_down(KeyCode::P) {
        player.fov+=3.1415/90.0;
    }
    if is_key_down(KeyCode::O) {
        player.fov-=3.1415/90.0;
    }

    if is_key_down(KeyCode::L) {
        player.ray_resolution+=1.0;
    }
    if is_key_down(KeyCode::K) {
        player.ray_resolution-=1.0;
    }
}
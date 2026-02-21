
use macroquad::{prelude::*};

use crate::Player;


static WIDTH: f32 = 9.0;
static HEIGHT: f32 = 9.0;

static SPEED: f32 = 0.8;
static SENSITIVITY: f32 = 50.0;
static RIGHT_ANGLE: f32 = 3.1415/2.;
static MAP: [[&str; WIDTH as usize]; HEIGHT as usize] =                [["#","#","#","#","#","#","#","#","#"],
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

pub fn update_key_input(player: &mut Player, mouse_lock: &mut bool) {

    let delta_time: f32 = get_frame_time();
    if is_key_down(KeyCode::W) {
        
        let new_x: f32 = player.position.x + f32::cos(player.camera_angle) * SPEED*delta_time;
        let new_y: f32 = player.position.y + f32::sin(player.camera_angle) * SPEED*delta_time;

        if !is_wall(&Vec2::new(new_x, new_y)).0 {
            player.position.x = new_x;
            player.position.y = new_y;
        }

    }

    if is_key_down(KeyCode::S) {

        let new_x: f32 = player.position.x - f32::cos(player.camera_angle) * SPEED*delta_time;
        let new_y: f32 = player.position.y - f32::sin(player.camera_angle) * SPEED*delta_time;

        if !is_wall(&Vec2::new(new_x, new_y)).0 {
            player.position.x = new_x;
            player.position.y = new_y;
        }

    }
    if is_key_down(KeyCode::A) {

        let new_x: f32 = player.position.x + f32::cos(player.camera_angle - RIGHT_ANGLE) * SPEED*delta_time;
        let new_y: f32 = player.position.y + f32::sin(player.camera_angle - RIGHT_ANGLE) * SPEED*delta_time;

        if !is_wall(&Vec2::new(new_x, new_y)).0 {
            player.position.x = new_x;
            player.position.y = new_y;
        }

    }
    if is_key_down(KeyCode::D) {

        let new_x: f32 = player.position.x + f32::cos(player.camera_angle + RIGHT_ANGLE) * SPEED*delta_time;
        let new_y: f32 = player.position.y + f32::sin(player.camera_angle + RIGHT_ANGLE) * SPEED*delta_time;

        if !is_wall(&Vec2::new(new_x, new_y)).0 {
            player.position.x = new_x;
            player.position.y = new_y;
        }
    }

    debug_key_input(player);

    if is_key_pressed(KeyCode::Escape) {
        *mouse_lock = !*mouse_lock;
        set_cursor_grab(*mouse_lock);
        show_mouse(!*mouse_lock);
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
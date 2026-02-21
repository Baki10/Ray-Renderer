
use macroquad::{prelude::*};


static WIDTH: f32 = 9.0;
static HEIGHT: f32 = 9.0;

static SPEED: f32 = 0.8;
static SENSITIVITY: f32 = 50.0;
static MAP: [[&str; WIDTH as usize]; HEIGHT as usize] =                [["#","#","#","#","#","#","#","#","#"],
                                                                        ["#"," "," "," "," "," "," "," ","#"],
                                                                        ["#"," ","#","#"," "," "," "," ","#"],
                                                                        ["#"," ","#"," "," "," "," "," ","#"],
                                                                        ["#"," "," "," ","@"," ","#"," ","#"],
                                                                        ["#"," ","#"," "," "," "," "," ","#"],
                                                                        ["#"," ","#"," "," "," ","#"," ","#"],
                                                                        ["#"," ","#"," "," "," "," "," ","#"],
                                                                        ["#","#","#","#","#","#","#","#","#"]];


pub fn is_wall(x: &f32, y: &f32) -> (bool, f32, f32) {

    let grid_x: i32 = *x as i32;
    let grid_y: i32 = *y as i32;

    for wall_x in 0..WIDTH as i32 {
        for wall_y in 0..HEIGHT as i32 {

            if MAP[wall_y as usize][wall_x as usize] == "#" {

                if wall_x == grid_x && wall_y == grid_y {
                    return (true, wall_x as f32, wall_y as f32);
                }
            }
        }
    }

    return (false, -1., -1.);
}

pub fn update_key_input(delta_time: &f32, x: &mut f32, y: &mut f32, camera_angle: &mut f32, fov: &mut f32, x_resolution: &mut f32, mouse_lock: &mut bool) {

    if is_key_down(KeyCode::W) {
        
        let new_x: f32 = *x + f32::cos(*camera_angle)*SPEED*(*delta_time);
        let new_y: f32 = *y + f32::sin(*camera_angle)*SPEED*(*delta_time);
        if !is_wall(&new_x, &new_y).0 {
            *x = new_x;
            *y = new_y;
        }
    }
    if is_key_down(KeyCode::S) {
        let new_x: f32 = *x - f32::cos(*camera_angle)*SPEED*(*delta_time);
        let new_y: f32 = *y - f32::sin(*camera_angle)*SPEED*(*delta_time);
        if !is_wall(&new_x, &new_y).0 {
            *x = new_x;
            *y = new_y;
        }
    }
    if is_key_down(KeyCode::A) {

        let new_x: f32 = *x + f32::cos(*camera_angle-3.1415/2.0)*SPEED*(*delta_time);
        let new_y: f32 = *y + f32::sin(*camera_angle-3.1415/2.0)*SPEED*(*delta_time);
        if !is_wall(&new_x, &new_y).0 {
            *x = new_x;
            *y = new_y;
        }
    }
    if is_key_down(KeyCode::D) {
        let new_x: f32 = *x + f32::cos(*camera_angle+3.1415/2.0)*SPEED*(*delta_time);
        let new_y: f32 = *y + f32::sin(*camera_angle+3.1415/2.0)*SPEED*(*delta_time);
        if !is_wall(&new_x, &new_y).0 {
            *x = new_x;
            *y = new_y;
        }
    }

    if is_key_down(KeyCode::Right) {
        *camera_angle += 0.01;
    }
    if is_key_down(KeyCode::Left) {
        *camera_angle -= 0.01;
    }

    if is_key_down(KeyCode::P) {
        *fov+=3.1415/90.0;
    }
    if is_key_down(KeyCode::O) {
        *fov-=3.1415/90.0;
    }

    if is_key_down(KeyCode::L) {
        *x_resolution+=1.0;
    }
    if is_key_down(KeyCode::K) {
        *x_resolution-=1.0;
    }

    if is_key_pressed(KeyCode::Escape) {
        *mouse_lock = !*mouse_lock;
        set_cursor_grab(*mouse_lock);
        show_mouse(!*mouse_lock);
    }
}

pub fn update_mouse_input(delta_time: &f32, camera_angle: &mut f32) {
    let delta_x: f32 = mouse_delta_position().x;
    *camera_angle -= delta_x*SENSITIVITY*(*delta_time);
}
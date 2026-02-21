
use macroquad::{prelude::*};
mod ray_casting;
mod utils;

static SCREEN_WIDTH: f32 = 1600.0;
static SCREEN_HEIGHT: f32 = 900.0;

static WIDTH: f32 = 9.0;
static HEIGHT: f32 = 9.0;


#[macroquad::main("MyGame")]
async fn main() {

    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    
    let mut x: f32 = WIDTH/2.0;
    let mut y: f32 = HEIGHT/2.0;

    let mut camera_angle: f32 = 0.0;
    let mut x_resolution: f32 = 200.0;
    let mut fov: f32 = 3.1415/3.0;
    let mut mouse_lock: bool = false;
    let delta_time: f32 = get_frame_time();

    let wall_texture: Texture2D = load_texture("src/wall_texture.png").await.unwrap();


    loop {
        clear_background(BLACK);

        draw_rectangle(0.0, SCREEN_HEIGHT/2.0, SCREEN_WIDTH, SCREEN_HEIGHT/2.0, Color::new(0.15, 0.15, 0.15, 1.));
        ray_casting::scatter_rays(&x, &y, &camera_angle, &fov, &x_resolution, &wall_texture);

        utils::update_key_input(&delta_time, &mut x, &mut y, &mut camera_angle, &mut fov, &mut x_resolution,&mut mouse_lock);
        if mouse_lock {
            utils::update_mouse_input(&delta_time, &mut camera_angle);
        }

        next_frame().await;
    }
}
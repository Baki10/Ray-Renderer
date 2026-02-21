
use macroquad::{prelude::*};
mod ray_casting;
mod utils;

static SCREEN_WIDTH: f32 = 1600.0;
static SCREEN_HEIGHT: f32 = 900.0;

static WIDTH: f32 = 9.0;
static HEIGHT: f32 = 9.0;

pub struct Player {
    position: Vec2,
    camera_angle: f32,
    ray_resolution: f32,
    fov: f32
}

#[macroquad::main("MyGame")]
async fn main() {

    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut player: Player = Player {
        position: Vec2::new(WIDTH/2., HEIGHT/2.),
        camera_angle: 0.,
        ray_resolution: 200.,
        fov: 3.1415/3.
    };

    let mut mouse_lock: bool = false;
    let delta_time: f32 = get_frame_time();

    let wall_texture: Texture2D = load_texture("src/wall_texture.png").await.unwrap();


    loop {
        clear_background(BLACK);
        
        draw_rectangle(0.0, SCREEN_HEIGHT/2.0, SCREEN_WIDTH, SCREEN_HEIGHT/2.0, Color::new(0.15, 0.15, 0.15, 1.));
        ray_casting::scatter_rays(&player, &wall_texture);

        utils::update_key_input(&mut player, &mut mouse_lock);
        if mouse_lock {
            utils::update_mouse_input(&delta_time, &mut player);
        }

        next_frame().await;
    }
}
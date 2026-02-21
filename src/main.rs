
use macroquad::{prelude::*};
mod ray_casting;
mod utils;

pub struct Player {
    position: Vec2,
    camera_angle: f32,
    ray_resolution: f32,
    fov: f32
}

#[macroquad::main("MyGame")]
async fn main() {

    request_new_screen_size(utils::SCREEN_WIDTH, utils::SCREEN_HEIGHT);
    let mut player: Player = Player {
        position: Vec2::new(utils::WIDTH/2., utils::HEIGHT/2.),
        camera_angle: 0.,
        ray_resolution: 200.,
        fov: 3.1415/3.
    };

    let mut mouse_lock: bool = false;
    let mut show_fps: bool = false;
    let delta_time: f32 = get_frame_time();

    let wall_texture: Texture2D = load_texture("src/wall_texture.png").await.unwrap();


    loop {
        clear_background(BLACK);
        
        draw_rectangle(0.0, utils::SCREEN_HEIGHT/2.0, utils::SCREEN_WIDTH, utils::SCREEN_HEIGHT/2.0, Color::new(0.15, 0.15, 0.15, 1.));

        ray_casting::scatter_rays(&player, &wall_texture);

        if show_fps {
            draw_fps();
        }

        utils::update_key_input(&mut player, &mut mouse_lock, &mut show_fps);
        if mouse_lock {
            utils::update_mouse_input(&delta_time, &mut player);
        }

        next_frame().await;
    }
}
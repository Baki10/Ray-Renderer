use macroquad::{prelude::*};
use crate::{Player, utils as utils};

static SCREEN_WIDTH: f32 = 1600.0;
static SCREEN_HEIGHT: f32 = 900.0;

static VIEW_DISTANCE: f32 = 9.0;

struct Ray {
    position: Vec2,
    step: Vec2
}

impl Ray {
    fn next_step(&mut self) {
        self.position.x += self.step.x;
        self.position.y += self.step.y;
    }
}

pub fn scatter_rays(player: &Player, wall_texture: &Texture2D) {
    
    let segment_width: f32 = SCREEN_WIDTH/player.ray_resolution;

    for ray_index in 0..player.ray_resolution as usize {

        let angle_segment: f32 = player.fov/player.ray_resolution;
        let angle_offset: f32 = (ray_index as f32 - player.ray_resolution/2.0) * angle_segment;
        let ray_angle: f32 = player.camera_angle + angle_offset;


        let (distance, texture_displacement): (f32, f32) = ray(&player.position, &ray_angle);
        let normal_distance: f32 = distance*f32::cos(angle_offset);

        if normal_distance <= 0.0 {
            continue;
        }

        let distance_ratio: f32 = VIEW_DISTANCE/normal_distance;
        let segment_height: f32 = SCREEN_HEIGHT/7.0*distance_ratio;

        let texture_position: Vec2 = Vec2 {
            x: (ray_index as f32)*segment_width,
            y: (SCREEN_HEIGHT - segment_height)/2.0,
        };

        let row_color: Color = Color {
            r: 1.5/(normal_distance),
            g: 1.5/(normal_distance), 
            b: 1.5/(normal_distance), 
            a: 1.0,
        };

        draw_texture_sample(&wall_texture, texture_position, row_color, texture_displacement, segment_height, segment_width);

    }

}


pub fn ray(position: &Vec2, ray_angle: &f32) -> (f32, f32) {
 
    let mut iterations: f32 = 150.;

    let mut ray: Ray = Ray {
        position: *position,
        step: Vec2 {
            x: f32::cos(*ray_angle) * (VIEW_DISTANCE / iterations),
            y: f32::sin(*ray_angle) * (VIEW_DISTANCE / iterations)
        }

    };

    let texture_displacement: f32 = draw_ray(&mut ray, &mut iterations);

    if ray.position.x < 0. {
        return (-1., -1.);
    }

    let dx: f32 = position.x - ray.position.x;
    let dy: f32 = position.y - ray.position.y;
    let ray_distance: f32 = f32::sqrt(dx*dx + dy*dy);

    return (ray_distance, texture_displacement);
}

fn draw_ray(ray: &mut Ray, iterations: &mut f32) -> f32 {

    if *iterations <= 0. {
        ray.position.x = -1000.0;
        return -100.;
    }

    let (is_wall, wall_position): (bool, Vec2) = utils::is_wall(&ray.position);

    if is_wall {
        let wall_normal: Vec2 = get_wall_normal(&ray.position, &wall_position);
        let displacement: f32 = get_texture_displacement(wall_normal, &ray.position, wall_position);
        return displacement;
    }

    ray.next_step();

    *iterations-=1.;
    return draw_ray(ray, iterations);
}

fn draw_texture_sample(texture: &Texture2D, position: Vec2, color: Color, displacement: f32, texture_height: f32, texture_width: f32) {
    draw_texture_ex(&texture, position.x, position.y, color, DrawTextureParams { 
        dest_size: Some(vec2(texture_width, texture_height)), source: Some(Rect::new(displacement*texture.width(), 0., texture_width, texture.height())),..Default::default() });
}

fn get_texture_displacement(wall_normal: Vec2, ray_position: &Vec2, wall_position: Vec2) -> f32 {
    let mut displacement: f32 = 0.;

    if wall_normal.x == 0. {
        displacement = ray_position.x-wall_position.x;
    } else if wall_normal.y == 0. {
        displacement = ray_position.y-wall_position.y;
    }

    return displacement;
}

fn get_wall_normal(ray_position: &Vec2, wall_position: &Vec2) -> Vec2 {

    let mut normal: Vec2 = Vec2::new(0., 0.);

    let distance_x: f32 = ray_position.x - (wall_position.x + 0.5);
    let distance_y: f32 = ray_position.y - (wall_position.y + 0.5);

    if distance_x.abs() > distance_y.abs() {
        normal.x = distance_x/distance_x.abs();
    } else {
        normal.y = distance_y/distance_y.abs();
    }

    return normal;
}
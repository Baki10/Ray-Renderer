use macroquad::{prelude::*};
use crate::utils as utils;

static SCREEN_WIDTH: f32 = 1600.0;
static SCREEN_HEIGHT: f32 = 900.0;

static VIEW_DISTANCE: f32 = 9.0;

pub fn scatter_rays(x: &f32, y: &f32, camera_angle: &f32, fov: &f32, x_resolution: &f32, wall_texture: &Texture2D) {

    let w: f32 = SCREEN_WIDTH/x_resolution;
    for index in 0..*x_resolution as usize {

        let angle_offset: f32 = (index as f32 - ((*x_resolution/2.0) as i32) as f32)*fov/(*x_resolution);
        let ray_angle: f32 = *camera_angle + angle_offset;
        let ray_data: (f32, f32) = ray(&x, &y, &ray_angle);
        let distance: f32 = ray_data.0*f32::cos(angle_offset);

        if distance <= 0.0 {
            continue;
        }

        let distance_ratio: f32 = VIEW_DISTANCE/distance;
        let h: f32 = SCREEN_HEIGHT/7.0*distance_ratio;

        let pos_x: f32 = (index as f32)*w;
        let pos_y: f32 = (SCREEN_HEIGHT - h)/2.0;

        let row_color: Color = Color::new(1.5/(distance), 1.5/(distance), 1.5/(distance), 1.0);

        //draw_rectangle(pos_x, pos_y, w, h, row_color);
        draw_texture_sample(&wall_texture, pos_x, pos_y, row_color, ray_data.1, h, w);

    }

}


pub fn ray(x: &f32, y: &f32, ray_angle: &f32) -> (f32, f32) {
    let mut ray_x: f32 = *x;
    let mut ray_y: f32 = *y;
    
    let iterations: i32 = 150;

    let ray_dx: f32 = f32::cos(*ray_angle)*(VIEW_DISTANCE/(iterations as f32));
    let ray_dy: f32 = f32::sin(*ray_angle)*(VIEW_DISTANCE/(iterations as f32));

    let texture_displacement: f32 = draw_ray(&mut ray_x, &mut ray_y, &ray_dx, &ray_dy, 0, &iterations);


    //draw_ray(&mut ray_x, &mut ray_y, &ray_dx, &ray_dy, 0, &iterations);
    if ray_x < 0.0 {
        return (-1., -1.);
    }

    ray_x = *x-ray_x;
    ray_y = *y-ray_y;


    return (f32::sqrt(ray_x*ray_x + ray_y*ray_y), texture_displacement);
}

fn draw_ray(ray_x: &mut f32, ray_y: &mut f32, ray_dx: &f32, ray_dy: &f32, iteration_index: i32, iterations: &i32) -> f32 {
    if iteration_index > *iterations {
        *ray_x = - 1000.0;
        return -100.;
    }

    let wall_data: (bool, f32, f32) = utils::is_wall(ray_x, ray_y);



    if wall_data.0 {
        let wall_normal: (f32, f32) = get_wall_normal(&ray_x, &ray_y, &wall_data.1, &wall_data.2);
        let displacement: f32 = get_texture_displacement(&wall_normal, ray_x, ray_y, &wall_data.1, &wall_data.2);
        return displacement;
    }

    *ray_x+=*ray_dx;
    *ray_y+=*ray_dy;

    return draw_ray(ray_x, ray_y, ray_dx, ray_dy, iteration_index+1, iterations);
}

fn draw_texture_sample(texture: &Texture2D, x: f32, y: f32, color: Color, displacement: f32, texture_height: f32, texture_width: f32) {
    draw_texture_ex(&texture, x, y, color, DrawTextureParams { 
        dest_size: Some(vec2(texture_width, texture_height)), source: Some(Rect::new(displacement*texture.width(), 0., texture_width, texture.height())),..Default::default() });
}

fn get_texture_displacement(wall_normal: &(f32,f32), ray_x: &f32, ray_y: &f32, wall_x: &f32, wall_y: &f32) -> f32 {
    let mut displacement: f32 = 0.;

    if wall_normal.0 == 0. {
        displacement = *ray_x-(*wall_x);
    } else if wall_normal.1 == 0. {
        displacement = *ray_y-(*wall_y);
    }

    return displacement;
}

fn get_wall_normal(ray_x: &f32, ray_y: &f32, wall_x: &f32, wall_y: &f32) -> (f32,f32) {

    let mut normal: (f32, f32) = (0.,0.);

    let distance_x: f32 = *ray_x-(*wall_x+0.5);
    let distance_y: f32 = *ray_y-(*wall_y+0.5);

    if distance_x.abs() > distance_y.abs() {
        normal.0 = distance_x/distance_x.abs();
    } else {
        normal.1 = distance_y/distance_y.abs();
    }

    return normal;
}
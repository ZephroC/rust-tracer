use nalgebra::Vector3;

use colour::RGB;
use geom::Drawable;

use crate::Resolution;
use crate::tracer::colour::Material;
use std::cmp::max;

pub mod geom;
pub mod colour;

pub struct Camera {
    pub pos: Vector3<f64>,
    pub dir: Vector3<f64>,
    pub fov: f64,
}

pub struct Ray {
    orig: Vector3<f64>,
    dir: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Vector3<f64>, dir: Vector3<f64>) -> Ray {
        Ray {
            orig: orig,
            dir: dir.normalize(),
        }
    }

    pub fn point_along(&self, dist: f64) -> Vector3<f64> {
        (self.dir * dist) + self.orig
    }
}

pub struct PointLight {
    pub pos: Vector3<f64>,
    pub colour: RGB,
}

pub struct SceneState {
    pub geom: Vec<Box<dyn Drawable>>,
    pub point_lights: Vec<PointLight>,
    pub camera: Camera,
}

struct HitInformation<'a> {
    dist: f64,
    material: &'a Material,
    point: Vector3<f64>,
    normal: Vector3<f64>,
}

impl SceneState {
    pub fn rasterise(&self, frame: &mut Vec<u8>, res: &Resolution) {
        let width = res.width;
        let height = res.height;
        let (top_left, x_stride, y_stride) = screen_to_coord_stride(width as f64, height as f64, &self.camera);
        for y in 0..height {
            let y_pixel_pos = &top_left + (&y_stride * (y as f64));
            for x in 0..width {
                let pixel_pos = y_pixel_pos + (&x_stride * (x as f64));
                // let pixel_pos = screen_to_coord(x, y, width as f64, height as f64, &self.camera);
                let ray = Ray::new(
                    self.camera.pos,
                    pixel_pos - &self.camera.pos,
                );

                let mut hit_info: Option<HitInformation> = None;

                for object in &self.geom {
                    match object.intersect(&ray) {
                        (hit, _) if hit < 0.0 => {}
                        (dist, Some(normal)) => {
                            match hit_info {
                                None => {
                                    let hit_point = ray.point_along(dist);
                                    hit_info = Some(HitInformation {
                                        dist: dist,
                                        material: object.material_at(&hit_point),
                                        point: hit_point,
                                        normal: normal,
                                    });
                                }
                                Some(prev_hit_info) if dist < prev_hit_info.dist => {
                                    let hit_point = ray.point_along(dist);
                                    hit_info = Some(HitInformation {
                                        dist: dist,
                                        material: object.material_at(&hit_point),
                                        point: hit_point,
                                        normal: normal,
                                    });
                                }
                                Some(_) => {}
                            }
                        }
                        (_, _) => {
                            println!("Some weird error happened with object at {}, {}", x, y);
                        }
                    }
                }

                let draw_colour = match hit_info {
                    Some(info) => { self.colour_for_hit(info) }
                    None => None
                };

                let draw_colour = match draw_colour {
                    Some(actual_colour) => { actual_colour }
                    None => RGB { r: 0, g: 0, b: 0 }
                };

                let array_loc: usize = ((x as u32 + (y as u32 * width as u32)) * 4) as usize;
                //Need a better memcopy though kind of unsafe
                frame[array_loc] = draw_colour.r;
                frame[array_loc + 1] = draw_colour.g;
                frame[array_loc + 2] = draw_colour.b;
                frame[array_loc + 3] = 255;
            }
        }
    }

    fn colour_for_hit(&self, hit_info: HitInformation) -> Option<RGB> {
        let ambient = 0.1;
        let mut colour_pts:Vec<RGB> = Vec::new();
        for light in &self.point_lights {
            let mut in_shadow  = false;
            let hit_to_light: Vector3<f64> = light.pos - hit_info.point;
            let light_dist = hit_to_light.norm();
            let new_ray: Ray = Ray::new(Vector3::from_data(hit_info.point.data), hit_to_light);
            for object in &self.geom {
                match object.intersect(&new_ray) {
                    (dist, Some(norm)) if dist > 0.00005 && dist < light_dist => {

                        // println!("hit!");
                        let next_hit_point = new_ray.point_along(dist);

                        // // println!("Hit object: {:?}",object);
                        // println!("Distance from hit: {}", dist);
                        // println!("Distance from light: {}", light_dist);
                        // println!("light: {},{},{}", light.pos[0], light.pos[1], light.pos[2]);
                        // println!("hit: {},{},{}", hit_info.point[0], hit_info.point[1], hit_info.point[2]);
                        // println!("next_hit_point: {},{},{}", next_hit_point[0], next_hit_point[1], next_hit_point[2]);
                        //
                        // println!("ray point: {},{},{}", new_ray.orig[0], new_ray.orig[1], new_ray.orig[2]);
                        // println!("ray dir: {},{},{}", new_ray.dir[0], new_ray.dir[1], new_ray.dir[2]);
                        in_shadow = true;
                        break;
                    }
                    (_, _) => {}
                }
            }
            if in_shadow {
                let ambient_colour:RGB = hit_info.material.rgb.multiply(ambient);
                colour_pts.push(ambient_colour);
            } else {
                let ambient_colour:RGB = hit_info.material.rgb.multiply(ambient);
                let dot_n = hit_to_light.normalize().dot(&hit_info.normal);

                // if hit_info.point[1] > 2.9 && hit_info.point[2] > 5.9 && hit_info.point[2] < 6.1 {
                //     println!("dot_n: {}",dot_n);
                //     println!("hit point: {:?}",hit_info.point);
                // }

                let diff_frac = if dot_n > 0.0 {
                    dot_n * hit_info.material.diffuse
                } else {
                  0.0
                };
                let diffuse_colour:RGB = hit_info.material.rgb.multiply(diff_frac);
                colour_pts.push(RGB{
                    r: ambient_colour.r + diffuse_colour.r,
                    g: ambient_colour.g + diffuse_colour.g,
                    b: ambient_colour.b + diffuse_colour.b
                });
            }
        }
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let size = colour_pts.len() as u8;
        for colour in colour_pts {
            r += colour.r;
            g += colour.g;
            b += colour.b;
        }
        r = r / size as u8;
        g = g / size as u8;
        b = b / size as u8;
        return Some(RGB {
            r,
            g,
            b,
        });
    }
}

fn screen_to_coord_stride(width: f64, height: f64, camera: &Camera) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
    let screen_pos: Vector3<f64> = &camera.dir + &camera.pos;
    // println!("Screen pos: {:?}",screen_pos);
    let vp_right: Vector3<f64> = Vector3::new(0.0, 1.0, 0.0).cross(&camera.dir).normalize();
    // println!("Right: {:?}",vp_right);
    let vp_up: Vector3<f64> = camera.dir.cross(&vp_right).normalize();
    // println!("Up: {:?}",vp_up);
    let half_width_geom = camera.dir.magnitude() * (camera.fov.to_radians() / 2.0).tan();
    // println!("Half width: {}",half_width_geom);
    let half_height_geom = half_width_geom * height / width;
    // println!("Half height: {}",half_height_geom);
    let left_side_geom: Vector3<f64> = screen_pos - (vp_right * half_width_geom);
    // println!("Left side middle pos: {:?}",left_side_geom);
    let x_pixel_stride_right: Vector3<f64> = &vp_right * (half_width_geom / width * 2.0);
    // println!("Vector to add per x pixel: {:?}",x_pixel_stride_right);
    //Doing a minus here
    let y_pixel_stride_down: Vector3<f64> = &vp_up * ((half_height_geom * -2.0) / height);
    // println!("Vector to add per y pixel: {:?}",y_pixel_stride_down);
    let top_left: Vector3<f64> = left_side_geom + vp_up * half_height_geom;
    // println!("Vector top left: {:?}",top_left);
    (top_left, x_pixel_stride_right, y_pixel_stride_down)
}


#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use approx;
    use nalgebra::Vector3;

    use crate::tracer::{Camera, screen_to_coord_stride};

    #[test]
    fn test_screen_coords() {
        let at_orig_cam = Camera {
            pos: Vector3::new(0.0, 0.0, -1.0),
            dir: Vector3::new(0.0, 0.0, 1.0),
            fov: 90.0,
        };
        let (top_left, x_pixel, y_pixel) = screen_to_coord_stride(100.0, 100.0, &at_orig_cam);
        let angle: f64 = 45.0;
        let tan_of_a = angle.to_radians().tan();
        approx::assert_ulps_eq!(-1.0 / tan_of_a, top_left[0], max_ulps =  5);
        approx::assert_ulps_eq!(1.0 / tan_of_a, top_left[1]);
        approx::assert_ulps_eq!(0.0, top_left[2]);

        approx::assert_ulps_eq!( 2.0 / tan_of_a, x_pixel[0] * 100.0);
        approx::assert_ulps_eq!( 0.0, x_pixel[1] * 100.0);
        approx::assert_ulps_eq!( 0.0, x_pixel[2] * 100.0);

        approx::assert_ulps_eq!( 0.0, y_pixel[0] * 100.0);
        approx::assert_ulps_eq!( -2.0 / tan_of_a, y_pixel[1] * 100.0);
        approx::assert_ulps_eq!( 0.0, y_pixel[2] * 100.0);

        let first_cam = Camera {
            pos: Vector3::new(0.0, 0.0, 0.0),
            dir: Vector3::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0),
            fov: 90.0,
        };

        let (top_left, x_pixel, y_pixel) = screen_to_coord_stride(100.0, 100.0, &first_cam);
        approx::assert_ulps_eq!(0.0, top_left[0]);
        approx::assert_ulps_eq!(2.0 * angle.to_radians().cos(), top_left[1]);
        approx::assert_ulps_eq!(1.0, top_left[2]);

        approx::assert_ulps_eq!( 0.0, x_pixel[0] * 100.0);
        approx::assert_ulps_eq!( 0.0, x_pixel[1] * 100.0);
        approx::assert_ulps_eq!( -2.0, x_pixel[2] * 100.0, max_ulps = 5);

        let cos_of_a = angle.to_radians().cos();
        approx::assert_ulps_eq!( 2.0 * cos_of_a, y_pixel[0] * 100.0);
        approx::assert_ulps_eq!( -2.0 * cos_of_a, y_pixel[1] * 100.0);
        approx::assert_ulps_eq!( 0.0, y_pixel[2] * 100.0);
    }
}
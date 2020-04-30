use nalgebra::Vector3;

use colour::RGB;
use geom::Drawable;
use rand::prelude::*;

use crate::Resolution;
use crate::tracer::colour::Material;

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
    pub intensity:f64
}

pub struct SceneState {
    pub geom: Vec<Box<dyn Drawable>>,
    pub point_lights: Vec<PointLight>,
    pub camera: Camera,
    pub ambient: f64,
    pub background_colour:RGB
}

struct HitInformation<'a> {
    dist: f64,
    material: &'a Material,
    point: Vector3<f64>,
    normal: Vector3<f64>,
}

impl SceneState {
    pub fn rasterise(&self, frame: &mut Vec<u8>, res: &Resolution, samples:u8) {
        let width = res.width;
        let height = res.height;
        let (top_left, x_stride, y_stride) = screen_to_coord_stride(width as f64, height as f64, &self.camera);
        //find better seed for this later
        let mut rng = rand::thread_rng();

        for y in 0..height {
            let y_pixel_pos = &top_left + (&y_stride * (y as f64));
            for x in 0..width {
                // let pixel_pos = screen_to_coord(x, y, width as f64, height as f64, &self.camera);
                let mut r_total:u32 = 0;
                let mut g_total:u32 = 0;
                let mut b_total:u32 = 0;

                for sample in 0..samples {
                    let rand_x:f64 = rng.gen();
                    let rand_y:f64 = rng.gen();

                    let pixel_pos = if sample == 0 {
                        y_pixel_pos + (&x_stride * (x as f64))
                    } else {
                        y_pixel_pos + (&x_stride * (x as f64)) + (&x_stride * rand_x) + (&y_stride * rand_y)
                    };


                    let ray = Ray::new(
                        self.camera.pos,
                        pixel_pos - &self.camera.pos,
                    );
                    let draw_colour = self.cast_ray(ray);
                    r_total += draw_colour.r as u32;
                    g_total += draw_colour.g as u32;
                    b_total += draw_colour.b as u32;
                }
                let array_loc: usize = ((x as u32 + (y as u32 * width as u32)) * 4) as usize;
                //Need a better memcopy though kind of unsafe
                frame[array_loc] = (r_total / samples as u32) as u8;
                frame[array_loc + 1] = (g_total / samples as u32) as u8;
                frame[array_loc + 2] = (b_total / samples as u32) as u8;
                frame[array_loc + 3] = 255;
            }
        }
    }


    fn cast_ray(&self, ray:Ray) -> RGB {
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
                    println!("Some weird error happened with object");
                }
            }
        }

        //TODO this needs a tidy up, 2 matches like this. Blergh
        let draw_colour = match hit_info {
            Some(info) => { self.colour_for_hit(info, ray) }
            None => None
        };

        let draw_colour = match draw_colour {
            Some(actual_colour) => { actual_colour }
            None => RGB {
                r: self.background_colour.r,
                g: self.background_colour.g,
                b: self.background_colour.b
            }
        };
        return draw_colour;
    }

    fn colour_for_hit(&self, hit_info: HitInformation, ray:Ray) -> Option<RGB> {
        let ambient = self.ambient;
        let ambient_colour:RGB = hit_info.material.rgb.multiply(ambient);
        let mut colour_pts:Vec<RGB> = Vec::new();
        let reflect =  ray.dir - 2.0 * ray.dir.dot(&hit_info.normal) * hit_info.normal;

        for light in &self.point_lights {
            let mut in_shadow  = false;
            let hit_to_light: Vector3<f64> = light.pos - hit_info.point;
            let light_dist = hit_to_light.norm();
            let new_ray: Ray = Ray::new(Vector3::from_data(hit_info.point.data), hit_to_light);
            for object in &self.geom {
                match object.intersect(&new_ray) {
                    (dist, Some(_norm)) if dist > 0.00005 && dist < light_dist => {
                        // println!("hit!");
                        // let next_hit_point = new_ray.point_along(dist);
                        in_shadow = true;
                        break;
                    }
                    (_, _) => {}
                }
            }
            if !in_shadow {
                let dot_n = hit_to_light.normalize().dot(&hit_info.normal);
                let diff_frac = if dot_n > 0.0 {
                    dot_n * hit_info.material.diffuse * light.intensity
                } else {
                  0.0
                };
                let light_reflect =  hit_to_light.normalize() - 2.0 * dot_n * &hit_info.normal;
                let spec_frac:f64 = (light_reflect.dot(&ray.dir.normalize())).max(0.0).powf(hit_info.material.specular_exp) * hit_info.material.specular * light.intensity;
                let spec_colour:RGB = hit_info.material.rgb.multiply(spec_frac);
                // let diffuse_colour:RGB = RGB::new(0,0,0);
                let diffuse_colour:RGB = hit_info.material.rgb.multiply(diff_frac);
                colour_pts.push(RGB{
                    r: std::cmp::min(spec_colour.r + diffuse_colour.r, 255),
                    g: std::cmp::min(spec_colour.g + diffuse_colour.g, 255),
                    b: std::cmp::min(spec_colour.b + diffuse_colour.b, 255)
                });
            }
        }
        let mut r:u32 = 0;
        let mut g:u32 = 0;
        let mut b:u32 = 0;
        let size = colour_pts.len() as u8;
        for colour in colour_pts {
            r += colour.r as u32;
            g += colour.g as u32;
            b += colour.b as u32;
        }
        if size > 0 {
            r = r / size as u32;
            g = g / size as u32;
            b = b / size as u32;
        }
        return Some(RGB {
            r: std::cmp::min(r + ambient_colour.r as u32, 255) as u8,
            g: std::cmp::min(g + ambient_colour.g as u32, 255) as u8,
            b: std::cmp::min(b + ambient_colour.b as u32, 255) as u8
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
use crate::sub_modules::config::Resolution;
use nalgebra::Vector3;

pub struct Camera {
    pub pos: Vector3<f64>,
    pub dir: Vector3<f64>,
    pub fov: f64,
    pub screen: Screen,
}

pub struct Screen {
    pub height: f64,
    pub width: f64,
    pub distance: f64,
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Ray {
    orig: Vector3<f64>,
    vec: Vector3<f64>,
}

pub struct Sphere {
    pub pos: Vector3<f64>,
    pub radius: f64,
    pub colour: RGB,
}

pub struct SceneState {
    pub geom: Vec<Box<dyn Intersects>>,
    pub camera: Camera,
}

pub trait Intersects {
    fn intersect(&self, ray: &Ray) -> (f64, &RGB);
}

impl Intersects for Sphere {
    fn intersect(&self, ray: &Ray) -> (f64, &RGB) {
        let orig_to_loc = ray.orig - &self.pos;
        // normal quadratic abc
        let a = ray.vec.dot(&ray.vec);
        let b = 2.0 * orig_to_loc.dot(&ray.vec);
        let c = orig_to_loc.dot(&orig_to_loc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        // Reference C for later
        // if(discriminant < 0){
        //     return -1.0;
        // }
        // else{
        //     return (-b - sqrt(discriminant)) / (2.0*a);
        // }
        if discriminant < 0.0 {
            return (-1.0, &RGB { r: 0, g: 0, b: 0 });
        } else {
            let dist = (-b - discriminant.sqrt()) / 2.0 * a;
            return ( dist, &self.colour);
        }
    }
}

fn screen_to_coord(x: u16, y: u16, width: f64, height: f64, camera: &Camera) -> Vector3<f64> {
    let pixel_width: f64 = camera.screen.width / width;
    let pixel_height: f64 = camera.screen.height / height;
    let pixel_x: f64 = x as f64 * pixel_width - camera.screen.width / 2.0;
    let pixel_y: f64 = y as f64 * pixel_height - camera.screen.height / 2.0;
    return nalgebra::Vector3::new(pixel_x, pixel_y, camera.screen.distance);
}

impl SceneState {
    pub fn rasterise(&self, frame: &mut Vec<u8>, res: &Resolution) {
        let width = res.width;
        let height = res.height;
        for x in 0..width {
            for y in 0..height {
                let pixelPos = screen_to_coord(x, y, width as f64, height as f64, &self.camera);
                let ray = Ray {
                    orig: self.camera.pos.clone(),
                    vec: pixelPos.normalize(),
                };
                let mut pixel_colour = RGB { r: 0, g: 0, b: 0 };
                for object in &self.geom {
                    let (hit, colour) = object.intersect(&ray);
                    if hit  > 0.0{
                        let normalise = (ray.vec.normalize() * hit) - &self.camera.pos;
                        let attenuate = normalise.normalize().dot(&self.camera.dir.normalize());
                        pixel_colour.r = (attenuate * colour.r as f64) as u8;
                        pixel_colour.g = (attenuate * colour.g as f64) as u8;
                        pixel_colour.b = (attenuate * colour.b as f64) as u8;
                    }
                }
                let array_loc: usize = ((x as u32 + (y as u32 * width as u32)) * 4) as usize;
                //Need a bettery memcopy though kind of unsafe
                frame[array_loc] = pixel_colour.r;
                frame[array_loc + 1] = pixel_colour.g;
                frame[array_loc + 2] = pixel_colour.b;
                frame[array_loc + 3] = 255;
            }
        }
    }
}

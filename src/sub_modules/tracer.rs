use crate::sub_modules::window::MainState;
use nalgebra::Vector3;

pub struct Camera {
    pub pos: Vector3<f64>,
    pub dir: Vector3<f64>,
    pub fov: f64,
    pub frustrum: Frustrum
}

pub struct Frustrum {
    pub height: f64,
    pub width: f64,
    pub distance: f64
}

pub struct RGBA {
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub a:u8
}

pub struct Ray {
    orig: Vector3<f64>,
    vec: Vector3<f64>
}

pub struct Sphere {
    pub pos: Vector3<f64>,
    pub radius: f64,
    pub colour:RGBA
}

pub trait Intersects {
    fn intersect(&self, ray:&Ray) -> (bool,&RGBA);
}

impl Intersects for Sphere{
    fn intersect(&self, ray:&Ray) -> (bool,&RGBA) {
        let orig_to_loc = ray.orig - &self.pos;
        // normal quadratic abc
        let a = ray.vec.dot(&ray.vec);
        let b = 2.0 * orig_to_loc.dot(&ray.vec);
        let c = orig_to_loc.dot(&orig_to_loc) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;
// Reference C for later
        // if(discriminant < 0){
        //     return -1.0;
        // }
        // else{
        //     return (-b - sqrt(discriminant)) / (2.0*a);
        // }
        if discriminant < 0.0 {
           return (false,&RGBA{
               r: 0,
               g: 0,
               b: 0,
               a: 0,
           })
        }  else {
            return (true,&self.colour)
        }
    }
}

fn screenToCoord(x:u16, y:u16, width:f64,height:f64,camera:&Camera) -> Vector3<f64> {
        let pixel_width:f64 = camera.frustrum.width / width;
        let pixel_height:f64 = camera.frustrum.height / height;
        let pixel_x:f64 = x as f64 * pixel_width - camera.frustrum.width / 2.0;
        let pixel_y:f64 = y as f64 * pixel_height - camera.frustrum.height/2.0;
        return nalgebra::Vector3::new(pixel_x,pixel_y,camera.frustrum.distance);
    }


pub fn rasterise(state: &mut MainState) {
    let width = state.resolution.width;
    let height = state.resolution.height;
    for x in 0..width {
        for y in 0..height {
            let pixelPos = screenToCoord(x,y,width as f64,height as f64, &state.camera);
            let ray = Ray {
                orig: state.camera.pos.clone(),
                vec: pixelPos.normalize()
            };
            let mut pixelColour = RGBA{
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            };
            for object in &state.scene {
                let (hit,colour) = object.intersect(&ray);
                if hit {
                    pixelColour.r = colour.r;
                    pixelColour.g = colour.g;
                    pixelColour.b = colour.b;
                    pixelColour.a = colour.a;
                }
            }
            let array_loc:usize = ((x as u32 + (y as u32 * width as u32)) * 4) as usize;
            //Need a bettery memcopy though kind of unsafe
            state.frame[array_loc] = pixelColour.r;
            state.frame[array_loc+1] = pixelColour.g;
            state.frame[array_loc+2] = pixelColour.b;
            state.frame[array_loc+3] = pixelColour.a;
        }
    }
}


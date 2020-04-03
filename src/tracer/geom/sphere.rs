use nalgebra::Vector3;

use crate::tracer::colour::{RGB, Material};
use crate::tracer::geom::{Intersects, MaterialAt, Drawable};
use crate::tracer::Ray;

pub struct Sphere {
    pub pos: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(pos: Vector3<f64>, radius: f64, colour: RGB) -> Sphere {
        Sphere {
            pos,
            radius,
            material: Material {
                rgb: colour,
                diffuse: 1.0
            }
        }
    }
}

impl MaterialAt for Sphere {
    fn material_at(&self, _hit:&Vector3<f64>) -> &Material {
        &self.material
    }
}

impl Intersects for Sphere {
    fn intersect(&self, ray: &Ray) -> (f64,  Option<Vector3<f64>>) {
        // the vector from the ray origin (camera origin) to the centre of the sphere.
        let orig_to_loc = &ray.orig - &self.pos;
        // normal quadratic abc
        let a = ray.dir.dot(&ray.dir);
        let b = 2.0 * orig_to_loc.dot(&ray.dir);
        let c = orig_to_loc.dot(&orig_to_loc) - self.radius*self.radius;
        let discriminant =  b*b - 4.0*a*c;
        return if discriminant < 0.0 {
            (-1.0,  None)
        } else {
            let numerator = -b - discriminant.sqrt();
            if numerator > 0.0 {
                let dist = numerator / (2.0 * a);

                (dist, Some((&ray.dir * dist - &self.pos).normalize()))
            }
            else {
                (-1.0, None)
            }
        }
    }
}

impl Drawable for Sphere {}


#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use approx;
    use crate::tracer::colour::RGB;
    use crate::tracer::geom::sphere::Sphere;
    use crate::tracer::geom::Intersects;
    use crate::tracer::Ray;


    #[test]
    fn test_intersects() {
        let sphere = Sphere::new(Vector3::new(0.0,0.0,3.0), 1.0, RGB{r:0,g:0,b:0});
        let ray = Ray{
            orig: Vector3::new(0.0,0.0,0.0),
            dir: Vector3::new(0.0, 0.0, 1.0)
        };
        let (dist,norm) = sphere.intersect(&ray);
        approx::assert_ulps_eq!(dist, 2.0, max_ulps =  3);
        approx::assert_ulps_eq!(0.0, norm.unwrap()[0],max_ulps = 3);
        approx::assert_ulps_eq!(0.0, norm.unwrap()[1],max_ulps = 3);
        approx::assert_ulps_eq!(-1.0, norm.unwrap()[2],max_ulps = 3);

    }

}
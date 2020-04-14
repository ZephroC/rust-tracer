use nalgebra::Vector3;

use crate::tracer::colour::{RGB, Material};
use crate::tracer::geom::{Intersects, MaterialAt, Drawable};
use crate::tracer::Ray;

pub struct Plane {
    pub point: Vector3<f64>,
    pub norm: Vector3<f64>,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vector3<f64>, norm: Vector3<f64>, colour: RGB) -> Plane {
        Plane {
            point,
            norm,
            material: Material {
                rgb: colour,
                diffuse: 0.6
            }
        }
    }
}

impl MaterialAt for Plane {
    fn material_at(&self, _hit:&Vector3<f64>) -> &Material {
        &self.material
    }
}

impl Intersects for Plane {
    fn intersect(&self, ray: &Ray) -> (f64,  Option<Vector3<f64>>) {
        let orig_to_point = &self.point - &ray.orig;
        let denom = self.norm.dot(&ray.dir);
        let d = orig_to_point.dot(&self.norm) / denom;
        if d > 0.0001 {
            (d, Some(self.norm.clone()))
        } else {
            (-1.0,  None)
        }
    }
}

impl Drawable for Plane {}


#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use approx;
    use crate::tracer::colour::RGB;
    use crate::tracer::geom::plane::Plane;
    use crate::tracer::geom::Intersects;
    use crate::tracer::Ray;


    #[test]
    fn test_intersects() {
        let sphere = Plane::new(Vector3::new(0.0,0.0,3.0), Vector3::new(0.0,0.0,-1.0), RGB{r:0,g:0,b:0});
        let ray = Ray{
            orig: Vector3::new(0.0,0.0,0.0),
            dir: Vector3::new(0.0, 0.0, 1.0)
        };
        let (dist,norm) = sphere.intersect(&ray);
        approx::assert_ulps_eq!(dist, 3.0, max_ulps =  3);
        approx::assert_ulps_eq!(0.0, norm.unwrap()[0],max_ulps = 3);
        approx::assert_ulps_eq!(0.0, norm.unwrap()[1],max_ulps = 3);
        approx::assert_ulps_eq!(-1.0, norm.unwrap()[2],max_ulps = 3);
        approx::assert_ulps_eq!(1.0, norm.unwrap().magnitude() , max_ulps=3);
    }

}
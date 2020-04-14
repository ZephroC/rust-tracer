pub mod sphere;
pub mod plane;


use nalgebra::Vector3;

use crate::tracer::colour::Material;
use crate::tracer::Ray;


pub trait Intersects {
    fn intersect(&self, ray: &Ray) -> (f64, Option<Vector3<f64>>);
}

pub trait MaterialAt {
    fn material_at(&self, hit_point:&Vector3<f64>) -> &Material;
}

pub trait Drawable: MaterialAt + Intersects {}
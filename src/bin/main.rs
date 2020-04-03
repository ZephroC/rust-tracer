use std::env;

use ggez::GameResult;
use nalgebra::Vector3;
use serde_yaml::Value;

use rust_tracer::{window, config};
use rust_tracer::tracer::{Camera, SceneState, PointLight};
use rust_tracer::tracer::geom::sphere::Sphere;
use rust_tracer::tracer::geom::Drawable;
use rust_tracer::tracer::colour::RGB;

fn unwrap_xyz(xyz: &serde_yaml::Value) -> Vector3<f64> {
    nalgebra::Vector3::new(
        xyz["x"].as_f64().unwrap(),
        xyz["y"].as_f64().unwrap(),
        xyz["z"].as_f64().unwrap(),
    )
}

fn unwrap_rgb(rgb: &serde_yaml::Value) -> RGB {
    RGB {
        r: rgb["r"].as_u64().unwrap() as u8,
        g: rgb["g"].as_u64().unwrap() as u8,
        b: rgb["b"].as_u64().unwrap() as u8,
    }
}

pub fn main() -> GameResult {
    let args: Vec<String> = env::args().collect();
    let config = config::parse_args(args);
    let scene_file = std::fs::File::open(&config.filename)?;
    let deserialised: Value = serde_yaml::from_reader(&scene_file).unwrap();

    let camera = &deserialised["camera"];
    let camera = Camera {
        pos: unwrap_xyz(&camera["pos"]),
        dir: unwrap_xyz(&camera["dir"]),
        fov: camera["fov"].as_f64().unwrap()
    };
    let mut geom: Vec<Box<dyn Drawable>> = vec![];
    let mut point_lights: Vec<PointLight> = vec![];

    for sphere in deserialised["spheres"].as_sequence().unwrap() {
        geom.push(Box::new(Sphere::new (
            unwrap_xyz(&sphere["pos"]),
            sphere["radius"].as_f64().unwrap(),
            unwrap_rgb(&sphere["colour"]),
        )));
    }

    for point_light in deserialised["point_lights"].as_sequence().unwrap() {
        point_lights.push(PointLight {
            pos: unwrap_xyz(&point_light["pos"]),
            colour: unwrap_rgb(&point_light["colour"]),
        });
    }

    let scene = SceneState { geom, point_lights, camera };
    window::run(config.resolution,&scene)
}
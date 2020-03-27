use std::collections::BTreeMap;
use std::env;

use ggez::GameResult;
use nalgebra::Vector3;
use serde_yaml::Value;

use crate::sub_modules::tracer::{Camera, Intersects, SceneState, Screen, Sphere, RGB};
use crate::sub_modules::window::WindowState;
use crate::sub_modules::{config, window};

mod sub_modules;

struct SceneFile {
    camera: Camera,
    spheres: [Sphere],
}

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
        fov: camera["fov"].as_f64().unwrap(),
        screen: Screen {
            height: camera["screen"]["height"].as_f64().unwrap(),
            width: camera["screen"]["width"].as_f64().unwrap(),
            distance: camera["screen"]["distance"].as_f64().unwrap(),
        },
    };
    let mut geom: Vec<Box<dyn Intersects>> = vec![];

    for sphere in deserialised["spheres"].as_sequence().unwrap() {
        geom.push(Box::new(Sphere {
            pos: unwrap_xyz(&sphere["pos"]),
            radius: sphere["radius"].as_f64().unwrap(),
            colour: unwrap_rgb(&sphere["colour"]),
        }));
    }

    let scene = SceneState { geom, camera };
    window::run(&mut WindowState::new(config.resolution, &scene))
}

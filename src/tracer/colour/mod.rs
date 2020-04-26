//TODO I also want float 0.0-1.0 versions as switching between floats and u8s is annoying me.
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r:u8,g:u8,b:u8) -> RGB {
        RGB { r, g, b}
    }

    pub fn multiply(&self, fraction:f64) -> RGB {
        RGB {
            r: mult_channel(self.r,fraction),
            g: mult_channel(self.g,fraction),
            b: mult_channel(self.b,fraction),
        }
    }
}

fn mult_channel(chn:u8, fraction: f64 ) -> u8 {
    let normal = (chn as f64) / 255.0;
    let fractioned = normal * fraction * 255.0;
    fractioned as u8
}

pub struct Material {
    pub rgb:RGB,
    pub diffuse:f64,
    pub specular:f64
}
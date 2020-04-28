use std::convert::TryFrom;
use crate::Resolution;
use std::sync::RwLock;

pub struct FrameBuffer {
    res: Resolution,
    front_buffer: RwLock<Vec<u8>>,
    rear_buffer: RwLock<Vec<u8>>,
    swap: bool,
}

impl FrameBuffer {
    pub fn new(res: Resolution) -> FrameBuffer {
        let byte_stride = 4;
        let pixels: u32 = res.width as u32 * res.height as u32;
        let mut front_buffer: Vec<u8> = vec![255; usize::try_from(pixels * byte_stride).unwrap()];
        let mut rear_buffer: Vec<u8> = vec![255; usize::try_from(pixels * byte_stride).unwrap()];

        FrameBuffer {
            res,
            front_buffer: RwLock::new(front_buffer),
            rear_buffer: RwLock::new(rear_buffer),
            swap: true
        }
    }

    pub fn read_buffer(&self) -> &RwLock<Vec<u8>> {
        if self.swap {
            &self.front_buffer
        } else {
            &self.rear_buffer
        }
    }
    pub fn write_buffer(&self) -> &RwLock<Vec<u8>> {
        if self.swap {
            &self.rear_buffer
        } else {
            &self.front_buffer
        }
    }

    pub fn swap(&mut self) {
        self.swap = !self.swap;
    }
}
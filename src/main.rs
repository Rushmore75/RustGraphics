/*
    Oliver Atkinson
    Started in May, 2022
*/

pub mod window;
mod bounding_box;
pub mod peripherals;
pub mod vertex;
pub mod application;

fn main() {
    // used by wgpu
    env_logger::init();

    pollster::block_on(application::run());

}



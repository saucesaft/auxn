mod uxn;
mod system;
mod devices;
mod operations;

use nih_plug::prelude::*;

use talsnd::Gain;

fn main() {
    nih_export_standalone::<Gain>();
}
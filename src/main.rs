mod devices;
mod operations;
mod system;
mod uxn;

use nih_plug::prelude::*;

use auxn::Gain;

fn main() {
    nih_export_standalone::<Gain>();
}

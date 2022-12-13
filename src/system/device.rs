use arrayvec::ArrayVec;

#[derive(Clone)]
pub struct Device {
    // u: &mut UXN,
    data: ArrayVec::<u8, 16>,
    // dei: Option::<unsafe extern "C" fn(*mut Device, Uint8) -> Uint8>,
    // deo: Option::<unsafe extern "C" fn(*mut Device, Uint8) -> ()>,
}

impl Device {
    pub fn new() -> Self {
        Device { data: ArrayVec::<u8, 16>::new()}
    }
}
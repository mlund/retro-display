// Copyright 2024 Mikael Lund aka Wombat
//
// Licensed under the Apache license, version 2.0 (the "license");
// you may not use this file except in compliance with the license.
// You may obtain a copy of the license at
//
//     http://www.apache.org/licenses/license-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the license is distributed on an "as is" basis,
// without warranties or conditions of any kind, either express or implied.
// See the license for the specific language governing permissions and
// limitations under the license.

#![no_std]
#![feature(start)]

use core::panic::PanicInfo;
use embedded_graphics::{
    image::{Image, ImageRawLE},
    prelude::*,
};
use retro_display::c64::{PetsciiDisplay, VicIIPalette};

const LOGO_DATA: &[u8] = include_bytes!("eg-logo.raw");

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut display = PetsciiDisplay {};
    display.clear(VicIIPalette::Black).unwrap();

    let logo: ImageRawLE<VicIIPalette> = ImageRawLE::new(LOGO_DATA, 20);
    Image::new(&logo, Point::new(10, 0))
        .draw(&mut display)
        .unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

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
    mono_font::{ascii::FONT_5X8, MonoTextStyleBuilder},
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};
use retro_display::c64::{C64Color, PetsciiDisplay};

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut display = PetsciiDisplay {};
    display.clear(C64Color::Blue).unwrap();

    Circle::new(Point::new(8, 0), 25)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(2)
                .stroke_color(C64Color::LightRed)
                .fill_color(C64Color::Red)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(C64Color::Yellow)
        .background_color(C64Color::DarkGray)
        .build();
    Text::new("RUST-MOS", Point::new(0, 14), style)
        .draw(&mut display)
        .unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

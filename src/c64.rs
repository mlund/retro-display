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

//! Commodore 64 display drivers and color palette

use embedded_graphics_core::pixelcolor::*;
use embedded_graphics_core::prelude::*;

/// C64 color palette
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum C64Color {
    Black = 0,
    White = 1,
    Red = 2,
    Cyan = 3,
    Purple = 4,
    Green = 5,
    Blue = 6,
    Yellow = 7,
    Orange = 8,
    Brown = 9,
    LightRed = 10,
    /// Also known as gray 1
    DarkGray = 11,
    /// Also known as gray 2
    Gray = 12,
    LightGreen = 13,
    LightBlue = 14,
    /// Also known as Gray 3
    LightGray = 15,
}

impl const From<Gray8> for C64Color {
    fn from(value: Gray8) -> Self {
        match value.luma() {
            0..=31 => Self::Black,
            32..=63 => Self::DarkGray,
            64..=95 => Self::Gray,
            96..=127 => Self::LightGray,
            128..=255 => Self::White,
        }
    }
}

/// Todo: currently this simply transforms to grey scale
impl const From<Rgb555> for C64Color {
    fn from(value: Rgb555) -> Self {
        Gray8::from(value).into()
    }
}

/// Todo: currently this simply transforms to grey scale
impl const From<Rgb888> for C64Color {
    fn from(value: Rgb888) -> Self {
        Gray8::from(value).into()
    }
}

impl const From<C64Color> for Rgb888 {
    fn from(color: C64Color) -> Self {
        match color {
            C64Color::Black => Self::new(0, 0, 0),
            C64Color::White => Self::new(255, 255, 255),
            C64Color::Red => Self::new(136, 0, 0),
            C64Color::Cyan => Self::new(170, 255, 238),
            C64Color::Purple => Self::new(204, 68, 204),
            C64Color::Green => Self::new(0, 204, 85),
            C64Color::Blue => Self::new(0, 0, 170),
            C64Color::Yellow => Self::new(238, 238, 119),
            C64Color::Orange => Self::new(221, 136, 85),
            C64Color::Brown => Self::new(102, 68, 0),
            C64Color::LightRed => Self::new(255, 119, 119),
            C64Color::DarkGray => Self::new(51, 51, 51),
            C64Color::Gray => Self::new(119, 119, 119),
            C64Color::LightGreen => Self::new(170, 255, 102),
            C64Color::LightBlue => Self::new(0, 136, 255),
            C64Color::LightGray => Self::new(187, 187, 187),
        }
    }
}

impl const PixelColor for C64Color {
    type Raw = ();
}

/// Color PETSCII Display Driver
///
/// Simple display driver for the C64's PETSCII text mode.
/// A "pixel" is a colored, filled square character on the 40 x 25 text display.
///
/// # Examples
/// ~~~
/// use retro_display::c64::{C64Color, PetsciiDisplay};
/// let mut display = PetsciiDisplay {};
/// display.clear(C64Color::Blue)?;
/// ~~~
pub struct PetsciiDisplay {}

impl PetsciiDisplay {
    /// VIC-II video memory pointer
    const VIDEO_RAM: *mut u8 = (0x0400) as *mut u8;
    /// VIC-II color memory pointer
    const COLOR_RAM: *mut u8 = (0xd800) as *mut u8;
    /// PETSCII symbol to mimic a pixel (filled square)
    const PIXEL_SYMBOL: u8 = 0xa0;
}

impl const OriginDimensions for PetsciiDisplay {
    fn size(&self) -> Size {
        Size::new(40, 25)
    }
}

impl const DrawTarget for PetsciiDisplay {
    type Color = C64Color;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if let (x @ 0..=39, y @ 0..=24) = coord.into() {
                let index = (x + y * 40) as isize;
                unsafe {
                    Self::COLOR_RAM.offset(index).write(color as u8);
                    Self::VIDEO_RAM.offset(index).write(Self::PIXEL_SYMBOL);
                }
            }
        }
        Ok(())
    }
}

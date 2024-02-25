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

use embedded_graphics_core::pixelcolor::raw::RawU4;
use embedded_graphics_core::pixelcolor::*;
use embedded_graphics_core::prelude::*;

/// Color palette of the VIC-II chip found in e.g. the Commodore 64 and 128
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VicIIPalette {
    /// Black color (<a style="color:#000000;">&#9679;</a>)
    Black = 0,
    /// White color (<a style="color:#FFFFFF;">&#9679;</a>)
    White = 1,
    /// Red color (<a style="color:#9F4E44;">&#9679;</a>)
    Red = 2,
    /// Cyan color (<a style="color:#6ABFC6;">&#9679;</a>)
    Cyan = 3,
    /// Purple color (<a style="color:#A057A3;">&#9679;</a>)
    Purple = 4,
    /// Green color (<a style="color:#5CAB5E;">&#9679;</a>)
    Green = 5,
    /// Blue color (<a style="color:#50459B;">&#9679;</a>)
    Blue = 6,
    /// Yellow color (<a style="color:#C9D487;">&#9679;</a>)
    Yellow = 7,
    /// Orange color (<a style="color:#A1683C;">&#9679;</a>)
    Orange = 8,
    /// Brown color (<a style="color:#6D5412;">&#9679;</a>)
    Brown = 9,
    /// Light Red color (<a style="color:#CB7E75;">&#9679;</a>)
    LightRed = 10,
    /// Dark Gray color or "gray 1" (<a style="color:#626262;">&#9679;</a>)
    DarkGray = 11,
    /// Gray color or "gray 2" (<a style="color:#898989;">&#9679;</a>)
    Gray = 12,
    /// Light Green color (<a style="color:#9AE29B;">&#9679;</a>)
    LightGreen = 13,
    /// Light Blue color (<a style="color:#887ECB;">&#9679;</a>)
    LightBlue = 14,
    /// Light Gray color or "gray 3" (<a style="color:#ADADAD;">&#9679;</a>)
    LightGray = 15,
}

impl const From<Gray8> for VicIIPalette {
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
impl const From<Rgb555> for VicIIPalette {
    fn from(value: Rgb555) -> Self {
        Gray8::from(value).into()
    }
}

/// Todo: currently this simply transforms to grey scale
impl const From<Rgb888> for VicIIPalette {
    fn from(value: Rgb888) -> Self {
        Gray8::from(value).into()
    }
}

impl const From<VicIIPalette> for Rgb888 {
    fn from(color: VicIIPalette) -> Self {
        match color {
            VicIIPalette::Black => Self::new(0, 0, 0),
            VicIIPalette::White => Self::new(255, 255, 255),
            VicIIPalette::Red => Self::new(136, 0, 0),
            VicIIPalette::Cyan => Self::new(170, 255, 238),
            VicIIPalette::Purple => Self::new(204, 68, 204),
            VicIIPalette::Green => Self::new(0, 204, 85),
            VicIIPalette::Blue => Self::new(0, 0, 170),
            VicIIPalette::Yellow => Self::new(238, 238, 119),
            VicIIPalette::Orange => Self::new(221, 136, 85),
            VicIIPalette::Brown => Self::new(102, 68, 0),
            VicIIPalette::LightRed => Self::new(255, 119, 119),
            VicIIPalette::DarkGray => Self::new(51, 51, 51),
            VicIIPalette::Gray => Self::new(119, 119, 119),
            VicIIPalette::LightGreen => Self::new(170, 255, 102),
            VicIIPalette::LightBlue => Self::new(0, 136, 255),
            VicIIPalette::LightGray => Self::new(187, 187, 187),
        }
    }
}

impl const PixelColor for VicIIPalette {
    type Raw = RawU4;
}

impl const From<RawU4> for VicIIPalette {
    fn from(value: RawU4) -> Self {
        let color_index = value.into_inner();

        // SAFETY:
        // RawU4 guarantees that the value inside will always be in the range
        // 0..=15. This and the fact that VicIIPalette is repr(u8) makes it save
        // to transmute the u8 to a VicIIPalette.
        unsafe { core::mem::transmute::<u8, VicIIPalette>(color_index) }
    }
}

impl const From<VicIIPalette> for RawU4 {
    fn from(value: VicIIPalette) -> Self {
        RawU4::new(value as u8)
    }
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
    /// Number of columns in the C64 PETSCII display
    const COLS: isize = 40;
    /// Number of rows in the C64 PETSCII display
    const ROWS: isize = 25;
    /// VIC-II video memory pointer
    const VIDEO_RAM: *mut u8 = (0x0400) as *mut u8;
    /// VIC-II color memory pointer
    const COLOR_RAM: *mut u8 = (0xd800) as *mut u8;
    /// PETSCII symbol to mimic a pixel (filled square)
    const PIXEL_SYMBOL: u8 = 0xa0;

    /// Set pixel without checking if the position is within bounds
    ///
    /// # Safety
    ///
    /// Unsafe as the index is unchecked and may write to memory outside the display.
    ///
    unsafe fn set_pixel_unchecked(index: isize, color: VicIIPalette) {
        Self::COLOR_RAM.offset(index).write(color as u8);
        Self::VIDEO_RAM.offset(index).write(Self::PIXEL_SYMBOL);
    }

    /// Set pixel with bounds checking
    fn set_pixel(coord: &Point, color: VicIIPalette) {
        // inelegant but small(est?) binary size
        let x = coord.x as isize;
        if (0..Self::COLS).contains(&x) {
            let y = coord.y as isize;
            if (0..Self::ROWS).contains(&y) {
                let index = x + y * Self::COLS;
                unsafe {
                    Self::set_pixel_unchecked(index, color);
                }
            }
        }
    }
}

impl const OriginDimensions for PetsciiDisplay {
    fn size(&self) -> Size {
        Size::new(Self::COLS as u32, Self::ROWS as u32)
    }
}

impl DrawTarget for PetsciiDisplay {
    type Color = VicIIPalette;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        pixels
            .into_iter()
            .for_each(|Pixel(coord, color)| Self::set_pixel(&coord, color));
        Ok(())
    }
}

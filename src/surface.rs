// SPDX-License-Identifier: GPL-3.0-only
//! Host-independent mutable pixels for offscreen rendering.

use std::ops::Range;

const MAX_PIXELS: u64 = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub origin: Point,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceError {
    InvalidDimensions,
    InvalidPixelCount,
    AllocationFailed,
}

impl std::fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidDimensions => "surface dimensions exceed the pixel budget or are zero",
            Self::InvalidPixelCount => "pixel count does not match the surface dimensions",
            Self::AllocationFailed => "could not allocate a pixel buffer",
        })
    }
}

impl std::error::Error for SurfaceError {}

/// A row-major raster of owned 0xAARRGGBB words.
///
/// Drawing replaces destination pixels exactly, including the alpha byte.
/// A surface contains at most 1,048,576 pixels and has no host handles.
#[derive(Debug, PartialEq, Eq)]
pub struct Surface {
    width: u32,
    height: u32,
    pixels: Box<[u32]>,
}

impl Surface {
    pub fn new(width: u32, height: u32, pixel: u32) -> Result<Self, SurfaceError> {
        let count = pixel_count(width, height)?;
        let mut pixels = Vec::new();
        pixels
            .try_reserve_exact(count)
            .map_err(|_| SurfaceError::AllocationFailed)?;
        pixels.resize(count, pixel);
        Ok(Self {
            width,
            height,
            pixels: pixels.into_boxed_slice(),
        })
    }

    /// Reconstructs a surface from an exact row-major pixel payload.
    pub fn from_pixels(width: u32, height: u32, pixels: Box<[u32]>) -> Result<Self, SurfaceError> {
        if pixel_count(width, height)? != pixels.len() {
            return Err(SurfaceError::InvalidPixelCount);
        }
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    /// Fills the intersection of the rectangle and this surface.
    pub fn fill(&mut self, rect: Rect, pixel: u32) {
        let columns = clipped_span(rect.origin.x, rect.width, self.width);
        if columns.is_empty() {
            return;
        }
        let rows = clipped_span(rect.origin.y, rect.height, self.height);
        let stride = self.width as usize;
        for row in rows {
            let offset = row * stride;
            self.pixels[offset + columns.start..offset + columns.end].fill(pixel);
        }
    }

    /// Copies a rectangle from another surface. Both surfaces clip the copy,
    /// preserving the correspondence between source and destination pixels.
    pub fn blit(&mut self, source: &Self, rect: Rect, destination: Point) {
        let Some(copy) = Transfer::clip(rect, destination, source.dimensions(), self.dimensions())
        else {
            return;
        };
        for row in 0..copy.height {
            let from = copy.source_row(row, source.width as usize);
            let to = copy.destination_row(row, self.width as usize);
            self.pixels[to..to + copy.width]
                .copy_from_slice(&source.pixels[from..from + copy.width]);
        }
    }

    /// Copies within the same surface, reading the original pixels even
    /// when the clipped source and destination overlap.
    pub fn copy_within(&mut self, rect: Rect, destination: Point) {
        let size = self.dimensions();
        let Some(copy) = Transfer::clip(rect, destination, size, size) else {
            return;
        };
        let stride = self.width as usize;
        for row in 0..copy.height {
            // Copy bottom-up when moving downward so no unread row is overwritten.
            let row = if copy.destination_y > copy.source_y {
                copy.height - 1 - row
            } else {
                row
            };
            let from = copy.source_row(row, stride);
            let to = copy.destination_row(row, stride);
            self.pixels.copy_within(from..from + copy.width, to);
        }
    }
}

fn pixel_count(width: u32, height: u32) -> Result<usize, SurfaceError> {
    let count = u64::from(width) * u64::from(height);
    if count == 0 || count > MAX_PIXELS {
        return Err(SurfaceError::InvalidDimensions);
    }
    usize::try_from(count).map_err(|_| SurfaceError::InvalidDimensions)
}

fn clipped_span(start: i32, length: u32, extent: u32) -> Range<usize> {
    let start = i64::from(start);
    let end = start + i64::from(length);
    let extent = i64::from(extent);
    start.clamp(0, extent) as usize..end.clamp(0, extent) as usize
}

struct Transfer {
    source_x: usize,
    source_y: usize,
    destination_x: usize,
    destination_y: usize,
    width: usize,
    height: usize,
}

impl Transfer {
    fn clip(
        rect: Rect,
        destination: Point,
        source_size: (u32, u32),
        destination_size: (u32, u32),
    ) -> Option<Self> {
        let (source_x, destination_x, width) = paired_span(
            rect.origin.x,
            destination.x,
            rect.width,
            source_size.0,
            destination_size.0,
        )?;
        let (source_y, destination_y, height) = paired_span(
            rect.origin.y,
            destination.y,
            rect.height,
            source_size.1,
            destination_size.1,
        )?;
        Some(Self {
            source_x,
            source_y,
            destination_x,
            destination_y,
            width,
            height,
        })
    }

    fn source_row(&self, row: usize, stride: usize) -> usize {
        (self.source_y + row) * stride + self.source_x
    }

    fn destination_row(&self, row: usize, stride: usize) -> usize {
        (self.destination_y + row) * stride + self.destination_x
    }
}

fn paired_span(
    source: i32,
    destination: i32,
    length: u32,
    source_extent: u32,
    destination_extent: u32,
) -> Option<(usize, usize, usize)> {
    let source = i64::from(source);
    let destination = i64::from(destination);
    // Intersect in rectangle-relative coordinates to keep the pixel mapping.
    let first = 0.max(-source).max(-destination);
    let end = i64::from(length)
        .min(i64::from(source_extent) - source)
        .min(i64::from(destination_extent) - destination);
    if first >= end {
        return None;
    }
    Some((
        (source + first) as usize,
        (destination + first) as usize,
        (end - first) as usize,
    ))
}

// SPDX-License-Identifier: GPL-3.0-only
use j2me_vm::surface::{Point, Rect, Surface, SurfaceError};

fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}

fn rect(x: i32, y: i32, width: u32, height: u32) -> Rect {
    Rect {
        origin: point(x, y),
        width,
        height,
    }
}

#[test]
fn dimensions_and_payload_must_fit_owned_surface() {
    assert_eq!(Surface::new(0, 3, 0), Err(SurfaceError::InvalidDimensions));
    assert_eq!(
        Surface::new(u32::MAX, u32::MAX, 0),
        Err(SurfaceError::InvalidDimensions)
    );
    assert_eq!(
        Surface::from_pixels(2, 2, Box::new([1, 2, 3])),
        Err(SurfaceError::InvalidPixelCount)
    );
}

#[test]
fn fill_clips_without_overflow_and_preserves_exact_pixel_words() {
    let mut surface = Surface::new(3, 2, 0xff00_0000).unwrap();
    surface.fill(rect(-1, 1, 3, u32::MAX), 0x107f_fefd);
    surface.fill(rect(i32::MAX, i32::MIN, u32::MAX, u32::MAX), 0);
    surface.fill(rect(0, 0, 3, 0), 0);
    assert_eq!(
        surface.pixels(),
        &[
            0xff00_0000,
            0xff00_0000,
            0xff00_0000,
            0x107f_fefd,
            0x107f_fefd,
            0xff00_0000
        ],
    );
    let (width, height) = surface.dimensions();
    let reconstructed = Surface::from_pixels(width, height, surface.pixels().into()).unwrap();
    assert_eq!(surface, reconstructed);
}

#[test]
fn blit_clips_both_surfaces_without_shifting_pixel_pairs() {
    let source =
        Surface::from_pixels(4, 3, Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])).unwrap();
    let mut destination = Surface::new(3, 2, 99).unwrap();
    destination.blit(&source, rect(-1, 0, 6, 4), point(0, -1));
    assert_eq!(destination.pixels(), &[99, 5, 6, 99, 9, 10]);
    destination.blit(
        &source,
        rect(0, 0, u32::MAX, u32::MAX),
        point(i32::MIN, i32::MAX),
    );
    assert_eq!(destination.pixels(), &[99, 5, 6, 99, 9, 10]);
}

#[test]
fn self_copy_matches_a_snapshot_of_the_source_in_each_overlap_direction() {
    let original: Vec<u32> = (0..20).collect();
    for region in [rect(0, 0, 4, 3), rect(1, 1, 4, 3), rect(-1, -1, 5, 4)] {
        for destination in [
            point(1, 1),
            point(-1, -1),
            point(1, 0),
            point(0, 1),
            point(0, 0),
        ] {
            let mut expected = original.clone();
            for dy in 0..region.height as i32 {
                for dx in 0..region.width as i32 {
                    let source_x = region.origin.x + dx;
                    let source_y = region.origin.y + dy;
                    let dest_x = destination.x + dx;
                    let dest_y = destination.y + dy;
                    if [source_x, dest_x].into_iter().all(|x| (0..5).contains(&x))
                        && [source_y, dest_y].into_iter().all(|y| (0..4).contains(&y))
                    {
                        expected[(dest_y * 5 + dest_x) as usize] =
                            original[(source_y * 5 + source_x) as usize];
                    }
                }
            }
            let mut surface =
                Surface::from_pixels(5, 4, original.clone().into_boxed_slice()).unwrap();
            surface.copy_within(region, destination);
            assert_eq!(surface.pixels(), expected, "{region:?} -> {destination:?}");
        }
    }
}

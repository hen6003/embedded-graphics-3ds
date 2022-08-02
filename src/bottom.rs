use ctru::gfx::{BottomScreen, Screen};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};
use std::cell::RefMut;

pub struct BottomDisplay3DS<'a> {
    size: (u32, u32),
    screen: RefMut<'a, BottomScreen>,
}

impl<'a> BottomDisplay3DS<'a> {
    pub fn new(mut screen: RefMut<'a, BottomScreen>) -> Self {
        let frame_buffer = screen.get_raw_framebuffer();

        Self {
            size: (frame_buffer.width.into(), frame_buffer.height.into()),
            screen,
        }
    }

    pub fn double_buffering(&mut self, state: bool) {
        self.screen.set_double_buffering(state)
    }
}

impl embedded_graphics_core::geometry::Dimensions for BottomDisplay3DS<'_> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Size::new(self.size.1, self.size.0))
    }
}

impl embedded_graphics_core::draw_target::DrawTarget for BottomDisplay3DS<'_> {
    type Color = Rgb888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let frame_buffer = self.screen.get_raw_framebuffer();

        for Pixel(coord, color) in pixels {
            let y = frame_buffer.width as i32 - coord.y - 1;

            if !(coord.x < 0
                || y < 0
                || coord.x > frame_buffer.height.into()
                || y > frame_buffer.width as i32 - 1)
            {
                unsafe {
                    frame_buffer
                        .ptr
                        .offset(
                            (3 * (coord.x * frame_buffer.width as i32 + y))
                                .try_into()
                                .unwrap(),
                        )
                        .copy_from([color.b(), color.g(), color.r()].as_ptr(), 3);
                }
            }
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let frame_buffer = self.screen.get_raw_framebuffer();
        let column = [color.b(), color.g(), color.r()].repeat(frame_buffer.height as usize);

        let mut position = frame_buffer.ptr;
        unsafe {
            for _ in 0..frame_buffer.width {
                position.copy_from(column.as_ptr(), column.len());

                position = position.offset(frame_buffer.height as isize * 3);
            }
        }

        Ok(())
    }
}

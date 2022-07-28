use embedded_graphics_3ds::TopDisplay3DS;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
    text::{Alignment, Text},
};

use ctru::console::Console;
use ctru::services::hid::KeyPad;
use ctru::services::{Apt, Hid};
use ctru::Gfx;

fn main() {
    ctru::init();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.bottom_screen.borrow_mut());

    let mut display = TopDisplay3DS::new(gfx.top_screen.borrow_mut());

    let fill = PrimitiveStyle::with_fill(Rgb888::RED);
    let thin_stroke = PrimitiveStyle::with_stroke(Rgb888::CYAN, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(Rgb888::GREEN, 3);
    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);

    // Main loop
    let mut rect_pos = Point::zero();

    while apt.main_loop() {
        // Input
        hid.scan_input();

        if hid.keys_held().intersects(KeyPad::KEY_DOWN) {
            rect_pos.y += 10;
        }
        if hid.keys_held().intersects(KeyPad::KEY_UP) {
            rect_pos.y -= 10;
        }
        if hid.keys_held().intersects(KeyPad::KEY_LEFT) {
            rect_pos.x -= 10;
        }
        if hid.keys_held().intersects(KeyPad::KEY_RIGHT) {
            rect_pos.x += 10;
        }

        // Drawing
        display.clear(Rgb888::BLACK).unwrap();

        Rectangle::new(rect_pos, Size::new(16, 16))
            .into_styled(fill)
            .draw(&mut display)
            .unwrap();

        // Draw a circle with a 3px wide stroke.
        Circle::new(Point::new(88, 10), 17)
            .into_styled(thick_stroke)
            .draw(&mut display)
            .unwrap();

        // Draw a triangle
        Triangle::new(
            Point::new(16, 16),
            Point::new(16 + 16, 16),
            Point::new(16 + 8, 0),
        )
        .into_styled(thin_stroke)
        .draw(&mut display)
        .unwrap();

        // Draw centered text.
        let text = "embedded-graphics";
        Text::with_alignment(
            text,
            display.bounding_box().center() + Point::new(0, 15),
            character_style,
            Alignment::Center,
        )
        .draw(&mut display)
        .unwrap();

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}

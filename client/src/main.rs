extern crate raylib;
extern crate text_box;

use text_box::TextWindow;

#[cfg(target_family = "wasm")]
extern "C" {
    fn emscripten_set_main_loop(
        loop_fn: extern "C" fn(),
        fps: i32,
        sim_infinite_loop: i32
    );
}

#[cfg(target_family = "wasm")]
#[no_mangle]
pub extern "C" fn on_resize(
    width: i32,
    height: i32
) {
    raylib::set_window_size(width, height);
}

#[cfg(target_family = "wasm")]
extern "C" fn em_main_loop() {
    main_loop();
}

#[repr(C)]
struct DisplayData {
    text: TextWindow,
    width: i32,
    height: i32,
    draw_change: bool,
}

fn main_loop() {
    let mut data: &mut DisplayData;
    unsafe {
        data = DISPLAY_DATA.as_mut().unwrap();
    }

    if raylib::is_window_resized() {
        data.width = raylib::get_screen_width();
        data.height = raylib::get_screen_height();
        data.draw_change = true;
        data.text.set_size(
            data.width as f32,
            data.height as f32
        );
    }

    let key = raylib::keyboard::get_key_pressed();
    if key == raylib::keyboard::UP {
        data.text.scroll(-1);
        data.draw_change = true;
    }
    if key == raylib::keyboard::DOWN {
        data.text.scroll(1);
        data.draw_change = true;
    }

    raylib::begin_drawing(); 

    if data.draw_change {
        data.draw_change = false;

        raylib::clear_background(raylib::color::DARKGRAY);
        data.text.draw();
    }

    raylib::end_drawing();    
}

static mut DISPLAY_DATA: Option<DisplayData> = None;

fn main() {
    raylib::set_config_flags(raylib::flags::WINDOW_RESIZABLE);
    raylib::init_window(640, 480, "Client");
    raylib::set_window_min_size(360, 360);
    raylib::set_target_fps(60);

    let text_box = TextWindow::new(
        "hello this is a nice test of my very nice text box drawing software. oh boy oh BOY do I hope this works and looks nice! here is a whole lot more text so I cant test out whether scrolling works. oh boy do I hope scrolling workks, if that workds then I don't know what I will do! I might just go downstairs and take a short break, or maybe watch an episode of something like the devil is a part timer or Spy x Family! I love those shows, comedy anime are actually pretty great!",
        |s| {
            let parts = s.split(|c| {
                c == '.' || c == ',' || c == '!'
                    || c == '?'
            });

            for part in parts {
                if part == "software" {
                    return true;
                }
            }
            false
        },
        raylib::Rectangle {
            x: 0.0, y: 0.0,
            width: raylib::get_screen_width() as f32,
            height: raylib::get_screen_width() as f32,
        },
        6.0,
        4.0,
        raylib::color::MEDGRAY,
        raylib::color::RAYWHITE,
        raylib::get_font_default(),
        20.0,
        1.0,
        raylib::color::RAYWHITE,
        raylib::color::YELLOW,
        1.5
    );

    let data = DisplayData {
        text: text_box,
        width: raylib::get_screen_width(),
        height: raylib::get_screen_height(),
        draw_change: true,
    };

    unsafe {
        DISPLAY_DATA = Some(data);
    }

    #[cfg(target_family = "wasm")]
    unsafe {
        emscripten_set_main_loop(
            em_main_loop,
            0,
            1
        );
    }

    #[cfg(target_family = "unix")]
    while !raylib::window_should_close() {
        main_loop();
    }
}

use wasm_timer::Instant;
use std::time::Duration;
use std::collections::VecDeque;

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

extern crate raylib;

fn measure_text_lines(
    text: &[u8], width: i32, font_size: i32
) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut words = text.split(|&c| {
        c == b' '
    });
    let x_offset = font_size / 2;
    let y_offset = font_size + (font_size / 4);

    while let Some(word) = words.next() {
        let word_width = raylib::measure_text(&word, font_size);

        if x + word_width > width {
            x = 0;
            y += y_offset;
        }
 
        x += word_width + x_offset;
    }
    y + y_offset
}

fn draw_text_lines(
    text: &[u8], start_x: i32, start_y: i32, width: i32,
    font_size: i32, show_cursor: bool, cursor_pos: usize
) {
    let mut x = 0;
    let mut y = 0;
    let mut ccount = 0;
    let mut words = text.split(|&c| {
        c == b' '
    });
    let x_offset = font_size / 2;
    let y_offset = font_size + (font_size / 4);

    while let Some(word) = words.next() {
        let word_width = raylib::measure_text(&word, font_size);

        if x + word_width > width {
            x = 0;
            y += y_offset;
        }

        if show_cursor && cursor_pos >= ccount 
            && ccount + word.len() >= cursor_pos
        {
            let char_offset = cursor_pos - ccount;
            let cx1 = raylib::measure_text(
                &word[0..char_offset], font_size
            );
            raylib::draw_rectangle(
                start_x + x + cx1, start_y + y, 2, font_size,
                raylib::color::RAYWHITE
            );
        }

        raylib::draw_text(
            &word,
            start_x + x,
            start_y + y,
            font_size,
            raylib::color::RAYWHITE
        );
        
        x += word_width + x_offset;
        ccount += word.len() + 1;
    }
}

const MCAPACITY: usize = 100;
const MTEXT_LEN: usize = 10000;
const FONT_SIZE: i32 = 24;
const LINE_HEIGHT: i32 = FONT_SIZE + FONT_SIZE / 4;

#[repr(C)]
struct DisplayData {
    messages: VecDeque::<Vec::<u8>>,
    text: Vec::<u8>,
    width: i32,
    height: i32,
    caps: bool,
    show_cursor: bool,
    cursor_pos: usize,
    mshift: i32,
    can_shift_up: bool,
    cursor_inst: Instant,
    cursor_tstep: Duration,
    held_inst: Instant,
    held_start: Instant,
    held_key: i32,
    held_start_tstep: Duration,
    held_tstep: Duration,
}

fn main_loop() {
    let mut data: &mut DisplayData;
    unsafe {
        data = DISPLAY_DATA.as_mut().unwrap();
    }

    let mut key = raylib::keyboard::get_key_pressed();
    let shift = !raylib::keyboard::is_key_up(
        raylib::keyboard::LEFT_SHIFT
    ) || !raylib::keyboard::is_key_up(
        raylib::keyboard::RIGHT_SHIFT
    );
 
    if key != 0 {
        println!("key: {}, time: {}", key, data.held_inst.elapsed().as_millis());
        data.held_key = key;
        data.held_start = Instant::now();
    } else if data.held_key != 0 {
        if !raylib::keyboard::is_key_up(data.held_key) {
            if data.held_start.elapsed()
                > data.held_start_tstep
            {
                key = data.held_key;
            }
        } else {
            data.held_key = 0;
        }
    }

    if data.held_inst.elapsed() > data.held_tstep {
        if key != 0 {
            data.held_inst = Instant::now();
        }

        match key {
            raylib::keyboard::CAPS_LOCK => data.caps = !data.caps,
            raylib::keyboard::BACKSPACE => {
                if data.cursor_pos > 0 {
                    data.text.remove(data.cursor_pos - 1);
                    data.cursor_pos -= 1;
                    data.show_cursor = true;
                }
                data.cursor_inst = Instant::now();
            },
            raylib::keyboard::LEFT => {
                if data.cursor_pos > 0 {
                    data.cursor_pos -= 1;
                }
                data.show_cursor = true;
                data.cursor_inst = Instant::now();
            },
            raylib::keyboard::RIGHT => {
                if data.cursor_pos < data.text.len() {
                    data.cursor_pos += 1;
                }
                data.show_cursor = true;
                data.cursor_inst = Instant::now();
            },
            raylib::keyboard::UP => {
                if data.can_shift_up {
                    data.mshift += 1;
                }
            },
            raylib::keyboard::DOWN => {
                if data.mshift > 0 {
                    data.mshift -= 1;
                }
            },
            raylib::keyboard::ENTER => {
                if data.text.len() > 0 {
                    if data.messages.len() >= MCAPACITY {
                        data.messages.pop_front();
                    }
                    data.messages.push_back(data.text.clone());
                    data.mshift = 0;
                }
                data.text.clear();
                data.cursor_pos = 0;
            },
            key => {
                if let Some(c) = raylib::keyboard::get_char(
                    key, shift, data.caps
                ) {
                    if data.text.len() < MTEXT_LEN {
                        data.text.insert(data.cursor_pos, c);
                        data.cursor_pos += 1;
                        data.show_cursor = true;
                        data.cursor_inst = Instant::now();
                    }
                }
            },
        }
    }

    if raylib::is_window_resized() {
        data.width = raylib::get_screen_width();
        data.height = raylib::get_screen_height();
        data.mshift = 0;
    }

    let typing_height = measure_text_lines(
        &data.text, data.width - 2 * LINE_HEIGHT, FONT_SIZE
    );
    let typing_y = data.height - LINE_HEIGHT - typing_height;

    if data.cursor_inst.elapsed() > data.cursor_tstep {
        data.show_cursor = !data.show_cursor;
        data.cursor_inst = Instant::now();
    }

    let mut message_y: i32 = typing_y + data.mshift * LINE_HEIGHT;
    data.can_shift_up = false;

    raylib::begin_drawing(); 
    raylib::clear_background(raylib::color::DARKGRAY);

    // draw messages
    for message in data.messages.iter().rev() {
        let message_height = measure_text_lines(
            &message, data.width - 2 * LINE_HEIGHT, FONT_SIZE
        );

        message_y -= message_height + LINE_HEIGHT;

        if message_y < LINE_HEIGHT {
            data.can_shift_up = true;
        }

        draw_text_lines(
            &message,
            LINE_HEIGHT,
            message_y,
            data.width - 2 * LINE_HEIGHT,
            FONT_SIZE,
            false,
            0
        ); 
    }

    // top border box
    raylib::draw_rectangle(
        0,
        0,
        data.width,
        LINE_HEIGHT,
        raylib::color::DARKGRAY
    );

    // bottom border box
    raylib::draw_rectangle(
        0,
        typing_y - LINE_HEIGHT,
        data.width,
        data.height - typing_y + LINE_HEIGHT,
        raylib::color::DARKGRAY
    );

    // typing box
    draw_text_lines(
        &data.text,
        LINE_HEIGHT,
        typing_y,
        data.width - 2 * LINE_HEIGHT,
        FONT_SIZE,
        data.show_cursor,
        data.cursor_pos
    ); 

    raylib::end_drawing();
}

static mut DISPLAY_DATA: Option<DisplayData> = None;

fn main() {
    raylib::set_config_flags(raylib::flags::WINDOW_RESIZABLE);
    raylib::init_window(640, 480, "raylib rust test");
    raylib::set_target_fps(60);

    let data = DisplayData {
        messages: VecDeque::<Vec::<u8>>::with_capacity(
            MCAPACITY
        ),
        text: Vec::<u8>::new(),
        width: raylib::get_screen_width(),
        height: raylib::get_screen_height(),
        caps: false,
        show_cursor: true,
        cursor_pos: 0,
        mshift: 0,
        can_shift_up: false,
        cursor_inst: Instant::now(),
        cursor_tstep: Duration::from_millis(500),
        held_inst: Instant::now(),
        held_start: Instant::now(),
        held_key: 0,
        held_start_tstep: Duration::from_millis(490),
        held_tstep: Duration::from_millis(16),
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

    #[cfg(target_family = "windows")]
    while !raylib::window_should_close() {
        main_loop();
    }
}

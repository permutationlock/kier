//use std::net::TcpStream;
use std::time::{Instant, Duration};
use std::collections::VecDeque;

use kier::raylib;

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

fn main() {
    //let mut sstream = TcpStream::connect("127.0.0.1:9090").unwrap();
    //let mut stdin = io::stdin();
    //let mut buffer : [u8; 50] = [0; 50];

    raylib::set_config_flags(raylib::flags::WINDOW_RESIZABLE);
    raylib::init_window(640, 480, "raylib rust test");
    raylib::set_target_fps(30);

    const MCAPACITY: usize = 100;
    const MTEXT_LEN: usize = 10000;
    const FONT_SIZE: i32 = 24;
    const LINE_HEIGHT: i32 = FONT_SIZE + FONT_SIZE / 4;

    let mut messages = VecDeque::<Vec::<u8>>::with_capacity(
        MCAPACITY
    );
    let mut text = Vec::<u8>::new();

    let mut width: i32 = raylib::get_screen_width();
    let mut height: i32 = raylib::get_screen_height();

    let mut caps: bool = false;
    let mut show_cursor: bool = true;
    let mut cursor_pos: usize = 0;
    let mut mshift: i32 = 0;
    let mut can_shift_up: bool = false;

    let mut cursor_inst: Instant = Instant::now();
    let cursor_tstep: Duration = Duration::from_millis(500);
    
    let mut held_inst: Instant = Instant::now();
    let mut held_start: Instant = Instant::now();
    let mut held_key: i32 = 0;
    let held_start_tstep: Duration = Duration::from_millis(490);
    let held_tstep: Duration = Duration::from_millis(30);

    //stream.set_nonblocking(true).expect("Cannot set non-blocking");
    while !raylib::window_should_close() {
        let mut key = raylib::keyboard::get_key_pressed();
        let shift = !raylib::keyboard::is_key_up(
            raylib::keyboard::LEFT_SHIFT
        ) || !raylib::keyboard::is_key_up(
            raylib::keyboard::RIGHT_SHIFT
        );
       
        if key != 0 {
            held_key = key;
            held_inst = Instant::now();
            held_start = Instant::now();
        } else if held_key != 0 {
            if !raylib::keyboard::is_key_up(held_key) {
                if held_start.elapsed() > held_start_tstep {
                    if held_inst.elapsed() > held_tstep {
                        held_inst = Instant::now();
                        key = held_key;
                    }
                }
            } else {
                held_key = 0;
            }
        }

        match key {
            raylib::keyboard::CAPS_LOCK => caps = !caps,
            raylib::keyboard::BACKSPACE => {
                if cursor_pos > 0 {
                    text.remove(cursor_pos - 1);
                    cursor_pos -= 1;
                    show_cursor = true;
                }
                cursor_inst = Instant::now();
            },
            raylib::keyboard::LEFT => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                }
                show_cursor = true;
                cursor_inst = Instant::now();
            },
            raylib::keyboard::RIGHT => {
                if cursor_pos < text.len() {
                    cursor_pos += 1;
                }
                show_cursor = true;
                cursor_inst = Instant::now();
            },
            raylib::keyboard::UP => {
                if can_shift_up {
                    mshift += 1;
                }
            },
            raylib::keyboard::DOWN => {
                if mshift > 0 {
                    mshift -= 1;
                }
            },
            raylib::keyboard::ENTER => {
                if text.len() > 0 {
                    if messages.len() >= MCAPACITY {
                        messages.pop_front();
                    }
                    messages.push_back(text.clone());
                    mshift = 0;
                }
                text.clear();
                cursor_pos = 0;
            },
            key => {
                if let Some(c) = raylib::keyboard::get_char(
                    key, shift, caps
                ) {
                    if text.len() < MTEXT_LEN {
                        text.insert(cursor_pos, c);
                        cursor_pos += 1;
                        show_cursor = true;
                        cursor_inst = Instant::now();
                    }
                }
            },
        }

        if raylib::is_window_resized() {
            width = raylib::get_screen_width();
            height = raylib::get_screen_height();
            mshift = 0;
        }

        let typing_height = measure_text_lines(
            &text, width - 2 * LINE_HEIGHT, FONT_SIZE
        );
        let typing_y = height - LINE_HEIGHT - typing_height;

        if cursor_inst.elapsed() > cursor_tstep {
            show_cursor = !show_cursor;
            cursor_inst = Instant::now();
        }

        let mut message_y: i32 = typing_y + mshift * LINE_HEIGHT;
        can_shift_up = false;

        raylib::begin_drawing(); 
        raylib::clear_background(raylib::color::DARKGRAY);

        // draw messages
        for message in messages.iter().rev() {
            let message_height = measure_text_lines(
                &message, width - 2 * LINE_HEIGHT, FONT_SIZE
            );

            message_y -= message_height + LINE_HEIGHT;

            if message_y < LINE_HEIGHT {
                can_shift_up = true;
            }

            draw_text_lines(
                &message,
                LINE_HEIGHT,
                message_y,
                width - 2 * LINE_HEIGHT,
                FONT_SIZE,
                false,
                0
            ); 
        }

        // top border box
        raylib::draw_rectangle(
            0,
            0,
            width,
            LINE_HEIGHT,
            raylib::color::DARKGRAY
        );

        // bottom border box
        raylib::draw_rectangle(
            0,
            typing_y - LINE_HEIGHT,
            width,
            height - typing_y + LINE_HEIGHT,
            raylib::color::DARKGRAY
        );

        // typing box
        draw_text_lines(
            &text,
            LINE_HEIGHT,
            typing_y,
            width - 2 * LINE_HEIGHT,
            FONT_SIZE,
            show_cursor,
            cursor_pos
        ); 

        raylib::end_drawing();
    }
}

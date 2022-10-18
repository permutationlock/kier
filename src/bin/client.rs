//use std::net::TcpStream;
use std::time::{Instant, Duration};
use std::collections::VecDeque;

#[allow(unused)] 
mod raylib {
    #[allow(unused)] 
    pub mod flags {
        #[allow(unused)] 
        pub const WINDOW_RESIZABLE: u32 = 0x00000004;
    }

    #[allow(unused)] 
    pub mod log_level {
        pub const LOG_ALL: i32 = 0;
        pub const LOG_TRACE: i32 = 1;
        pub const LOG_DEBUG: i32 = 2;
        pub const LOG_INFO: i32 = 3;
        pub const LOG_WARNING: i32 = 4;
        pub const LOG_ERROR: i32 = 5;
        pub const LOG_FATAL: i32 = 6;
        pub const LOG_NONE: i32 = 7;
    }

    #[allow(unused)] 
    pub mod keyboard {
        pub const NULL: i32 = 0;
        pub const APOSTROPHE: i32 = 39;
        pub const COMMA: i32 = 44;
        pub const MINUS: i32 = 45;
        pub const PERIOD: i32 = 46;
        pub const SLASH: i32 = 47;
        pub const ZERO: i32 = 48;
        pub const ONE: i32 = 49;
        pub const TWO: i32 = 50;
        pub const THREE: i32 = 51;
        pub const FOUR: i32 = 52;
        pub const FIVE: i32 = 53;
        pub const SIX: i32 = 54;
        pub const SEVEN: i32 = 55;
        pub const EIGHT: i32 = 56;
        pub const NINE: i32 = 57;
        pub const SEMICOLON: i32 = 59;
        pub const EQUAL: i32 = 61;
        pub const A: i32 = 65;
        pub const B: i32 = 66;
        pub const C: i32 = 67;
        pub const D: i32 = 68;
        pub const E: i32 = 69;
        pub const F: i32 = 70;
        pub const G: i32 = 71;
        pub const H: i32 = 72;
        pub const I: i32 = 73;
        pub const J: i32 = 74;
        pub const K: i32 = 75;
        pub const L: i32 = 76;
        pub const M: i32 = 77;
        pub const N: i32 = 78;
        pub const O: i32 = 79;
        pub const P: i32 = 80;
        pub const Q: i32 = 81;
        pub const R: i32 = 82;
        pub const S: i32 = 83;
        pub const T: i32 = 84;
        pub const U: i32 = 85;
        pub const V: i32 = 86;
        pub const W: i32 = 87;
        pub const X: i32 = 88;
        pub const Y: i32 = 89;
        pub const Z: i32 = 90;
        pub const LEFT_BRACKET: i32 = 91;
        pub const BACKSLASH: i32 = 92;
        pub const RIGHT_BRACKET: i32 = 93;
        pub const GRAVE: i32 = 96;

        pub const SPACE: i32 = 32;
        pub const ESCAPE: i32 = 256;
        pub const ENTER: i32 = 257;
        pub const TAB: i32 = 258;
        pub const BACKSPACE: i32 = 259;
        pub const INSERT: i32 = 260;
        pub const DELETE: i32 = 261;
        pub const RIGHT: i32 = 262;
        pub const LEFT: i32 = 263;
        pub const DOWN: i32 = 264;
        pub const UP: i32 = 265;
        pub const PAGE_UP: i32 = 266;
        pub const PAGE_DOWN: i32 = 267;
        pub const HOME: i32 = 268;
        pub const END: i32 = 269;
        pub const CAPS_LOCK: i32 = 280;
        pub const SCROLL_LOCK: i32 = 281;
        pub const NUM_LOCK: i32 = 282;
        pub const PRINT_SCREEN: i32 = 283;
        pub const PAUSE: i32 = 284;
        pub const LEFT_SHIFT: i32 = 340;
        pub const LEFT_CONTROL: i32 = 341;
        pub const LEFT_ALT: i32 = 342;
        pub const RIGHT_SHIFT: i32 = 344;
        pub const RIGHT_CONTROL: i32 = 345;
        pub const RIGHT_ALT: i32 = 346;
        
        pub fn get_char(
            key: i32, shift: bool, caps: bool
        ) -> Option<u8> {
            if key >= A && key <= Z {
                let mut c: u8 = if shift || caps { b'A' }
                    else { b'a' };
                Some(c + ((key - A) as u8))
            } else {
                match key {
                    APOSTROPHE =>
                        Some(if shift { b'\"' } else { b'\'' }),
                    COMMA => Some(if shift { b'<' } else { b',' }),
                    MINUS => Some(if shift { b'_' } else { b'-' }),
                    PERIOD => Some(if shift { b'>' } else { b'.' }),
                    SLASH => Some(if shift { b'?' } else { b'/' }),
                    ZERO => Some(if shift { b')' } else { b'0' }),
                    ONE => Some(if shift { b'!' } else { b'1' }),
                    TWO => Some(if shift { b'@' } else { b'2' }),
                    THREE => Some(if shift { b'#' } else { b'3' }),
                    FOUR => Some(if shift { b'$' } else { b'4' }),
                    FIVE => Some(if shift { b'%' } else { b'5' }),
                    SIX => Some(if shift { b'^' } else { b'6' }),
                    SEVEN => Some(if shift { b'&' } else { b'7' }),
                    EIGHT => Some(if shift { b'*' } else { b'8' }),
                    NINE => Some(if shift { b'(' } else { b'9' }),
                    SEMICOLON =>
                        Some(if shift { b':' } else { b';' }),
                    EQUAL => Some(if shift { b'+' } else { b'=' }),
                    LEFT_BRACKET =>
                        Some(if shift { b'{' } else { b'[' }),
                    RIGHT_BRACKET =>
                        Some(if shift { b'}' } else { b'}' }),
                    BACKSLASH =>
                        Some(if shift { b'|' } else { b'\\' }),
                    GRAVE => Some(if shift { b'~' } else { b'`' }),
                    SPACE => Some(b' '),
                    ENTER => Some(b'\n'),
                    TAB => Some(b'\t'),
                    _ => None,
                }
            }
        }

        #[link(name = "raylib")]
        extern "C" {
            fn GetKeyPressed() -> i32;    
            fn IsKeyUp(key: i32) -> bool;    
        }

        pub fn get_key_pressed() -> i32 {
            unsafe { GetKeyPressed() }
        }

        pub fn is_key_up(key: i32) -> bool {
            unsafe { IsKeyUp(key) }
        }
    }

    pub mod text {
    }

    #[repr(C)]
    pub struct Vector2 {
        x: f32,
        y: f32,
    }

    #[repr(C)]
    pub struct Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    #[repr(C)]
    pub struct Texture {
        id: u32,
        width: i32,
        height: i32,
        mipmaps: i32,
        format: i32,
    }
    
    #[repr(C)]
    pub struct Image {
        data: *const libc::c_void,
        width: i32,
        height: i32,
        mipmaps: i32,
        format: i32,
    }

    #[repr(C)]
    pub struct GlyphInfo {
        value: i32,
        offset_x: i32,
        offset_y: i32,
        advance_x: i32,
        image: Image,
    }

    #[repr(C)]
    pub struct Font {
        base_size: i32,
        glyph_count: i32,
        glyph_padding: i32,
        texture: Texture,
        recs: *const Rectangle,
        glyphs: GlyphInfo,
    }

    #[repr(C)]
    pub struct Color {
        x: u8,
        y: u8,
        z: u8,
        w: u8,
    }

    pub mod color {
        use crate::raylib::Color;

        pub const RAYWHITE: Color = Color {
            x: 245, y: 245, z: 245, w: 255
        };
        pub const DARKGRAY: Color = Color {
            x: 80, y: 80, z: 80, w: 255
        };
        pub const MEDGRAY: Color = Color {
            x: 170, y: 170, z: 170, w: 255
        };
    }

    #[link(name = "raylib")]
    extern "C" {
        fn InitWindow(
            width: i32, height: i32,
            title: *const i8
        );
        fn SetTargetFPS(fps: i32);
        fn CloseWindow();
        fn BeginDrawing();
        fn EndDrawing();
        fn DrawText(
            text: *const i8,
            pos_x: i32,
            pos_y: i32,
            font_size: i32,
            color: Color
        );
        fn MeasureText(text: *const i8, font_size: i32) -> i32;
        fn MeasureTextEx(
            font: Font,
            text: *const i8,
            font_size: f32,
            spacing: f32
        ) -> Vector2;
        fn DrawRectangle(
            pos_x: i32,
            pos_y: i32,
            width: i32,
            height: i32,
            color: Color
        );
        fn ClearBackground(color: Color);
        fn WindowShouldClose() -> bool;
        fn IsWindowResized() -> bool;
        fn SetConfigFlags(flags: u32);
        fn GetScreenWidth() -> i32;
        fn GetScreenHeight() -> i32;
    }

    pub fn init_window(width: i32, height: i32, title: &str) {
        let c_title = std::ffi::CString::new(title).unwrap();
        unsafe {
            InitWindow(
                width,
                height, 
                c_title.as_ptr()
            );
        }
    }

    pub fn set_target_fps(fps: i32) {
        unsafe { SetTargetFPS(fps); }
    }

    pub fn close_window() {
        unsafe { CloseWindow(); }
    }

    pub fn begin_drawing() {
        unsafe { BeginDrawing(); }
    }

    pub fn end_drawing() {
        unsafe { EndDrawing(); }
    }

    pub fn draw_text(
        text: &[u8],
        pos_x: i32,
        pos_y: i32,
        font_size: i32,
        color: Color
    ) {
        let c_text = std::ffi::CString::new(text).unwrap();
        unsafe {
            DrawText(
                c_text.as_ptr(), pos_x, pos_y, font_size, color
            );
        }
    }

    pub fn measure_text(text: &[u8], font_size: i32) -> i32 { 
        let c_text = std::ffi::CString::new(text).unwrap();
        unsafe { MeasureText(c_text.as_ptr(), font_size) }
    }

    pub fn measure_text_ex(
        font: Font, text: &[u8], font_size: f32, spacing: f32
    ) -> Vector2 {
        let c_text = std::ffi::CString::new(text).unwrap();
        unsafe {
            MeasureTextEx(font, c_text.as_ptr(), font_size, spacing)
        }
    }

    pub fn draw_rectangle(
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        color: Color
    ) {
        unsafe { DrawRectangle(pos_x, pos_y, width, height, color); }
    }

    pub fn clear_background(color: Color) {
        unsafe { ClearBackground(color); }
    }

    pub fn window_should_close() -> bool {
        unsafe { WindowShouldClose() }
    }

    pub fn is_window_resized() -> bool {
        unsafe { IsWindowResized() }
    }

    pub fn set_config_flags(flags: u32) {
        unsafe { SetConfigFlags(flags); }
    }

    pub fn get_screen_width() -> i32 {
        unsafe { GetScreenWidth() }
    }

    pub fn get_screen_height() -> i32 {
        unsafe { GetScreenHeight() }
    }
}

fn get_font_size(width: i32, text: &[u8]) -> i32 {
    let mut fsize: i32 = 11;
    while raylib::measure_text(text, fsize) < width {
        fsize += 1;
    }
    fsize - 1
}

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

    const TEST_WTEXT: [u8; 64] = [b'w'; 64];
    const TEST_NTEXT: [u8; 48] = [b'w'; 48];
    const MCAPACITY: usize = 100;

    let mut messages = VecDeque::<Vec::<u8>>::with_capacity(
        MCAPACITY
    );
    let mut text = Vec::<u8>::new();

    let mut width: i32 = raylib::get_screen_width();
    let mut height: i32 = raylib::get_screen_height();
    let mut font_size: i32 = if width > height {
        get_font_size(width - 75, &TEST_WTEXT)
    } else {
        get_font_size(width - 75, &TEST_NTEXT)
    };

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
    let held_tstep: Duration = Duration::from_millis(50);

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
                    }
                    key = held_key;
                }
            } else {
                held_key = 0;
            }
        }

        if key == raylib::keyboard::CAPS_LOCK {
            caps = !caps;
        } else if key == raylib::keyboard::BACKSPACE {
            if cursor_pos > 0 {
                text.remove(cursor_pos - 1);
                cursor_pos -= 1;
                show_cursor = true;
            }
            cursor_inst = Instant::now();
        } else if key == raylib::keyboard::LEFT {
            if cursor_pos > 0 {
                cursor_pos -= 1;
            }
            show_cursor = true;
            cursor_inst = Instant::now();
        } else if key == raylib::keyboard::RIGHT {
            if cursor_pos < text.len() {
                cursor_pos += 1;
            }
            show_cursor = true;
            cursor_inst = Instant::now();
        } else if key == raylib::keyboard::UP {
            if can_shift_up {
                mshift += 1;
            }
        } else if key == raylib::keyboard::DOWN {
            if mshift > 0 {
                mshift -= 1;
            }
        } else if key == raylib::keyboard::ENTER {
            if text.len() > 0 {
                if messages.len() >= MCAPACITY {
                    messages.pop_front();
                }
                messages.push_back(text.clone());
                mshift = 0;
            }
            text.clear();
            cursor_pos = 0;
        } else if let Some(c) = raylib::keyboard::get_char(
            key, shift, caps
        ) {
            text.insert(cursor_pos, c);
            cursor_pos += 1;
            show_cursor = true;
            cursor_inst = Instant::now();
        }

        if raylib::is_window_resized() {
            width = raylib::get_screen_width();
            height = raylib::get_screen_height();
            font_size = if width > height {
                get_font_size(width - 20, &TEST_WTEXT)
            } else {
                get_font_size(width - 20, &TEST_NTEXT)
            };
        }

        let line_width = font_size + font_size / 4;
        let typing_height = measure_text_lines(
            &text, width - 50, font_size
        );
        let typing_y = height - line_width - typing_height;

        if cursor_inst.elapsed() > cursor_tstep {
            show_cursor = !show_cursor;
            cursor_inst = Instant::now();
        }

        let mut message_y: i32 = typing_y + mshift * line_width;
        can_shift_up = false;

        raylib::begin_drawing(); 
        raylib::clear_background(raylib::color::DARKGRAY);

        // draw messages
        for message in messages.iter().rev() {
            let message_height = measure_text_lines(
                &message, width - 50, font_size
            );
            if message_y - message_height < line_width {
                can_shift_up = true;
            }
            message_y -= message_height + line_width;

            draw_text_lines(
                &message, 30, message_y, width - 50,
                font_size, false, 0
            ); 
        }

        // top border box
        raylib::draw_rectangle(
            0, 0, width,
            10,
            raylib::color::DARKGRAY
        );

        // bottom border box
        raylib::draw_rectangle(
            0, typing_y - line_width, width,
            height - typing_y + line_width,
            raylib::color::DARKGRAY
        );

        // typing box
        raylib::draw_text(
            &[b'>'], 15, typing_y, font_size,
            raylib::color::RAYWHITE
        );
        draw_text_lines(
            &text, 30, typing_y, width - 50,
            font_size, show_cursor, cursor_pos
        ); 

        raylib::end_drawing();
    }
}

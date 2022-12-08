extern crate raylib;

use raylib::Rectangle;
use raylib::Vector2;
use raylib::Font;
use raylib::Color;
use raylib::{
    draw_text_ex,
    measure_text_ex,
    draw_rectangle_rec,
    check_collision_point_rec
};

struct Word {
    text: String,
    bbox: Rectangle,
    keyword: bool,
    visible: bool,
}

impl Word {
    fn new(text: &str, keyword: bool) -> Self {
        Self {
            text: String::from(text),
            bbox: Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            keyword: keyword,
            visible: false,
        }
    }
}

pub struct TextBox {
    words: Vec::<Word>,
    keywords: Vec::<usize>,
    bbox: Rectangle,
    color: Color,
    keyword_color: Color,
    font: Font,
    fsize: f32,
    spacing: f32,
    fixed_height: bool,
    line_spacing: f32,
    scroll: i32,
    lines: i32,
    drawn_lines: i32,
}

impl TextBox {
    pub fn new<T>(
        text: &str, is_keyword: T,
        bbox: Rectangle, font: Font, fsize: f32, spacing: f32,
        color: Color, keyword_color: Color,
        fixed_height: bool, line_spacing: f32
    ) -> Self where T: Fn(&str) -> bool {
        let mut words = Vec::new();
        let mut keywords = Vec::new();
        let mut i = 0;

        for word in text.split(|c| { c == ' ' }) {
            let keyword = is_keyword(word);
            words.push(Word::new(word, keyword));

            if keyword {
                keywords.push(i);
            }

            i += 1;
        }

        let mut text_box = TextBox {
            words: words,
            keywords: keywords,
            bbox: bbox,
            color: color,
            keyword_color: keyword_color,
            font: font,
            fsize: fsize,
            spacing: 0.0,
            fixed_height: fixed_height,
            line_spacing: 0.0,
            scroll: 0,
            lines: 0,
            drawn_lines: 0,
        };

        text_box.set_font(font, fsize, spacing, line_spacing);

        text_box
    }

    fn compute_lines(&mut self) {
        let mut x = 0.0;
        let mut line = 0;

        let xspace = self.spacing * self.fsize * 0.5;
        let yspace = self.line_spacing * self.fsize;

        self.drawn_lines = if self.fixed_height {
            (self.bbox.height / yspace) as i32
        } else {
            1024
        };
 
        for word in &mut self.words {
            if x + word.bbox.width > self.bbox.width {
                x = 0.0;
                line += 1;
            }

            if line >= (self.scroll + self.drawn_lines) {
                word.visible = false;
            } else if line < self.scroll {
                word.visible = false;
            } else {
                word.visible = true;
                word.bbox.x = x;
                word.bbox.y = ((line - self.scroll) as f32) * yspace;
            }

            x += word.bbox.width + xspace;
        }

        self.lines = if x == 0.0 { line } else { line + 1 };

        if !self.fixed_height {
            self.bbox.height = (self.lines as f32) * yspace;
        }
    }

    pub fn set_bbox(&mut self, bbox: Rectangle) {
        self.bbox = bbox;
        self.compute_lines();
    }

    pub fn get_bbox(&self) -> Rectangle{
        self.bbox
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.bbox.width = width;
        self.bbox.height = height;
        self.compute_lines();
        self.scroll(0);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.bbox.x = x;
        self.bbox.y = y;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_font(
        &mut self, font: Font, fsize: f32, spacing: f32,
        line_spacing: f32
    ) {
        self.font = font;
        self.fsize = fsize;
        self.spacing = spacing;
        self.line_spacing = line_spacing;

        for word in &mut self.words {
            let size = measure_text_ex(
                self.font, &word.text, self.fsize, self.spacing
            );
            word.bbox.width = size.x;
            word.bbox.height = size.y;
        }

        self.compute_lines();
    }

    pub fn scroll(&mut self, scroll_change: i32) {
        self.scroll += scroll_change;

        if self.scroll > self.lines - self.drawn_lines {
            self.scroll = self.lines - self.drawn_lines;
        }

        if self.scroll < 0 {
            self.scroll = 0;
        }

        self.compute_lines();
    }

    pub fn collision(&self, point: Vector2) -> bool {
        check_collision_point_rec(point, self.bbox)
    }

    pub fn keyword_collision(&self, point: Vector2) -> Option<&str> {
        for &kw_index in &self.keywords {
            let kw = &self.words[kw_index];
            if check_collision_point_rec(point, kw.bbox) {
                return Some(&kw.text);
            }
        }

        None
    }

    pub fn draw(&self) {
        for word in &self.words {
            if word.visible {
                draw_text_ex(
                    self.font,
                    &word.text,
                    Vector2 {
                        x: self.bbox.x + word.bbox.x,
                        y: self.bbox.y + word.bbox.y,
                    },
                    self.fsize,
                    self.spacing,
                    if word.keyword {
                        self.keyword_color
                    } else {
                        self.color
                    }
                );
            }
        }
    }
}

pub struct TextWindow {
    text_box: TextBox,
    bbox: Rectangle, 
    ibox: Rectangle,
    border_width: f32,
    margin: f32,
    background: Color,
    border_color: Color,
}

impl TextWindow {
    pub fn new<T>(
        text: &str, is_keyword: T,
        bbox: Rectangle, margin: f32, border_width: f32,
        background: Color, border_color: Color,
        font: Font, fsize: f32, spacing: f32,
        font_color: Color, keyword_color: Color,
        line_spacing: f32
    ) -> Self where T: Fn(&str) -> bool {
        let space = margin + border_width;
        let text_box = TextBox::new(
            text,
            is_keyword,
            Rectangle {
                x: bbox.x + space,
                y: bbox.y + space,
                width: bbox.width - 2.0 * space,
                height: bbox.height - 2.0 * space,
            },
            font,
            fsize,
            spacing,
            font_color,
            keyword_color,
            true,
            line_spacing
        );

        TextWindow {
            text_box: text_box,
            bbox: bbox,
            ibox: Rectangle {
                x: bbox.x + border_width,
                y: bbox.y + border_width,
                width: bbox.width - 2.0 * border_width,
                height: bbox.height - 2.0 * border_width,
            },
            margin: margin,
            border_width: border_width,
            background: background,
            border_color: border_color,
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.bbox.width = width;
        self.bbox.height = height;
        self.ibox.width = width - 2.0 * self.border_width;
        self.ibox.height = height - 2.0 * self.border_width;

        let space = 2.0 * (self.margin + self.border_width);
        self.text_box.set_size(width - space, height - space);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.bbox.x = x;
        self.bbox.y = y;
        self.ibox.x = x + self.border_width;
        self.ibox.y = y + self.border_width;

        let space = self.margin + self.border_width;
        self.text_box.set_position(x + space, y + space);
    }

    pub fn scroll(&mut self, scroll: i32) {
        self.text_box.scroll(scroll);
    }

    pub fn draw(&self) {
        draw_rectangle_rec(self.bbox, self.border_color);
        draw_rectangle_rec(self.ibox, self.background);
        self.text_box.draw();
    }
}

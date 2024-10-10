use crate::{Color as color, Container, LineFillMode, Style as style};
use colored::*;

#[derive(Clone)]
pub struct Line {
    pub line_content: Container<String>,
    is_filled: bool,
    is_colored: bool,
}

impl Line {
    pub fn new(line_content: String) -> Self {
        Self {
            line_content: Container::Ref(line_content),
            is_filled: false,
            is_colored: false,
        }
    }
    pub fn from_colored_string(line_content: String) -> Self {
        Self {
            line_content: Container::Ref(line_content),
            is_filled: false,
            is_colored: true,
        }
    }
    pub fn fill(&mut self, terminal_size: &(u16, u16), fill_mode: &LineFillMode) {
        if self.is_filled == true {
            return;
        }
        let length = self.line_content.clone().get_value().len();
        let mut fill_size = (terminal_size.0 as i16 - length as i16) as usize;
        if self.is_colored == true {
            fill_size += 17;
        }
        match fill_mode {
            &LineFillMode::Center => {
                match self.line_content.get_mut_value() {
                    a => *a = format!("{}{}", a, " ".to_string().repeat(fill_size / 2)),
                }

                match self.line_content.get_mut_value() {
                    a => *a = format!("{}{}", " ".to_string().repeat(fill_size / 2), a),
                }
            }
            &LineFillMode::Left(p) => {
                match self.line_content.get_mut_value() {
                    a => *a = format!("{}{}", a, " ".to_string().repeat(p)),
                }
                match self.line_content.get_mut_value() {
                    a => *a = format!("{}{}", " ".to_string().repeat(fill_size - p), a),
                }
            }
            &LineFillMode::Right(p) => match self.line_content.get_mut_value() {
                a => {
                    *a = format!("{}{}", a, " ".to_string().repeat(p));
                    *a = format!("{}{}", " ".to_string().repeat(fill_size - p), a);
                }
            },
        }
        self.is_filled = true;
    }
    pub fn debug_view(&mut self) {
        panic!("\nYour Line:{}\n", self.line_content.get_value());
    }
    pub fn merge(&self, other_line: &Line) -> Line {
        let line = self.line_content.clone();
        let mut result = String::new();
        if !self.is_filled {
            panic!("\n\nPENTUI LIB: Your Trying To Merge Unfilled Line.\n\n");
        }

        'line2_iter: for mut character in line.get_value().char_indices() {
            for base_character in other_line.line_content.get_value().char_indices() {
                if character.1 != ' ' {
                    result.insert(character.0, character.1);
                    continue 'line2_iter;
                }
                if base_character.0 != character.0 {
                    continue;
                }
                character.1 = base_character.1;
            }
            result.insert(character.0, character.1);
        }
        Self {
            line_content: Container::Ref(result),
            is_filled: true,
            is_colored: true,
        }
    }

    pub fn paint_line_text(&mut self, color: &color) {
        match color {
            &color::None => (),
            &color::Blue => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().blue().to_string()
            }
            &color::Dark => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().black().to_string()
            }
            &color::Green => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().green().to_string()
            }
            &color::Red => {
                *self.line_content.get_mut_value() = self.line_content.get_value().red().to_string()
            }
            &color::Rgb(r, g, b) => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().truecolor(r, g, b).to_string()
            }
            &color::Orange => {
                *self.line_content.get_mut_value() = self
                    .line_content
                    .get_value()
                    .truecolor(255, 165, 0)
                    .to_string()
            }
            &color::Grey => {
                *self.line_content.get_mut_value() = self
                    .line_content
                    .get_value()
                    .truecolor(128, 128, 128)
                    .to_string()
            }
            &color::Purple => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().purple().to_string()
            }
            &color::Yellow => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().yellow().to_string()
            }
            &color::Cyan => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().cyan().to_string()
            }
        }
        match color {
            &color::None => self.is_colored = false,
            _ => self.is_colored = true,
        }
    }
    pub fn paint_line_background(&mut self, color: &color) {
        match color {
            &color::None => (),
            &color::Blue => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().on_blue().to_string()
            }
            &color::Dark => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().on_black().to_string()
            }
            &color::Green => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().on_green().to_string()
            }
            &color::Red => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().on_red().to_string()
            }
            &color::Rgb(r, g, b) => {
                *self.line_content.get_mut_value() = self
                    .line_content
                    .get_value()
                    .on_truecolor(r, g, b)
                    .to_string();
            }
            &color::Orange => {
                *self.line_content.get_mut_value() = self
                    .line_content
                    .get_value()
                    .on_truecolor(255, 165, 0)
                    .to_string()
            }
            &color::Grey => {
                *self.line_content.get_mut_value() = self
                    .line_content
                    .get_value()
                    .on_truecolor(128, 128, 128)
                    .to_string()
            }
            &color::Purple => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().on_purple().to_string()
            }
            &color::Yellow => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().on_yellow().to_string()
            }
            &color::Cyan => {
                *self.line_content.get_mut_value() =
                    self.line_content.get_value().on_cyan().to_string()
            }
        }
        match color {
            &color::None => self.is_colored = false,
            _ => self.is_colored = true,
        }
    }
    pub fn set_line_style(&mut self, style: &Vec<style>) {
        for style in style.iter() {
            match style {
                &style::Normal => *self.line_content.get_mut_value() = self.line_content.get_value().normal().to_string(),
                &style::Bold => *self.line_content.get_mut_value() = self.line_content.get_value().bold().to_string(),
                &style::Italic => *self.line_content.get_mut_value() = self.line_content.get_value().italic().to_string(),
                &style::UnderLined => *self.line_content.get_mut_value() = self.line_content.get_value().underline().to_string(),
                &style::Strike => *self.line_content.get_mut_value() = self.line_content.get_value().strikethrough().to_string(),
            }
        }
    }
    pub fn paint_string_text(string: &String, color: &color) -> String {
        let mut string = string.clone();
        match color {
            &color::None => (),
            &color::Blue => string = string.blue().to_string(),
            &color::Dark => string = string.black().to_string(),
            &color::Green => string = string.green().to_string(),
            &color::Red => string = string.red().to_string(),
            &color::Rgb(r, g, b) => string = string.truecolor(r, g, b).to_string(),
            &color::Orange => string = string.truecolor(255, 165, 0).to_string(),
            &color::Grey => string = string.truecolor(128, 128, 128).to_string(),
            &color::Purple => string = string.purple().to_string(),
            &color::Yellow => string = string.yellow().to_string(),
            &color::Cyan => string = string.cyan().to_string(),
        }
        string
    }
    pub fn paint_string_background(string: &String, color: &color) -> String {
        let mut string = string.clone();
        match color {
            &color::None => (),
            &color::Blue => string = string.on_blue().to_string(),
            &color::Dark => string = string.on_black().to_string(),
            &color::Green => string = string.on_green().to_string(),
            &color::Red => string = string.on_red().to_string(),
            &color::Rgb(r, g, b) => {
                string = string.on_truecolor(r, g, b).to_string();
            }
            &color::Orange => string = string.on_truecolor(255, 165, 0).to_string(),
            &color::Grey => string = string.on_truecolor(128, 128, 128).to_string(),
            &color::Purple => string = string.on_purple().to_string(),
            &color::Yellow => string = string.on_yellow().to_string(),
            &color::Cyan => string = string.on_cyan().to_string(),
        }
        string
    }
    pub fn set_string_style(string: &String, style: &Vec<style>) -> String {
        let mut string = string.clone();
        for style in style.iter() {
            match style {
                &style::Normal => string = string.normal().to_string(),
                &style::Bold => string = string.bold().to_string(),
                &style::Italic => string = string.italic().to_string(),
                &style::UnderLined => string = string.underline().to_string(),
                &style::Strike => string = string.strikethrough().to_string(),
            }
        }
        string
    }
}

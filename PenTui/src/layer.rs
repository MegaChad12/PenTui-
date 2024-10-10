use crate::{Color, Container, LayerFillMode, Line, Style};

#[derive(Clone)]
pub struct Layer {
    pub layer_lines: Container<Vec<Line>>,
    is_filled: bool,
    is_widget: bool,
}

impl Layer {
    pub fn new(lines: Vec<Line>) -> Self {
        Self {
            layer_lines: Container::Ref(lines),
            is_filled: false,
            is_widget: false,
        }
    }
    pub fn new_static_widget(&mut self){
        self.is_widget = true
    }
    pub fn set(&mut self, lines: Vec<Line>) {
        self.layer_lines = Container::Ref(lines);
    }
    pub fn fill(&mut self, terminal_size: &(u16, u16), fill_mode: &LayerFillMode) {
        if self.is_widget {
            self.is_filled = true;
            return;
        }

        let length = self.layer_lines.get_value().len() as usize;
        let fill_size = (terminal_size.1 as i16 - length as i16) as i16;
        self.is_filled = true;
        match fill_mode {
            &LayerFillMode::Center => {
                for _ in 0..fill_size / 2 {
                    let mut line = Line::new("".to_string());
                    line.fill(terminal_size, &crate::LineFillMode::Center);
                    self.layer_lines.get_mut_value().insert(0, line);
                }
                for _ in 0..fill_size / 2 {
                    let mut line = Line::new("".to_string());
                    line.fill(terminal_size, &crate::LineFillMode::Center);
                    self.layer_lines.get_mut_value().push(line);
                }
            }
            &LayerFillMode::Up(p) => {
                let mut line = Line::new("".to_string());
                line.fill(terminal_size, &crate::LineFillMode::Center);
                for _ in 0..p {
                    self.layer_lines.get_mut_value().insert(0, line.clone());
                }
                for _ in 0..fill_size - p as i16 {
                    self.layer_lines.get_mut_value().push(line.clone());
                }
            }
            &LayerFillMode::Down(p) => {
                let mut line = Line::new("".to_string());
                line.fill(terminal_size, &crate::LineFillMode::Center);
                for _ in 0..p {
                    self.layer_lines.get_mut_value().push(line.clone());
                }
                for _ in 0..fill_size - 1 - p as i16{
                    self.layer_lines.get_mut_value().insert(0, line.clone());
                }
            }
        }
    }
    pub fn debug_view(&self) {
        for line in self.layer_lines.get_value().iter() {
            println!("{}", line.line_content.get_value());
        }
        panic!("\n   ^\nYour Layer.\n");
    }
    pub fn merge(&self, other_layer: &Layer, terminal_size: &(u16, u16)) -> Layer {
        if !self.is_filled || !other_layer.is_filled {
            panic!("\n\nTUI LIB : Layers must be filled before merging.\n\n");
        }
        let mut result_layer = Layer::new(vec![]);
        result_layer.fill(&terminal_size, &LayerFillMode::Center);
        for i in 1..terminal_size.1 as usize {
            result_layer.layer_lines.get_mut_value()[i - 1] =
                self.layer_lines.get_value()[i - 1].merge(&other_layer.layer_lines.get_value()[i - 1]);
        }
        result_layer
    }
    pub fn paint_layer_text(&mut self, color: &Color) {
        for line in self.layer_lines.get_mut_value().iter_mut() {
            line.paint_line_text(color);
        }
    }
    pub fn paint_layer_background(&mut self, color: &Color) {
        for line in self.layer_lines.get_mut_value().iter_mut() {
            line.paint_line_background(color);
        }
    }
    pub fn set_layer_style(&mut self, style: &Vec<Style>) {
        if self.is_widget == true {
            return;
        }
        for line in self.layer_lines.get_mut_value().iter_mut() {
            line.set_line_style(style);
        }
    }
    pub fn manipulate_line(&mut self,index:usize) -> &mut Line{
        if self.layer_lines.get_value().len() < index {
            println!("PenTui: Index Out Of Bounds.\nPenTui: Error At Function manipulate_line().");
            std::process::exit(-3);
        }
        &mut self.layer_lines.get_mut_value()[index]
    }
}

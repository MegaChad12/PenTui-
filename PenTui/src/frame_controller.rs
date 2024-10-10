use crate::{Color, Container, Layer, ManagmentMode, Style};
use crossterm::{
    cursor,
    terminal::{self, size},
    ExecutableCommand,
};
use std::{
    io::{Stdout, Write},
    sync::mpsc,
    thread,
};

pub struct FrameController {
    pub content_layers: Container<Vec<Layer>>,
    delay: std::time::Duration,
    //                width  height
    //                  v      v
    pub terminal_size: Container<(u16, u16)>,
    terminal_size_management: ManagmentMode,
    stdout: Option<Stdout>,
    pub result_frame: Layer,
}

impl FrameController {
    pub fn new() -> Self {
        Self {
            content_layers: Container::Ref(vec![]),
            delay: std::time::Duration::from_millis(0),
            terminal_size: Container::Ref(size().unwrap()),
            terminal_size_management: ManagmentMode::Auto,
            stdout: None,
            result_frame: Layer::new(vec![]),
        }
    }
    pub fn set_values(
        &mut self,
        delay: Option<std::time::Duration>,
        content_layers: Option<Vec<Layer>>,
        stdout: Option<Stdout>,
    ) {
        match content_layers {
            Some(c) => *self.content_layers.get_mut_value() = c,
            None => (),
        };
        match delay {
            Some(d) => self.delay = d,
            None => (),
        }
        match stdout {
            Some(a) => self.stdout = Some(a),
            None => (),
        }
    }
    pub fn size_managment_mode(&mut self, mode: ManagmentMode) {
        self.terminal_size_management = mode;
    }
    pub fn set_terminal_to_current_size(&mut self) {
        self.terminal_size = Container::Ref(size().unwrap());
    }
    pub fn draw(&mut self) {
        match self.terminal_size_management {
            ManagmentMode::Auto => {
                if self.terminal_size.get_value().0 <= 10 || self.terminal_size.get_value().1 <= 10 {
                    println!("\n\n Hey User, Try to make the terminal window Larger.\n\n");
                    std::process::exit(-2);
                }
                let mut i = 0 as u16;
                for line in self.result_frame.layer_lines.get_value().iter() {
                    if i == 0 {
                        println!("{}", line.line_content.get_value());

                        continue;
                    }
                    i += 1;

                    print!("{}", line.line_content.get_value());
                }
            }
            ManagmentMode::Manual(min, max) => {
                if self.terminal_size.get_value().0 >= max || self.terminal_size.get_value().1 >= max {
                    panic!("\n\n Hey User, Try to make the terminal window Smaller.\n\n");
                }
                if self.terminal_size.get_value().0 <= min || self.terminal_size.get_value().1 <= min {
                    panic!("\n\n Hey User, Try to make the terminal window Larger.\n\n");
                }
                let mut i = 0 as u16;
                for line in self.result_frame.layer_lines.get_value().iter() {
                    if i == 0 {
                        println!("{}", line.line_content.get_value());

                        continue;
                    }
                    i += 1;
                    print!("{}", line.line_content.get_value());
                }
            }
        }
    }
    pub fn wait(&self) {
        std::thread::sleep(self.delay);
    }

    pub fn clear_terminal(&mut self) {
        self.stdout
            .as_mut()
            .unwrap()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap()
            .execute(cursor::MoveTo(0, 0))
            .unwrap();
        self.stdout.as_mut().unwrap().write_all(b"\x1B[3J").unwrap();
        self.stdout.as_mut().unwrap();
        self.stdout.as_mut().unwrap().flush().unwrap();
    }

    pub fn merge_layers(&mut self) {
        let length = self.content_layers.get_value().len();
        let mut layers = self.content_layers.clone();
        let terminal_size = self.terminal_size.clone();
        let (tx, rx) = mpsc::channel();

        let merging_sub_thread = thread::spawn(move || {
            for i in 0..(length / 2) {
                layers.get_mut_value()[i + 1] = layers.get_value()[i + 1].merge(&layers.get_value()[i], terminal_size.get_value());
            }
            tx.send(layers.get_value()[length / 2].clone()).unwrap();
        });
        let result;
        for i in (length / 2)+1 as usize..(length - 1) as usize {
            self.content_layers.get_mut_value()[i + 1] =
                self.content_layers.get_value()[i + 1].merge(&self.content_layers.get_value()[i], self.terminal_size.get_value());
        }
        result = self.content_layers.get_value()[length - 1].clone();
        merging_sub_thread.join().unwrap();
        let thread_result = rx.recv().unwrap();
        self.result_frame = result.merge(&thread_result, self.terminal_size.get_value());
    }
    pub fn paint_result_layer_line(&mut self, color: &Color) {
        self.result_frame.paint_layer_text(color);
    }
    pub fn paint_result_layer_background(&mut self, color: &Color) {
        self.result_frame.paint_layer_background(color);
    }
    pub fn set_result_layer_style(&mut self, style: &Vec<Style>) {
        self.result_frame.set_layer_style(style);
    }
    pub fn set_terminal_size(&mut self, size: (u16, u16)) {
        *self.terminal_size.get_mut_value() = size;
    }
    pub fn manipulate_layer(&mut self,index:usize) -> &mut Layer{
        if self.content_layers.get_value().len() < index {
            println!("PenTui: Index Out Of Bounds.\nPenTui: Error At Function manipulate_line().");
            std::process::exit(-3);
        }
        &mut self.content_layers.get_mut_value()[index]
    }
}

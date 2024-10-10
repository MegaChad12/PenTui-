use crate::{Color, FrameController, Layer, LayerFillMode, Line, LineFillMode, Style};

pub enum BorderAnim {
    Cycle,
    LeftAndRight,
    UpDown,
}

#[derive(Clone)]
pub enum OptionRefString<'a> {
    Ref([&'a String; 5]),
    SimpleOwnerShip([String; 5]),
}
#[derive(Clone)]
pub struct Border<'a> {
    pub layer: &'a Layer,
    pub border_chars: OptionRefString<'a>,
    pub border_chars_color: Option<[Option<&'a Color>; 5]>,
    pub border_background_color: Option<[Option<&'a Color>; 5]>,
    pub border_style: Option<[Option<&'a Vec<Style>>; 5]>,
    pub border_size: &'a (u16, u16),
}
impl<'a> Border<'a> {
    pub fn make_border_widget(
        frame_ctr: &'a FrameController,
        layer: &'a mut Layer,
        chars: [String; 5],
    ) -> Self {
        let terminal_size = frame_ctr.terminal_size.get_value();

        layer.new_static_widget();
        for i in 0..(terminal_size.1 - 1) as usize {
            if i == 0 {
                let top_border =
                    format!("{}{}{}", ".", "-".repeat(terminal_size.0 as usize - 2), ".");
                let mut line = Line::new(top_border);
                line.fill(terminal_size, &LineFillMode::Center);

                layer.layer_lines.get_mut_value().push(line);
                continue;
            }
            if i == (terminal_size.1 - 2) as usize {
                let bottom_border =
                    format!("{}{}{}", "'", "-".repeat(terminal_size.0 as usize - 2), "'");
                let mut line = Line::new(bottom_border);
                line.fill(terminal_size, &LineFillMode::Center);

                layer.layer_lines.get_mut_value().push(line);
                continue;
            }
            let middle_border =
                format!("{}{}{}", "|", " ".repeat(terminal_size.0 as usize - 2), "|");
            let mut line = Line::new(middle_border);
            line.fill(terminal_size, &LineFillMode::Center);
            layer.layer_lines.get_mut_value().push(line);
        }
        layer.fill(terminal_size, &crate::LayerFillMode::Center);
        let optional_ref = OptionRefString::SimpleOwnerShip(chars);
        Self {
            layer,
            border_chars: optional_ref,
            border_chars_color: None,
            border_background_color: None,
            border_style: None,
            border_size: &terminal_size,
        }
    }
    pub fn make_configurable_border_widget(
        frame_ctr: &'a FrameController,
        text_color: [Option<&'a Color>; 5],
        text_background_color: [Option<&'a Color>; 5],
        style: [Option<&'a Vec<Style>>; 5],
        //          top-corners top  middle down-corner down
        //               1       2     3        4        5
        border_chars: [&'a String; 5],
        layer: &'a mut Layer,
    ) -> Self {
        let terminal_size = frame_ctr.terminal_size.get_value();

        layer.new_static_widget();

        for i in 0..(terminal_size.1 - 1) as usize {
            if i == 0 {
                let text_color = (
                    match text_color[0] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                    match text_color[1] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                );
                let text_background_color = (
                    match text_background_color[0] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                    match text_background_color[1] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                );
                let style = (
                    match style[0] {
                        Some(s) => s,
                        None => &vec![Style::Normal],
                    },
                    match style[0] {
                        Some(s) => s,
                        None => &vec![Style::Normal],
                    },
                );
                let top_border = format!(
                    "{}{}{}",
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[0], text_color.0),
                            text_background_color.0
                        ),
                        style.0
                    ),
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[1], text_color.1),
                            text_background_color.1
                        ),
                        style.1
                    )
                    .repeat(terminal_size.0 as usize - 2),
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[0], text_color.0),
                            text_background_color.0
                        ),
                        style.0
                    )
                );
                let mut line = Line::new(top_border);
                line.fill(terminal_size, &LineFillMode::Center);

                layer.layer_lines.get_mut_value().push(line);
                continue;
            }
            if i == (terminal_size.1 - 2) as usize {
                let text_color = (
                    match text_color[3] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                    match text_color[4] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                );
                let text_background_color = (
                    match text_background_color[3] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                    match text_background_color[4] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                );
                let style = (
                    match style[3] {
                        Some(s) => s,
                        None => &vec![Style::Normal],
                    },
                    match style[4] {
                        Some(s) => s,
                        None => &vec![Style::Normal],
                    },
                );
                let bottom_border = format!(
                    "{}{}{}",
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[3], text_color.0),
                            text_background_color.0
                        ),
                        style.0
                    ),
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[4], text_color.1),
                            text_background_color.1
                        ),
                        style.1
                    )
                    .repeat(terminal_size.0 as usize - 2),
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[3], text_color.0),
                            text_background_color.0
                        ),
                        style.0
                    )
                );
                let mut line = Line::new(bottom_border);
                line.fill(terminal_size, &LineFillMode::Center);

                layer.layer_lines.get_mut_value().push(line);
                continue;
            }
            let text_color = match text_color[2] {
                Some(c) => c,
                None => &Color::None,
            };
            let text_background_color = match text_background_color[2] {
                Some(c) => c,
                None => &Color::None,
            };
            let style = match style[2] {
                Some(s) => s,
                None => &vec![Style::Normal],
            };
            let middle_border = format!(
                "{}{}{}",
                Line::set_string_style(
                    &Line::paint_string_background(
                        &Line::paint_string_text(border_chars[2], text_color),
                        text_background_color
                    ),
                    style
                ),
                " ".repeat(terminal_size.0 as usize - 2),
                Line::set_string_style(
                    &Line::paint_string_background(
                        &Line::paint_string_text(border_chars[2], text_color),
                        text_background_color
                    ),
                    style
                )
            );
            let mut line = Line::new(middle_border);
            line.fill(terminal_size, &LineFillMode::Center);

            layer.layer_lines.get_mut_value().push(line);
        }
        layer.fill(terminal_size, &crate::LayerFillMode::Center);
        Self {
            layer,
            border_background_color: Some(text_background_color),
            border_chars_color: Some(text_color),
            border_chars: OptionRefString::Ref(border_chars),
            border_style: Some(style),
            border_size: &terminal_size,
        }
    }

    pub fn make_configurable_manual_border(
        frame_ctr: &FrameController,
        text_color: [Option<&'a Color>; 5],
        text_background_color: [Option<&'a Color>; 5],
        style: [Option<&'a Vec<Style>>; 5],
        //          top-corners top  middle down-corner down
        //               1       2     3        4        5
        border_chars: [&'a String; 5],
        layer: &'a mut Layer,
        border_size: &'a (u16, u16),
        layer_fill_mode: &LayerFillMode,
        line_fill_mode: &LineFillMode,
    ) -> Self {
        let terminal_size = border_size;
        let real_terminal_size = frame_ctr.terminal_size.get_value();

        for i in 0..(terminal_size.1 - 1) as usize {
            if i == 0 {
                let text_color = (
                    match text_color[0] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                    match text_color[1] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                );
                let text_background_color = (
                    match text_background_color[0] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                    match text_background_color[1] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                );
                let style = (
                    match style[0] {
                        Some(s) => s,
                        None => &vec![Style::Normal],
                    },
                    match style[0] {
                        Some(s) => s,
                        None => &vec![Style::Normal],
                    },
                );
                let top_border = format!(
                    "{}{}{}",
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[0], text_color.0),
                            text_background_color.0
                        ),
                        style.0
                    ),
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[1], text_color.1),
                            text_background_color.1
                        ),
                        style.1
                    )
                    .repeat(terminal_size.0 as usize - 2),
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[0], text_color.0),
                            text_background_color.0
                        ),
                        style.0
                    )
                );
                let mut line = Line::from_colored_string(top_border);
                line.fill(real_terminal_size, line_fill_mode);
                layer.layer_lines.get_mut_value().push(line);
                continue;
            }
            if i == (terminal_size.1 - 2) as usize {
                let text_color = (
                    match text_color[3] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                    match text_color[4] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                );
                let text_background_color = (
                    match text_background_color[3] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                    match text_background_color[4] {
                        Some(c) => c,
                        None => &Color::None,
                    },
                );
                let style = (
                    match style[3] {
                        Some(s) => s,
                        None => &vec![Style::Normal],
                    },
                    match style[4] {
                        Some(s) => s,
                        None => &vec![Style::Normal],
                    },
                );
                let bottom_border = format!(
                    "{}{}{}",
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[3], text_color.0),
                            text_background_color.0
                        ),
                        style.0
                    ),
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[4], text_color.1),
                            text_background_color.1
                        ),
                        style.1
                    )
                    .repeat(terminal_size.0 as usize - 2),
                    Line::set_string_style(
                        &Line::paint_string_background(
                            &Line::paint_string_text(border_chars[3], text_color.0),
                            text_background_color.0
                        ),
                        style.0
                    )
                );
                let mut line = Line::from_colored_string(bottom_border);
                line.fill(real_terminal_size, line_fill_mode);
                layer.layer_lines.get_mut_value().push(line);
                continue;
            }
            let text_color = match text_color[2] {
                Some(c) => c,
                None => &Color::None,
            };
            let text_background_color = match text_background_color[2] {
                Some(c) => c,
                None => &Color::None,
            };
            let style = match style[2] {
                Some(s) => s,
                None => &vec![Style::Normal],
            };
            let middle_border = format!(
                "{}{}{}",
                Line::set_string_style(
                    &Line::paint_string_background(
                        &Line::paint_string_text(border_chars[2], text_color),
                        text_background_color
                    ),
                    style
                ),
                " ".repeat(terminal_size.0 as usize - 2),
                Line::set_string_style(
                    &Line::paint_string_background(
                        &Line::paint_string_text(border_chars[2], text_color),
                        text_background_color
                    ),
                    style
                )
            );
            let mut line = Line::from_colored_string(middle_border);
            line.fill(real_terminal_size, line_fill_mode);
            layer.layer_lines.get_mut_value().push(line);
        }
        layer.fill(real_terminal_size, layer_fill_mode);
        Self {
            layer,
            border_chars: OptionRefString::Ref(border_chars),
            border_chars_color: Some(text_color),
            border_background_color: Some(text_background_color),
            border_style: Some(style),
            border_size: &border_size,
        }
    }
    pub fn make_simple_manual_border(
        frame_ctr: &FrameController,
        border_size: &'a (u16, u16),
        layer_fill_mode: &'a LayerFillMode,
        line_fill_mode: &'a LineFillMode,
        layer: &'a mut Layer,
    ) -> Self {
        let terminal_size = border_size;
        let real_terminal_size = frame_ctr.terminal_size.get_value();

        for i in 0..(terminal_size.1 - 1) as usize {
            if i == 0 {
                let top_border =
                    format!("{}{}{}", ".", "-".repeat(terminal_size.0 as usize - 2), ".");
                let mut line = Line::new(top_border);
                line.fill(real_terminal_size, line_fill_mode);
                layer.layer_lines.get_mut_value().push(line);
                continue;
            }
            if i == (terminal_size.1 - 2) as usize {
                let bottom_border =
                    format!("{}{}{}", "'", "-".repeat(terminal_size.0 as usize - 2), "'");
                let mut line = Line::new(bottom_border);
                line.fill(&real_terminal_size, line_fill_mode);
                layer.layer_lines.get_mut_value().push(line);
                continue;
            }
            let middle_border =
                format!("{}{}{}", "|", " ".repeat(terminal_size.0 as usize - 2), "|");
            let mut line = Line::new(middle_border);
            line.fill(&real_terminal_size, line_fill_mode);
            layer.layer_lines.get_mut_value().push(line);
        }
        layer.fill(&real_terminal_size, layer_fill_mode);
        Self {
            layer,
            border_chars: OptionRefString::SimpleOwnerShip([
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ]),
            border_chars_color: None,
            border_background_color: None,
            border_style: None,
            border_size: &border_size,
        }
    }
    pub fn set_border_values(
        &mut self,
        layer: Option<&'a Layer>,
        border_chars: Option<OptionRefString<'a>>,
        border_chars_color: Option<Option<[Option<&'a Color>; 5]>>,
        border_background_color: Option<Option<[Option<&'a Color>; 5]>>,
        border_style: Option<Option<[Option<&'a Vec<Style>>; 5]>>,
        border_size: Option<&'a (u16, u16)>,
    ) {
        match layer {
            Some(a) => self.layer = a,
            None => (),
        }
        match border_chars {
            Some(a) => self.border_chars = a,
            None => (),
        }
        match border_background_color {
            Some(a) => self.border_background_color = a,
            None => (),
        }
        match border_chars_color {
            Some(a) => self.border_chars_color = a,
            None => (),
        }
        match border_style {
            Some(a) => self.border_style = a,
            None => (),
        }
        match border_size {
            Some(a) => self.border_size = a,
            None => (),
        }
    }
}

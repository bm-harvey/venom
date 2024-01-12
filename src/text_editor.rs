use ratatui::prelude::*;
use ratatui::widgets::Widget;

#[derive(Default, Clone)]
pub struct TextEditor {
    text: Vec<Vec<char>>,
    cursor_loc: (usize, usize),
}

impl TextEditor {
    pub fn new(text: &str) -> Self {
        let text = text
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        Self {
            text,
            cursor_loc: (0, 0),
        }
    }

    pub fn lines(&self) -> impl Iterator<Item = &Vec<char>> {
        self.text.iter()
    }

    pub fn widget(&self) -> TextEditorWidget {
        TextEditorWidget {
            text: self.text.clone(),
            cursor_loc: self.cursor_loc,
        }
    }

    pub fn move_left(&mut self) -> &mut Self {
        if self.cursor_loc.0 > 0 {
            self.cursor_loc.0 -= 1;
        }
        self
    }
    pub fn move_right(&mut self) -> &mut Self {
        if self.cursor_loc.0 < self.text[self.cursor_loc.1].len() - 1 {
            self.cursor_loc.0 += 1;
        }
        self
    }
    pub fn move_down(&mut self) -> &mut Self {
        if self.cursor_loc.1 < self.text.len() - 1 {
            self.cursor_loc.1 += 1;
        }
        self
    }
    pub fn move_up(&mut self) -> &mut Self {
        if self.cursor_loc.1 > 0 {
            self.cursor_loc.1 -= 1;
        }
        self
    }
}

pub struct TextEditorWidget {
    text: Vec<Vec<char>>,
    cursor_loc: (usize, usize),
}

impl TextEditorWidget {
    pub fn lines(&self) -> impl Iterator<Item = &Vec<char>> {
        self.text.iter()
    }
}

impl Widget for TextEditorWidget {
    fn render(self, _area: Rect, _buf: &mut Buffer) {
        for (line_idx, line) in self.lines().enumerate() {
            for (c_idx, c) in line.iter().enumerate() {
                let style = if (c_idx, line_idx) == self.cursor_loc {
                    Style::default().underlined()
                } else {
                    Style::default()
                };

                _buf.set_string(
                    _area.left() + c_idx as u16,
                    _area.top() + line_idx as u16,
                    c.to_string(),
                    style,
                );

                //let x_pos = self.cursor_loc.0;
                //let before_cursor = Span::raw(line[..x_pos].iter().collect::<String>());
                //let cursor = Span::styled(line[x_pos].to_string(), Style::default().underlined());
                //let after_cursor = Span::raw(line[x_pos + 1..].iter().collect::<String>());
            }
        }
    }
}

use edtui::{EditorState, EditorView, Lines};
use std::cell::RefCell;
use std::rc::Rc;

use crate::task::TaskLabel;

/// Popup to edit tasks field-by-field
#[derive(Default)]
pub struct EditLabelsPopup {
    text_editor: EditorState,
}
impl EditLabelsPopup {
    pub fn load_text(&mut self, text: &str) -> &mut Self {
        self.text_editor = EditorState::new(Lines::from(text));
        self
    }

    pub fn load_labels(&mut self, labels: &[Rc<RefCell<TaskLabel>>]) -> &mut Self {
        let mut text = String::new();
        labels.iter().for_each(|label| {
            if !text.is_empty() {
                text.push('\n')
            }

            let label = label.borrow();
            let color = label.color_string();
            let line = format!(
                "{} {} {}",
                label.short_name().iter().collect::<String>(),
                color,
                label.long_name()
            );
            text.push_str(&line);
        });

        self.load_text(&text);
        self
    }

    pub fn text_editor_widget(&mut self) -> edtui::EditorView {
        EditorView::new(self.text_editor_mut())
    }

    pub fn text_editor_mut(&mut self) -> &mut EditorState {
        &mut self.text_editor
    }
    pub fn text_editor(&self) -> &EditorState {
        &self.text_editor
    }
}

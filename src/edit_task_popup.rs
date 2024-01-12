use edtui::{EditorState, EditorView, Lines};
use crate::app::EditableTaskProperty;
use strum::IntoEnumIterator;

/// Popup to edit tasks field-by-field
#[derive(Default)]
pub struct EditTaskPopup {
    property: EditableTaskProperty,
    text_editor: EditorState,
    focus: EditTaskFocus,
}

/// Current focus of the Task Editor Popup
#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub enum EditTaskFocus {
    #[default]
    Fields,
    Edit,
}

impl EditTaskPopup {
    pub fn property(&self) -> EditableTaskProperty {
        self.property
    }

    pub fn focus(&self) -> EditTaskFocus {
        self.focus
    }
    pub fn set_focus(&mut self, focus: EditTaskFocus) -> &mut Self {
        self.focus = focus;
        self
    }

    pub fn set_property(&mut self, property: EditableTaskProperty) -> &mut Self {
        self.property = property;
        self
    }

    pub fn decrement_property(&mut self) -> &mut Self {
        let properties = EditableTaskProperty::iter().collect::<Vec<_>>();
        let current_idx = properties
            .iter()
            .position(|&p| p == self.property())
            .unwrap();
        let idx = if current_idx == 0 {
            properties.len() - 1
        } else {
            current_idx - 1
        };

        self.property = properties[idx];
        self
    }
    pub fn increment_property(&mut self) -> &mut Self {
        let properties = EditableTaskProperty::iter().collect::<Vec<_>>();
        let current_idx = properties
            .iter()
            .position(|&p| p == self.property())
            .unwrap();
        let idx = if current_idx == properties.len() - 1 {
            0
        } else {
            current_idx + 1
        };
        self.property = properties[idx];
        self
    }

    pub fn load_text(&mut self, text: &str) -> &mut Self {
        self.text_editor = EditorState::new(Lines::from(text));
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

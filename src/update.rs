use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::input::key;
use edtui::Input;

use crate::app::App;
use crate::app::AppMode;
use crate::app::EditTaskFocus;

type KC = KeyCode;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.mode() {
        AppMode::TaskView => {
            match key_event.code {
                KC::Esc | KC::Char('q') => app.quit(),
                KC::Char('c') | KC::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit()
                    }
                }
                KC::Down | KC::Char('j') => app.increment_task_idx(),
                KC::Up | KC::Char('k') => app.decrement_task_idx(),
                KC::Char('a') => app.add_task(),
                KC::Char('e') => app.edit_task(),
                _ => {}
            };
        }
        AppMode::EditPopup => {
            let popup = app.edit_task_popup_mut();
            if popup.is_none() {
                return;
            }
            let popup = popup.as_mut().unwrap();
            let focus = popup.focus();
            match focus {
                EditTaskFocus::Edit => match key_event.code {
                    KC::Esc | KC::Char('q') => {
                        popup.set_focus(EditTaskFocus::Fields);
                    }
                    _ => {
                        let state = popup.text_editor_mut();
                        let mut input = Input::default();
                        input.on_key(key_event, state);
                    }
                },

                EditTaskFocus::Fields => {
                    match key_event.code {
                        KC::Esc | KC::Char('q') => {
                            app.set_edit_task_popup(None);
                            app.set_mode(AppMode::TaskView);
                        }
                        KC::Char('c') | KC::Char('C') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                app.set_edit_task_popup(None);
                                app.set_mode(AppMode::TaskView);
                            }
                        }
                        KC::Down | KC::Char('j') => {
                            popup.increment_property();
                            let property = popup.property();
                            let text = app.selected_task().text_to_edit(property.clone());
                            popup.load_text(&text);
                        }
                        KC::Up | KC::Char('k') => {
                            popup.decrement_property();
                            let text = app.selected_task().text_to_edit(popup.property());
                            popup.load_text(&text);
                        }
                        KC::Enter => {
                            let popup = app.edit_task_popup_mut();
                            if let Some(popup) = popup {
                                match popup.focus() {
                                    EditTaskFocus::Fields => {
                                        popup.set_focus(EditTaskFocus::Edit);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}

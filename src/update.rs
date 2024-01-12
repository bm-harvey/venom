use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::Input;

use crate::app::Venom;
use crate::app::VenomFocus;
use crate::edit_task_popup::EditTaskFocus;

type KC = KeyCode;

pub fn update(app: &mut Venom, key_event: KeyEvent) {
    match app.focus() {
        VenomFocus::MainView => {
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
        VenomFocus::EditPopup(popup) => {
            let focus = popup.borrow().focus();
            match focus {
                EditTaskFocus::Edit => match key_event.code {
                    KC::Esc => {
                        if popup.borrow().text_editor().mode == edtui::EditorMode::Normal {
                            popup.borrow_mut().set_focus(EditTaskFocus::Fields);
                        } else {
                            popup.borrow_mut().text_editor_mut().mode = edtui::EditorMode::Normal
                        }
                    }
                    KC::Char('c') => {
                        if key_event.modifiers == KeyModifiers::CONTROL {
                            if popup.borrow().text_editor().mode == edtui::EditorMode::Normal {
                                popup.borrow_mut().set_focus(EditTaskFocus::Fields);
                            } else {
                                popup.borrow_mut().text_editor_mut().mode =
                                    edtui::EditorMode::Normal
                            }
                        }
                    }
                    _ => {
                        let mut input = Input::default();
                        input.on_key(key_event, popup.borrow_mut().text_editor_mut());
                    }
                },
                EditTaskFocus::Fields => {
                    match key_event.code {
                        KC::Esc | KC::Char('q') => {
                            app.set_mode(VenomFocus::MainView);
                        }
                        KC::Char('c') | KC::Char('C') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                app.set_mode(VenomFocus::MainView);
                            }
                        }
                        KC::Down | KC::Char('j') => {
                            popup.borrow_mut().increment_property();
                            let property = popup.borrow().property();
                            let text = app.selected_task().text_to_edit(property);
                            popup.borrow_mut().load_text(&text);
                        }
                        KC::Up | KC::Char('k') => {
                            popup.borrow_mut().decrement_property();
                            let text = app.selected_task().text_to_edit(popup.borrow().property());
                            popup.borrow_mut().load_text(&text);
                        }
                        KC::Enter => {
                            popup.borrow_mut().set_focus(EditTaskFocus::Edit);
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}

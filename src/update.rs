use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::Input;
//use serde_json::json;

use crate::app::Venom;
use crate::app::VenomFocus;
use crate::edit_task_popup::EditTaskFocus;

type KC = KeyCode;
type KM = KeyModifiers;

pub fn update(app: &mut Venom, key_event: KeyEvent) {
    match app.focus() {
        VenomFocus::MainView => {
            match key_event.code {
                KC::Esc | KC::Char('q') => app.quit(),
                KC::Char('c') => {
                    if key_event.modifiers == KM::CONTROL {
                        app.quit()
                    }
                }
                KC::Down | KC::Char('j') => app.increment_task_idx(),
                KC::Up | KC::Char('k') => app.decrement_task_idx(),
                KC::Char('a') => app.add_task(),
                KC::Char('e') | KC::Enter => app.edit_task(),
                KC::Char(' ') => {
                    app.toggle_selected_task();
                    app.save_file();
                }
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
                            let mut text = String::new();
                            let mut previous_row = 0;
                            popup.borrow().text_editor().lines.iter().for_each(|c| {
                                let (c, idx) = c;
                                if idx.row > previous_row {
                                    previous_row = idx.row;
                                    text.push('\n');
                                }
                                if let Some(c) = c {
                                    text.push(*c);
                                }
                            });
                            app.selected_task()
                                .borrow_mut()
                                .set_property_from_str(popup.borrow().property(), &text);
                            // write save file
                            app.save_file();
                        } else {
                            popup.borrow_mut().text_editor_mut().mode = edtui::EditorMode::Normal
                        }
                    }
                    KC::Char('c') => {
                        if key_event.modifiers == KM::CONTROL {
                            if popup.borrow().text_editor().mode == edtui::EditorMode::Normal {
                                popup.borrow_mut().set_focus(EditTaskFocus::Fields);
                                let mut text = String::new();
                                let mut previous_row = 0;
                                popup.borrow().text_editor().lines.iter().for_each(|c| {
                                    let (c, idx) = c;
                                    if idx.row > previous_row {
                                        previous_row = idx.row;
                                        text.push('\n');
                                    }
                                    if let Some(c) = c {
                                        text.push(*c);
                                    }
                                });

                                let task = app.selected_task();
                                match popup.borrow().property() {
                                    crate::app::EditableTaskProperty::Label => {
                                        let label = app.task_db().label_by_tag(&text);
                                        task.borrow_mut().set_label(label);
                                    }
                                    _ => {
                                        task.borrow_mut().set_property_from_str(
                                            popup.borrow().property(),
                                            &text,
                                        );
                                    }
                                }
                                app.save_file();
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
                            if key_event.modifiers == KM::CONTROL {
                                app.set_mode(VenomFocus::MainView);
                            }
                        }
                        KC::Down | KC::Char('j') => {
                            popup.borrow_mut().increment_property();
                            let property = popup.borrow().property();
                            let text = app.selected_task().borrow().text_to_edit(property);
                            popup.borrow_mut().load_text(&text);
                        }
                        KC::Up | KC::Char('k') => {
                            popup.borrow_mut().decrement_property();
                            let text = app
                                .selected_task()
                                .borrow()
                                .text_to_edit(popup.borrow().property());
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

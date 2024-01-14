use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::EditorMode;
use edtui::Input;
use std::cell::RefCell;
use std::rc::Rc;
//use serde_json::json;

use crate::app::Venom;
use crate::app::VenomFocus;
use crate::edit_task_popup::EditTaskFocus;
use crate::task::TaskLabel;

type KC = KeyCode;
type KM = KeyModifiers;

pub fn update(app: &mut Venom, key_event: KeyEvent) {
    let focus = app.focus().clone();
    match focus {
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
                KC::Char('l') => app.edit_labels(),
                KC::Char('d') => {
                    app.remove_selected_task();
                    app.save_file();
                }
                KC::Char(' ') => {
                    app.toggle_selected_task();
                    app.save_file();
                }
                _ => {}
            };
        }
        VenomFocus::EditTaskPopup(popup) => {
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
                        } else {
                            let mut input = Input::default();
                            input.on_key(key_event, popup.borrow_mut().text_editor_mut());
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
                        KC::Char('c') => {
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
        VenomFocus::EditLabelsPopup(popup) => match key_event.code {
            KC::Esc => {
                if popup.borrow().text_editor().mode != EditorMode::Normal {
                    if popup.borrow().text_editor().mode != EditorMode::Normal {
                        popup.borrow_mut().text_editor_mut().mode = EditorMode::Normal;
                    } else {
                        let mut line = String::new();
                        let mut previous_row = 0;
                        let mut tags = vec![];
                        popup.borrow().text_editor().lines.iter().for_each(|c| {
                            let (c, idx) = c;

                            if idx.row > previous_row {
                                let words = line.split_whitespace().collect::<Vec<_>>();
                                if words.len() > 4 {
                                    let short_name = words[0];
                                    tags.push(short_name.to_string());
                                    let red = words[1].parse::<u8>();
                                    let green = words[2].parse::<u8>();
                                    let blue = words[3].parse::<u8>();
                                    let long_name = words[4..]
                                        .iter()
                                        .flat_map(|word| word.chars())
                                        .collect::<String>();
                                    if let (Ok(r), Ok(g), Ok(b)) = (red, green, blue) {
                                        match app.task_db().label_by_tag(short_name) {
                                            None => {
                                                let mut label = TaskLabel::default();
                                                label
                                                    .set_rgb(r, g, b)
                                                    .set_tag(short_name)
                                                    .set_name(&long_name);
                                                app.task_db_mut()
                                                    .add_label(Rc::new(RefCell::new(label)));
                                            }
                                            Some(label) => {
                                                label
                                                    .borrow_mut()
                                                    .set_rgb(r, g, b)
                                                    .set_tag(short_name)
                                                    .set_name(&long_name);
                                            }
                                        }
                                    }
                                }

                                // cleanup
                                previous_row = idx.row;
                                line.clear();
                            }
                            if let Some(c) = c {
                                line.push(*c);
                            }
                        });
                        let words = line.split_whitespace().collect::<Vec<_>>();
                        if words.len() > 4 {
                            let short_name = words[0];
                            tags.push(short_name.to_string());
                            let red = words[1].parse::<u8>();
                            let green = words[2].parse::<u8>();
                            let blue = words[3].parse::<u8>();
                            let long_name = words[4..]
                                .iter()
                                .flat_map(|word| word.chars())
                                .collect::<String>();
                            if let (Ok(r), Ok(g), Ok(b)) = (red, green, blue) {
                                match app.task_db().label_by_tag(short_name) {
                                    None => {
                                        let mut label = TaskLabel::default();
                                        label
                                            .set_rgb(r, g, b)
                                            .set_tag(short_name)
                                            .set_name(&long_name);
                                        app.task_db_mut().add_label(Rc::new(RefCell::new(label)));
                                    }
                                    Some(label) => {
                                        label
                                            .borrow_mut()
                                            .set_rgb(r, g, b)
                                            .set_tag(short_name)
                                            .set_name(&long_name);
                                    }
                                }
                            }
                        }

                        let mut to_remove = Vec::new();
                        app.task_db_mut().labels_mut_vec().iter().for_each(|label| {
                            let label_string = label.borrow().short_name_string();
                            if !tags.contains(&label_string) {
                                to_remove.push(label_string);
                            }
                        });
                        for s in to_remove {
                            app.task_db_mut().remove_label(&s);
                        }

                        app.save_file();
                        app.set_mode(VenomFocus::MainView);
                    }
                } else {
                    app.set_mode(VenomFocus::MainView);
                }
            }
            KC::Char('c') => {
                if key_event.modifiers == KM::CONTROL {
                    if popup.borrow().text_editor().mode != EditorMode::Normal {
                        popup.borrow_mut().text_editor_mut().mode = EditorMode::Normal;
                    } else {
                        let mut line = String::new();
                        let mut previous_row = 0;
                        let mut tags = vec![];
                        popup.borrow().text_editor().lines.iter().for_each(|c| {
                            let (c, idx) = c;

                            if idx.row > previous_row {
                                let words = line.split_whitespace().collect::<Vec<_>>();
                                if words.len() > 4 {
                                    let short_name = words[0];
                                    tags.push(short_name.to_string());
                                    let red = words[1].parse::<u8>();
                                    let green = words[2].parse::<u8>();
                                    let blue = words[3].parse::<u8>();
                                    let long_name = words[4..]
                                        .iter()
                                        .flat_map(|word| word.chars())
                                        .collect::<String>();
                                    if let (Ok(r), Ok(g), Ok(b)) = (red, green, blue) {
                                        match app.task_db().label_by_tag(short_name) {
                                            None => {
                                                let mut label = TaskLabel::default();
                                                label
                                                    .set_rgb(r, g, b)
                                                    .set_tag(short_name)
                                                    .set_name(&long_name);
                                                app.task_db_mut()
                                                    .add_label(Rc::new(RefCell::new(label)));
                                            }
                                            Some(label) => {
                                                label
                                                    .borrow_mut()
                                                    .set_rgb(r, g, b)
                                                    .set_tag(short_name)
                                                    .set_name(&long_name);
                                            }
                                        }
                                    }
                                }

                                // cleanup
                                previous_row = idx.row;
                                line.clear();
                            }
                            if let Some(c) = c {
                                line.push(*c);
                            }
                        });
                        let words = line.split_whitespace().collect::<Vec<_>>();
                        if words.len() > 4 {
                            let short_name = words[0];
                            tags.push(short_name.to_string());
                            let red = words[1].parse::<u8>();
                            let green = words[2].parse::<u8>();
                            let blue = words[3].parse::<u8>();
                            let long_name = words[4..]
                                .iter()
                                .flat_map(|word| word.chars())
                                .collect::<String>();
                            if let (Ok(r), Ok(g), Ok(b)) = (red, green, blue) {
                                match app.task_db().label_by_tag(short_name) {
                                    None => {
                                        let mut label = TaskLabel::default();
                                        label
                                            .set_rgb(r, g, b)
                                            .set_tag(short_name)
                                            .set_name(&long_name);
                                        app.task_db_mut().add_label(Rc::new(RefCell::new(label)));
                                    }
                                    Some(label) => {
                                        label
                                            .borrow_mut()
                                            .set_rgb(r, g, b)
                                            .set_tag(short_name)
                                            .set_name(&long_name);
                                    }
                                }
                            }
                        }

                        let mut to_remove = Vec::new();
                        app.task_db_mut().labels_mut_vec().iter().for_each(|label| {
                            let label_string = label.borrow().short_name_string();
                            if !tags.contains(&label_string) {
                                to_remove.push(label_string);
                            }
                        });
                        for s in to_remove {
                            app.task_db_mut().remove_label(&s);
                        }

                        app.save_file();
                        app.set_mode(VenomFocus::MainView);
                    }
                } else {
                    let mut input = Input::default();
                    input.on_key(key_event, popup.borrow_mut().text_editor_mut());
                }
            }
            _ => {
                let mut input = Input::default();
                input.on_key(key_event, popup.borrow_mut().text_editor_mut());
            }
        },
    }
}

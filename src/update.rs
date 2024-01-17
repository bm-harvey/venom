use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::EditorMode;
use edtui::Input;
use std::cell::RefCell;
use std::rc::Rc;
//use serde_json::json;

use crate::app::Venom;
use crate::app::VenomFocus;
use crate::edit_labels_popup::EditLabelsPopup;
use crate::edit_task_popup::EditTaskFocus;
use crate::edit_task_popup::EditTaskPopup;
use crate::task::TaskLabel;

type KC = KeyCode;
type KM = KeyModifiers;

pub fn update(app: &mut Venom, ke: KeyEvent) {
    let focus = app.focus().clone();
    match focus {
        VenomFocus::MainView => {
            match (ke.code, ke.modifiers) {
                (KC::Esc, _) | (KC::Char('c'), KM::CONTROL) => app.quit(),
                (KC::Down | KC::Char('j'), _) => app.increment_task_idx(),
                (KC::Up | KC::Char('k'), _) => app.decrement_task_idx(),
                (KC::Char('a'), _) => app.add_task(),
                (KC::Char('e') | KC::Enter, _) => {
                    if app.selected_task_idx() < app.task_db().len() {
                        app.edit_task();
                    }
                }
                (KC::Char('l'), _) => app.edit_labels(),
                (KC::Char('d'), _) => {
                    app.remove_selected_task();
                    app.save_file();
                }
                (KC::Char('r'), _) => app.add_task_based_on_current(),
                //(KC::Char('h'), _) => app.toggle_hide_completed(),
                (KC::Char(' '), _) => {
                    app.toggle_selected_task();
                    app.task_db_mut().sort_by_date();
                    app.save_file();
                }
                _ => {}
            };
        }
        VenomFocus::EditTaskPopup(popup) => {
            let focus = popup.borrow().focus();
            match focus {
                EditTaskFocus::Edit => match (ke.code, ke.modifiers) {
                    (KC::Esc, _) | (KC::Char('c'), KM::CONTROL) => {
                        if popup.borrow().text_editor().mode == edtui::EditorMode::Normal {
                            escape_task_edit(app, &popup)
                        } else {
                            popup.borrow_mut().text_editor_mut().mode = edtui::EditorMode::Normal
                        }
                    }
                    (_, _) => {
                        let mut input = Input::default();
                        input.on_key(ke, popup.borrow_mut().text_editor_mut());
                    }
                },
                EditTaskFocus::Fields => {
                    match (ke.code, ke.modifiers) {
                        (KC::Esc, _) | (KC::Char('c'), KM::CONTROL) => {
                            app.task_db_mut().sort_by_date();
                            app.set_mode(VenomFocus::MainView);
                        }
                        (KC::Down | KC::Char('j'), _) => {
                            popup.borrow_mut().increment_property();
                            let property = popup.borrow().property();
                            let text = app.selected_task().borrow().text_to_edit(property);
                            popup.borrow_mut().load_text(&text);
                        }
                        (KC::Up | KC::Char('k'), _) => {
                            popup.borrow_mut().decrement_property();
                            let text = app
                                .selected_task()
                                .borrow()
                                .text_to_edit(popup.borrow().property());
                            popup.borrow_mut().load_text(&text);
                        }
                        (KC::Enter, _) => {
                            popup.borrow_mut().set_focus(EditTaskFocus::Edit);
                        }
                        _ => {}
                    };
                }
            }
        }
        VenomFocus::EditLabelsPopup(popup) => match (ke.code, ke.modifiers) {
            (KC::Esc, _) | (KC::Char('c'), KM::CONTROL) => {
                if popup.borrow().text_editor().mode != EditorMode::Normal {
                    popup.borrow_mut().text_editor_mut().mode = EditorMode::Normal;
                } else {
                    escape_label_popup(app, &popup);
                }
            }
            (_, _) => {
                let mut input = Input::default();
                input.on_key(ke, popup.borrow_mut().text_editor_mut());
            }
        },
    }
}

fn escape_label_popup(app: &mut Venom, popup: &RefCell<EditLabelsPopup>) {
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
                if words.len() > 2 {
                    let short_name = words[0];
                    tags.push(short_name.to_string());
                    let color = words[1];
                    let long_name = words[2..]
                        .iter()
                        .flat_map(|word| {
                            let mut chars = word.chars().collect::<Vec<_>>();
                            chars.push(' ');
                            chars.into_iter()
                        })
                        .collect::<String>();
                    match app.task_db().label_by_tag(short_name) {
                        None => {
                            let mut label = TaskLabel::default();
                            label
                                .set_color(color)
                                .set_tag(short_name)
                                .set_name(long_name.trim_end());
                            app.task_db_mut().add_label(Rc::new(RefCell::new(label)));
                        }
                        Some(label) => {
                            label
                                .borrow_mut()
                                .set_color(color)
                                .set_tag(short_name)
                                .set_name(long_name.trim_end());
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
        if words.len() > 2 {
            let short_name = words[0];
            tags.push(short_name.to_string());
            let color = words[1];
            let long_name = words[2..]
                .iter()
                .flat_map(|word| {
                    let mut chars = word.chars().collect::<Vec<_>>();
                    chars.push(' ');
                    chars.into_iter()
                })
                .collect::<String>();
            match app.task_db().label_by_tag(short_name) {
                None => {
                    let mut label = TaskLabel::default();
                    label
                        .set_color(color)
                        .set_tag(short_name)
                        .set_name(long_name.trim_end());
                    app.task_db_mut().add_label(Rc::new(RefCell::new(label)));
                }
                Some(label) => {
                    label
                        .borrow_mut()
                        .set_color(color)
                        .set_tag(short_name)
                        .set_name(&long_name);
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

        for task in app.task_db().tasks().iter() {
            let mut task = task.borrow_mut();
            match task.label() {
                None => {}
                Some(label) => {
                    let tag = label.borrow().short_name().iter().collect::<String>();
                    task.set_label(app.task_db().label_by_tag(&tag));
                }
            }
        }
        app.save_file();
        app.set_mode(VenomFocus::MainView);
    }
}

fn escape_task_edit(app: &mut Venom, popup: &RefCell<EditTaskPopup>) {
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
            task.borrow_mut()
                .set_property_from_str(popup.borrow().property(), &text);
        }
    }
    app.save_file();
}

// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(not(target_os = "macos"))]
use msg::constellation_msg::CONTROL;
#[cfg(target_os = "macos")]
use msg::constellation_msg::SUPER;
use msg::constellation_msg::{Key, KeyModifiers};
use script::clipboard_provider::DummyClipboardContext;
use script::textinput::{TextInput, Selection, Lines, Direction};
use util::str::DOMString;

fn text_input(lines: Lines, s: &str) -> TextInput<DummyClipboardContext> {
    TextInput::new(lines, DOMString::from(s), DummyClipboardContext::new(""))
}

#[test]
fn test_textinput_delete_char() {
    let mut textinput = text_input(Lines::Single, "abcdefg");
    textinput.adjust_horizontal(2, Selection::NotSelected);
    textinput.delete_char(Direction::Backward);
    assert_eq!(textinput.get_content(), "acdefg");

    textinput.delete_char(Direction::Forward);
    assert_eq!(textinput.get_content(), "adefg");

    textinput.adjust_horizontal(2, Selection::Selected);
    textinput.delete_char(Direction::Forward);
    assert_eq!(textinput.get_content(), "afg");

    let mut textinput = text_input(Lines::Single, "a🌠b");
    // Same as "Right" key
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    textinput.delete_char(Direction::Forward);
    // Not splitting surrogate pairs.
    assert_eq!(textinput.get_content(), "ab");
}

#[test]
fn test_textinput_insert_char() {
    let mut textinput = text_input(Lines::Single, "abcdefg");
    textinput.adjust_horizontal(2, Selection::NotSelected);
    textinput.insert_char('a');
    assert_eq!(textinput.get_content(), "abacdefg");

    textinput.adjust_horizontal(2, Selection::Selected);
    textinput.insert_char('b');
    assert_eq!(textinput.get_content(), "ababefg");

    let mut textinput = text_input(Lines::Single, "a🌠c");
    // Same as "Right" key
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    textinput.insert_char('b');
    // Not splitting surrogate pairs.
    assert_eq!(textinput.get_content(), "a🌠bc");
}

#[test]
fn test_textinput_get_sorted_selection() {
    let mut textinput = text_input(Lines::Single, "abcdefg");
    textinput.adjust_horizontal(2, Selection::NotSelected);
    textinput.adjust_horizontal(2, Selection::Selected);
    let (begin, end) = textinput.get_sorted_selection().unwrap();
    assert_eq!(begin.index, 2);
    assert_eq!(end.index, 4);

    textinput.clear_selection();

    textinput.adjust_horizontal(-2, Selection::Selected);
    let (begin, end) = textinput.get_sorted_selection().unwrap();
    assert_eq!(begin.index, 2);
    assert_eq!(end.index, 4);
}

#[test]
fn test_textinput_replace_selection() {
    let mut textinput = text_input(Lines::Single, "abcdefg");
    textinput.adjust_horizontal(2, Selection::NotSelected);
    textinput.adjust_horizontal(2, Selection::Selected);

    textinput.replace_selection(DOMString::from("xyz"));
    assert_eq!(textinput.get_content(), "abxyzefg");
}

#[test]
fn test_textinput_current_line_length() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    assert_eq!(textinput.current_line_length(), 3);

    textinput.adjust_vertical(1, Selection::NotSelected);
    assert_eq!(textinput.current_line_length(), 2);

    textinput.adjust_vertical(1, Selection::NotSelected);
    assert_eq!(textinput.current_line_length(), 1);
}

#[test]
fn test_textinput_adjust_vertical() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    textinput.adjust_horizontal(3, Selection::NotSelected);
    textinput.adjust_vertical(1, Selection::NotSelected);
    assert_eq!(textinput.edit_point.line, 1);
    assert_eq!(textinput.edit_point.index, 2);

    textinput.adjust_vertical(-1, Selection::NotSelected);
    assert_eq!(textinput.edit_point.line, 0);
    assert_eq!(textinput.edit_point.index, 2);

    textinput.adjust_vertical(2, Selection::NotSelected);
    assert_eq!(textinput.edit_point.line, 2);
    assert_eq!(textinput.edit_point.index, 1);
}

#[test]
fn test_textinput_adjust_horizontal() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    textinput.adjust_horizontal(4, Selection::NotSelected);
    assert_eq!(textinput.edit_point.line, 1);
    assert_eq!(textinput.edit_point.index, 0);

    textinput.adjust_horizontal(1, Selection::NotSelected);
    assert_eq!(textinput.edit_point.line, 1);
    assert_eq!(textinput.edit_point.index, 1);

    textinput.adjust_horizontal(2, Selection::NotSelected);
    assert_eq!(textinput.edit_point.line, 2);
    assert_eq!(textinput.edit_point.index, 0);

    textinput.adjust_horizontal(-1, Selection::NotSelected);
    assert_eq!(textinput.edit_point.line, 1);
    assert_eq!(textinput.edit_point.index, 2);
}

#[test]
fn test_textinput_handle_return() {
    let mut single_line_textinput = text_input(Lines::Single, "abcdef");
    single_line_textinput.adjust_horizontal(3, Selection::NotSelected);
    single_line_textinput.handle_return();
    assert_eq!(single_line_textinput.get_content(), "abcdef");

    let mut multi_line_textinput = text_input(Lines::Multiple, "abcdef");
    multi_line_textinput.adjust_horizontal(3, Selection::NotSelected);
    multi_line_textinput.handle_return();
    assert_eq!(multi_line_textinput.get_content(), "abc\ndef");
}

#[test]
fn test_textinput_select_all() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    assert_eq!(textinput.edit_point.line, 0);
    assert_eq!(textinput.edit_point.index, 0);

    textinput.select_all();
    assert_eq!(textinput.edit_point.line, 2);
    assert_eq!(textinput.edit_point.index, 1);
}

#[test]
fn test_textinput_get_content() {
    let single_line_textinput = text_input(Lines::Single, "abcdefg");
    assert_eq!(single_line_textinput.get_content(), "abcdefg");

    let multi_line_textinput = text_input(Lines::Multiple, "abc\nde\nf");
    assert_eq!(multi_line_textinput.get_content(), "abc\nde\nf");
}

#[test]
fn test_textinput_set_content() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    assert_eq!(textinput.get_content(), "abc\nde\nf");

    textinput.set_content(DOMString::from("abc\nf"));
    assert_eq!(textinput.get_content(), "abc\nf");

    assert_eq!(textinput.edit_point.line, 0);
    assert_eq!(textinput.edit_point.index, 0);
    textinput.adjust_horizontal(3, Selection::Selected);
    assert_eq!(textinput.edit_point.line, 0);
    assert_eq!(textinput.edit_point.index, 3);
    textinput.set_content(DOMString::from("de"));
    assert_eq!(textinput.get_content(), "de");
    assert_eq!(textinput.edit_point.line, 0);
    assert_eq!(textinput.edit_point.index, 2);
}

#[test]
fn test_clipboard_paste() {
    #[cfg(target_os = "macos")]
    const MODIFIERS: KeyModifiers = SUPER;
    #[cfg(not(target_os = "macos"))]
    const MODIFIERS: KeyModifiers = CONTROL;

    let mut textinput = TextInput::new(Lines::Single,
                                       DOMString::from("defg"),
                                       DummyClipboardContext::new("abc"));
    assert_eq!(textinput.get_content(), "defg");
    assert_eq!(textinput.edit_point.index, 0);
    textinput.handle_keydown_aux(Key::V, MODIFIERS);
    assert_eq!(textinput.get_content(), "abcdefg");
}

#[test]
fn test_textinput_cursor_position_correct_after_clearing_selection() {
    let mut textinput = text_input(Lines::Single, "abcdef");

    // Single line - Forward
    textinput.adjust_horizontal(3, Selection::Selected);
    textinput.adjust_horizontal(1, Selection::NotSelected);
    assert_eq!(textinput.edit_point.index, 3);

    textinput.adjust_horizontal(-3, Selection::NotSelected);
    textinput.adjust_horizontal(3, Selection::Selected);
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point.index, 3);

    // Single line - Backward
    textinput.adjust_horizontal(-3, Selection::NotSelected);
    textinput.adjust_horizontal(3, Selection::Selected);
    textinput.adjust_horizontal(-1, Selection::NotSelected);
    assert_eq!(textinput.edit_point.index, 0);

    textinput.adjust_horizontal(-3, Selection::NotSelected);
    textinput.adjust_horizontal(3, Selection::Selected);
    textinput.adjust_horizontal_by_one(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput.edit_point.index, 0);


    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");

    // Multiline - Forward
    textinput.adjust_horizontal(4, Selection::Selected);
    textinput.adjust_horizontal(1, Selection::NotSelected);
    assert_eq!(textinput.edit_point.index, 0);
    assert_eq!(textinput.edit_point.line, 1);

    textinput.adjust_horizontal(-4, Selection::NotSelected);
    textinput.adjust_horizontal(4, Selection::Selected);
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point.index, 0);
    assert_eq!(textinput.edit_point.line, 1);

    // Multiline - Backward
    textinput.adjust_horizontal(-4, Selection::NotSelected);
    textinput.adjust_horizontal(4, Selection::Selected);
    textinput.adjust_horizontal(-1, Selection::NotSelected);
    assert_eq!(textinput.edit_point.index, 0);
    assert_eq!(textinput.edit_point.line, 0);

    textinput.adjust_horizontal(-4, Selection::NotSelected);
    textinput.adjust_horizontal(4, Selection::Selected);
    textinput.adjust_horizontal_by_one(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput.edit_point.index, 0);
    assert_eq!(textinput.edit_point.line, 0);
}

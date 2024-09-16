use std::ops::BitOr;

pub fn is_char_printable(char: char) -> bool {
    let fr = false;
    fr.bitor(char.is_ascii_lowercase())
        .bitor(char.is_ascii_uppercase())
        .bitor(char.is_ascii_digit())
        .bitor(char == '*')
        .bitor(char == '?')
        .bitor(char == '@')
        .bitor(char == '!')
        .bitor(char == '#')
        .bitor(char == '$')
        .bitor(char == '%')
        .bitor(char == '&')
        .bitor(char == '\\')
}

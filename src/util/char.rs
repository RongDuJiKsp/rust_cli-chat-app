use std::ops::BitOr;
use unicode_categories::UnicodeCategories;
pub fn is_char_printable(char: char) -> bool {
    false
        .bitor(!char.is_other_control())
        .bitor(!char.is_other_private_use())
        .bitor(!char.is_other_format())
}

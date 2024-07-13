use crate::character::Character; 

pub fn linear_pos_from_x_y(x: u16, y: u16, width: u16) -> u16 {
    let adjusted_y = y * width;
    return x + adjusted_y;
}

pub fn char_vec_from_string(string: String) -> Vec<Character> {
    let mut result = vec![];
    for ch in string.chars() {
        result.push(Character::from_char(ch));
    };
    return result;
}

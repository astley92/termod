use crossterm::style;
use rand::Rng;

use crate::colours;

#[derive(Clone, PartialEq)]
pub struct Character {
    pub c: char,
    pub attributes: style::Attributes,
    pub colour: style::Color,
}

pub fn empty_attr_set() -> style::Attributes {
    let mut attrs = style::Attributes::from(style::Attribute::Bold); 
    attrs.unset(style::Attribute::Bold);
    return attrs;
}

impl Character {
    const ATTRIBUTES: [style::Attribute; 4] = [
        style::Attribute::Bold,
        style::Attribute::Dim,
        style::Attribute::Underlined,
        style::Attribute::Reverse,
    ];

    pub fn blank() -> Character {
        return Character {
            c: ' ',
            colour: colours::GREY,
            attributes: empty_attr_set(),
        }
    }
    
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Character {
        let rand_char_code: u8 = rand::thread_rng().gen_range(32..=126);
        if rand_char_code == 32 {
            return Character::blank();
        };
        let rand_char = rand_char_code as char;
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        let colour = style::Color::Rgb {r: r, g: g, b: b};
        let attr_index = rng.gen_range(0..Self::ATTRIBUTES.len());
        let mut attrs = empty_attr_set();
        let attr = Self::ATTRIBUTES[attr_index];
        attrs.toggle(attr);
        return Character {
            c: rand_char,
            attributes: attrs,
            colour: colour,
        };
    }

    pub fn vec_from_string(string: &String) -> Vec<Character> {
        let mut result: Vec<Character> = vec![];
        let string_chars: Vec<char> = string.chars().collect();
        for i in 0..string_chars.len() {
            result.push(Character {
                c: string_chars[i],
                attributes: empty_attr_set(),
                colour: colours::GREY,
            })
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name;
    use super::*;

    fn type_of<T>(_: &T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn vec_from_string() {
        let starting_string = "Hello World".to_string();
        let starting_string_chars: Vec<char> = starting_string.chars().collect();
        let result = Character::vec_from_string(&starting_string);
        for i in 0..starting_string.len() {
            let character = &result[i];
            assert_eq!(type_of(character), "termod::character::Character");
            assert_eq!(character.c, starting_string_chars[i]);
        }
    }
}

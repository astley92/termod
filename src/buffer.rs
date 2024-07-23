use crate::character::Character;

#[derive(Debug)]
pub struct BufferMergeError;

#[derive(Clone)]
pub struct Buffer {
    pub width: u16,
    pub height: u16, 
    characters: Vec<Character>,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Buffer {
        let mut chars = vec![];
        for _ in 0..width {
            for _ in 0..height {
                chars.push(Character::blank());
            }
        }
        return Buffer {
            width: width,
            height: height,
            characters: chars,
        }
    }

    pub fn len(&self) -> usize {
        return self.characters.len();
    }

    pub fn merge(&self, position_to_insert: usize, other: &Buffer) -> Result<Buffer, BufferMergeError> {
        let x_offset = position_to_insert % self.width as usize;
        let y_offset = position_to_insert / self.width as usize;

        if x_offset + other.width as usize > self.width as usize {
            println!("Too wide!!");
            return Err(BufferMergeError);
        } else if y_offset + other.height as usize > self.height as usize {
            println!("Too high!!");
            return Err(BufferMergeError);
        }

        let mut new_buff = self.clone();
        for other_x in 0..other.width {
            for other_y in 0..other.height {
                let this_x = other_x + x_offset as u16;
                let this_y = other_y + y_offset as u16;
                let other_pos = other_y * other.width + other_x;
                let this_pos = this_y * self.width + this_x;
                new_buff[this_pos as usize] = other[other_pos as usize].clone();
            }
        }

        return Ok(new_buff);
    }

    pub fn insert_char_slice(&mut self, position: usize, chars: &Vec<Character>) {
        for i in 0..chars.len() {
            let offset = i + position;
            if self.characters[offset] != chars[i] {
                self.characters[offset] = chars[i].clone()
            }
        }
    }
}

impl std::ops::Index<usize> for Buffer {
    type Output = Character;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.characters[i];
    }
}

impl std::ops::IndexMut<usize> for Buffer {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.characters[i];
    }
}

#[cfg(test)]
mod insert_char_slice_tests {
    use super::*;
    use crate::character::Character;

    #[test]
    fn inserts_the_chars_at_the_expected_places() {
        let mut buffer_one = Buffer::new(6, 3);
        let string_chars = Character::vec_from_string(&"----".to_string(), None, None);

        buffer_one.insert_char_slice(7, &string_chars);
        let expected_res = [
            ' ', ' ', ' ', ' ', ' ', ' ', 
            ' ', '-', '-', '-', '-', ' ', 
            ' ', ' ', ' ', ' ', ' ', ' ',
        ];
        let result: Vec<char> = buffer_one.characters.into_iter().map(|x| x.c).collect();
        for i in 0..result.len() {
            assert_eq!(result[i], expected_res[i], "Incorrect at position {}", i);
        }
    }
}

#[cfg(test)]
mod merge_tests {
    use std::any::type_name;

    use crate::{character, colours};

    use super::*;
    use crossterm::style;

    fn type_of<T>(_: &T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn buffer_merge_returns_buffer() {
        let buffer_one = Buffer::new(6, 3);
        let mut buffer_two= Buffer::new(4, 1);
        for i in 0..buffer_two.len() {
            buffer_two[i] = Character {
                c: '-',
                colour: colours::GREY, 
                attributes: character::empty_attr_set(),
            }
        };

        let response = buffer_one.merge(0, &buffer_two).unwrap();
        assert_eq!(type_of(&response), "termod::buffer::Buffer");
    }

    #[test]
    fn buffer_merge_returns_expected_buffer() {
        let buffer_one = Buffer::new(6, 3);
        let mut buffer_two= Buffer::new(4, 1);
        for i in 0..buffer_two.len() {
            buffer_two[i] = Character {
                c: '-',
                colour: colours::GREY,
                attributes: character::empty_attr_set(),
            }
        };

        let response = buffer_one.merge(7, &buffer_two).unwrap();
        let expected_res = [
            ' ', ' ', ' ', ' ', ' ', ' ', 
            ' ', '-', '-', '-', '-', ' ', 
            ' ', ' ', ' ', ' ', ' ', ' ',
        ];
        let result: Vec<char> = response.characters.into_iter().map(|x| x.c).collect();
        for i in 0..result.len() {
            assert_eq!(result[i], expected_res[i], "Incorrect at position {}", i);
        }
    }

    #[test]
    fn buffer_merge_doesnt_mutate_original_buffers() {
        let buffer_one = Buffer::new(6, 3);
        let mut buffer_two= Buffer::new(4, 1);
        let attrs = character::empty_attr_set();
        for i in 0..buffer_two.len() {
            buffer_two[i] = Character {
                c: '-',
                colour: colours::GREY,
                attributes: attrs,
            }
        };

        let _ = buffer_one.merge(7, &buffer_two).unwrap();
        let expected_res = [
            ' ', ' ', ' ', ' ', ' ', ' ', 
            ' ', ' ', ' ', ' ', ' ', ' ', 
            ' ', ' ', ' ', ' ', ' ', ' ',
        ];
        let result: Vec<char> = buffer_one.characters.into_iter().map(|x| x.c).collect();
        for i in 0..result.len() {
            assert_eq!(result[i], expected_res[i], "Incorrect at position {}", i);
        }

        let expected_res = ['-', '-', '-', '-'];
        let result: Vec<char> = buffer_two.characters.into_iter().map(|x| x.c).collect();
        for i in 0..result.len() {
            assert_eq!(result[i], expected_res[i], "Incorrect at position {}", i);
        }
    }
}

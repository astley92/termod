use crate::character::Character;

#[derive(Debug)]
pub struct BufferMergeError;

#[derive(Clone)]
pub struct Buffer {
    width: u16,
    height: u16, 
    pub x: u16,
    pub y: u16,
    characters: Vec<Character>,
}

impl Buffer {
    pub fn new(width: u16, height: u16, x: u16, y: u16) -> Buffer {
        let mut chars = vec![];
        for _ in 0..width {
            for _ in 0..height {
                chars.push(Character::blank());
            }
        }
        return Buffer {
            width: width,
            height: height,
            x: x, 
            y: y,
            characters: chars,
        }
    }

    pub fn len(&self) -> usize {
        return self.characters.len();
    }

    pub fn push(&mut self, other: &Buffer) -> bool {
        if other.x + other.width > self.width {
            return false;
        } else if other.y + other.height > self.height {
            return false;
        }

        for other_x in 0..other.width {
            for other_y in 0..other.height {
                let this_x = other_x + other.x;
                let this_y = other_y + other.y;
                let other_pos = other_y * other.width + other_x;
                let this_pos = this_y * self.width + this_x;
                self[this_pos as usize] = other[other_pos as usize].clone();
            }
        }

        return true;
    }

    pub fn merge(&self, other: &Buffer) -> Result<Buffer, BufferMergeError> {
        if other.x + other.width > self.width {
            println!("Too wide!!");
            return Err(BufferMergeError);
        } else if other.y + other.height > self.height {
            println!("Too high!!");
            return Err(BufferMergeError);
        }

        let mut new_buff = self.clone();
        for other_x in 0..other.width {
            for other_y in 0..other.height {
                let this_x = other_x + other.x;
                let this_y = other_y + other.y;
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
            self.characters[offset] = chars[i].clone()
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
    fn returns_true_on_success() {
        let mut buffer_one = Buffer::new(6, 3, 0, 0);
        let string_chars = Character::vec_from_string(&"----".to_string());

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

    use super::*;
    use crossterm::style;

    fn type_of<T>(_: &T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn buffer_merge_returns_buffer() {
        let buffer_one = Buffer::new(6, 3, 0, 0);
        let mut buffer_two= Buffer::new(4, 1, 1, 1);
        let attrs = style::Attributes::from(style::Attribute::Bold);
        for i in 0..buffer_two.len() {
            buffer_two[i] = Character {
                c: '-',
                colour: style::Color::Rgb { r: 0, g: 0, b: 0 },
                attributes: attrs,
            }
        };

        let response = buffer_one.merge(&buffer_two).unwrap();
        assert_eq!(type_of(&response), "termod::buffer::Buffer");
    }

    #[test]
    fn buffer_merge_returns_expected_buffer() {
        let buffer_one = Buffer::new(6, 3, 0, 0);
        let mut buffer_two= Buffer::new(4, 1, 1, 1);
        let attrs = style::Attributes::from(style::Attribute::Bold);
        for i in 0..buffer_two.len() {
            buffer_two[i] = Character {
                c: '-',
                colour: style::Color::Rgb { r: 0, g: 0, b: 0 },
                attributes: attrs,
            }
        };

        let response = buffer_one.merge(&buffer_two).unwrap();
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
        let buffer_one = Buffer::new(6, 3, 0, 0);
        let mut buffer_two= Buffer::new(4, 1, 1, 1);
        let attrs = style::Attributes::from(style::Attribute::Bold);
        for i in 0..buffer_two.len() {
            buffer_two[i] = Character {
                c: '-',
                colour: style::Color::Rgb { r: 0, g: 0, b: 0 },
                attributes: attrs,
            }
        };

        let _ = buffer_one.merge(&buffer_two).unwrap();
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

#[cfg(test)]
mod buffer_push_tests {
    use super::*;
    use crossterm::style;

    #[test]
    fn buffer_push_success() {
        let mut buffer_one = Buffer::new(6, 3, 0, 0);
        let mut buffer_two= Buffer::new(4, 1, 1, 1);
        let attrs = style::Attributes::from(style::Attribute::Bold);
        for i in 0..buffer_two.len() {
            buffer_two[i] = Character {
                c: '-',
                colour: style::Color::Rgb { r: 0, g: 0, b: 0 },
                attributes: attrs,
            }
        };
        let response = buffer_one.push(&buffer_two);
        assert!(response);
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
    
    #[test]
    fn buffer_push_too_wide() {
        let mut buffer_one = Buffer::new(6, 3, 0, 0);
        let buffer_two= Buffer::new(7,1, 0, 0);
        let response = buffer_one.push(&buffer_two);
        assert!(!response);
    }
    
    #[test]
    fn buffer_push_too_tall() {
        let mut buffer_one = Buffer::new(6, 3, 0, 0);
        let buffer_two= Buffer::new(1,4, 0, 0);
        let response = buffer_one.push(&buffer_two);
        assert!(!response);
    }

    #[test]
    fn buffer_push_too_far_right() {
        let mut buffer_one = Buffer::new(6, 3, 0, 0);
        let buffer_two= Buffer::new(4,1, 5, 0);
        let response = buffer_one.push(&buffer_two);
        assert!(!response);
    }

    #[test]
    fn buffer_push_too_far_down() {
        let mut buffer_one = Buffer::new(6, 3, 0, 0);
        let buffer_two= Buffer::new(4,3, 0, 2);
        let response = buffer_one.push(&buffer_two);
        assert!(!response);
    }
}

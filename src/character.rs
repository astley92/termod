use crossterm::style;

#[derive(Clone, Debug)]
pub struct Character {
    pub character: char,
    pub attributes: Vec<style::Attribute>,
}

impl Character {
    pub fn from_char(ch: char) -> Character {
        let character = Character {
            character: ch,
            attributes: vec![],
        };

        return character;
    }
}

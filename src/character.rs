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

    pub fn toggle_reverse(&mut self) {
        if self.attributes.contains(&style::Attribute::Reverse) {
            self.attributes.retain(|x| *x != style::Attribute::Reverse)
        } else {
            self.attributes.push(style::Attribute::Reverse)
        }
    }
}

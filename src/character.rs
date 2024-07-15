use crossterm::style;

#[derive(Clone, Debug)]
pub struct Character {
    pub character: char,
    pub attributes: Vec<style::Attribute>,
    pub fg_colour: Option<style::Color>,
}

impl Character {
    pub fn from_char(ch: char) -> Character {
        let character = Character {
            character: ch,
            attributes: vec![],
            fg_colour: None,
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

    pub fn set_fg_colour(&mut self, colour: style::Color) {
        self.fg_colour = Some(colour);
    }
}

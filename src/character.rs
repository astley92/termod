use crossterm::style;
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Character {
    pub c: char,
    pub attributes: style::Attributes,
    pub colour: style::Color,
}

impl Character {
    const ATTRIBUTES: [style::Attribute; 4] = [
        style::Attribute::Bold,
        style::Attribute::Dim,
        style::Attribute::Underlined,
        style::Attribute::Reverse,
    ];

    pub fn blank() -> Character {
        let mut attrs = style::Attributes::from(style::Attribute::Bold);
        attrs.unset(style::Attribute::Bold);
        return Character {
            c: ' ',
            colour: style::Color::Rgb { r: 0, g: 0, b: 0 },
            attributes: attrs,
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
        let mut attrs = style::Attributes::from(style::Attribute::Bold);
        attrs.unset(style::Attribute::Bold);
        let attr = Self::ATTRIBUTES[attr_index];
        attrs.toggle(attr);
        return Character {
            c: rand_char,
            attributes: attrs,
            colour: colour,
        };
    }
}

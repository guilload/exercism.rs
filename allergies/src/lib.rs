use Allergen::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Allergen {
    Eggs         = 0,
    Peanuts      = 1,
    Shellfish    = 2,
    Strawberries = 3,
    Tomatoes     = 4,
    Chocolate    = 5,
    Pollen       = 6,
    Cats         = 7,
}

const ALLERGENS: [Allergen; 8] = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

pub struct Allergies {
    score: u8
}

impl Allergies {

    pub fn new(score: u8) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & 1 << *allergen as u8 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .cloned()
            .collect()
    }

}



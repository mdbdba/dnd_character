
#[derive(Debug)]
pub struct Ability {
    description: String,
    score: i8, 
}

#[derive(Debug)]
pub struct Abilities {
    strength: Ability,
    dexterity: Ability,
    constitution: Ability,
    intelligence: Ability,
    wisdom: Ability,
    charisma: Ability,
}
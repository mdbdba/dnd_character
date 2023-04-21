use std::collections::HashMap;
use crate::ancestry::CharacterAbility;

pub enum Vantage {
    Advantage,
    Disadvantage
}
pub fn get_vantage(src: Vantage) -> String {
    match src {
        Advantage=> "advantage".to_string(),
        Disadvantage=> "disadvantage".to_string(),
    }
}

pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder
}

pub fn get_damage_type(src: DamageType) -> String {
    match src {
        Acid=> "acid".to_string(),
        Bludgeoning=> "bludgeoning".to_string(),
        Cold=> "cold".to_string(),
        Fire=> "fire".to_string(),
        Force=> "force".to_string(),
        Lightning=> "lightning".to_string(),
        Necrotic=> "necrotic".to_string(),
        Piercing=> "piercing".to_string(),
        Poison=> "poison".to_string(),
        Psychic=> "psychic".to_string(),
        Radiant=> "radiant".to_string(),
        Slashing=> "slashing".to_string(),
        Thunder=> "thunder".to_string(),
    }
}

pub struct CharacterPreferences {
    pub name: String,
    pub ancestry: String,
    pub culture: String,
    pub age: Option<i16>,
    pub height: Option<i16>,
    pub weight: Option<i16>,
    pub alignment: Option<String>,
    pub skin_tone: Option<String>,
    pub hair_color: Option<String>,
    pub hair_type: Option<String>,
    pub eye_color: Option<String>,
    pub abilities: Option<HashMap<String, HashMap<String, CharacterAbility>>>,
    pub tool_proficiencies: Option<Vec<String>>
}

impl Default for CharacterPreferences {
    fn default() -> Self {
        CharacterPreferences {
            name: "None".to_string(),
            ancestry: "None".to_string(),
            culture: "None".to_string(),
            age: None,
            height: None,
            weight: None,
            alignment: None,
            skin_tone: None,
            hair_color: None,
            hair_type: None,
            eye_color: None,
            abilities: None,
            tool_proficiencies: None,
        }
    }
}
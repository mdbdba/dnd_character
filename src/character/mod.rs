use std::collections::HashMap;
use crate::ancestry::CharacterAbility;

pub struct CharacterPreferences {
    pub name: String,
    pub ancestry: String,
    pub culture: String,
    pub age: i16,
    pub height: i16,
    pub weight: i16,
    pub alignment: String,
    pub skin_tone: String,
    pub hair_color: String,
    pub hair_type: String,
    pub eye_color: String,
    pub abilities: Option<HashMap<String, HashMap<String, CharacterAbility>>>,
}

impl Default for CharacterPreferences {
    fn default() -> Self {
        CharacterPreferences {
            name: "None".to_string(),
            ancestry: "None".to_string(),
            culture: "None".to_string(),
            age: -999,
            height: -999,
            weight: -999,
            alignment: "None".to_string(),
            skin_tone: "None".to_string(),
            hair_color: "None".to_string(),
            hair_type: "None".to_string(),
            eye_color: "None".to_string(),
            abilities: None,
        }
    }
}
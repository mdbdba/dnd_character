use std::collections::HashMap;
use crate::ancestry::CharacterAbility;

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
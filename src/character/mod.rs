
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
    pub eye_color: String
}

impl Default for CharacterPreferences {
    fn default() -> Self {
        CharacterPreferences {
            name: String::from("None"),
            ancestry: String::from("None"),
            culture: String::from("None"),
            age: -999,
            height: -999,
            weight: -999,
            alignment: String::from("None"), 
            skin_tone: String::from("None"),
            hair_color: String::from("None"),
            hair_type: String::from("None"),
            eye_color: String::from("None")
        }
    }
}
use std::collections::HashMap;
// use crate::ancestry::CharacterAbility;


// #[derive(Clone)]
pub struct CharacterAbility {
    pub ability_name: String, // What the ability is named in the documentation
    pub category: String, // For sorting into what it does for the character. (Spell, Language)
    pub specific_effect: Vec<String>, // What is it allowing, like Spell Name, language name
    pub range: Vec<String>, // actual distances for spells and melee, or read, written, and spoken for language.
    pub mechanic: Vec<MechanicLevel>, // explaining the mechanic to be used. amt of damage, attack roll vs. save, etc
    pub availability: Vec<String>, // Always, 1 per long rest, etc.

}

pub fn get_weapon_proficiency(name: String, weapon_name: String) -> CharacterAbility {
    CharacterAbility{
        ability_name: name.clone(),
        category: "weapon".to_string(),
        specific_effect: vec!{weapon_name.clone()},
        range: vec!{"all".to_string()},
        mechanic: vec!{
            MechanicLevel {
                level: 1,
                roll_multiplier: None,
                roll_die: None,
                adjustment: None,
                category: MechanicCategory::WeaponProficiency,
            }},
        availability: vec!{"always".to_string()}
    }
}

pub fn get_tool_proficiency(name: String, tool_name: String ) -> CharacterAbility {
    CharacterAbility {
        ability_name: name.to_string(),
        category: "tool".to_string(),
        specific_effect: vec! {tool_name.clone()},
        range: vec! {"all".to_string()},
        mechanic: vec! {
            MechanicLevel {
                level: 1,
                roll_multiplier: None,
                roll_die: None,
                adjustment: None,
                category: MechanicCategory::ToolProficiency,
            }},
        availability: vec! {"always".to_string()}
    }
}



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

pub enum MechanicCategory {
    Damage,
    NoDamage,
    HalfDamage,
    AdditionalTarget,
    Advantage,
    Disadvantage,
    Sight,
    Language,
    HitPoints,
    WeaponProficiency,
    ToolProficiency
}
pub struct MechanicLevel {
    pub level: i16,
    pub roll_multiplier: Option<i16>,
    pub roll_die: Option<i16>,
    pub adjustment: Option<i16>,
    pub category: MechanicCategory,
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
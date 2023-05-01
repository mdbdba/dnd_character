use std::collections::HashMap;

pub struct CharacterAbility {
    pub ability_name: String, // What the ability is named in the documentation
    pub category: String,     // For sorting into what it does for the character. (Spell, Language)
    pub specific_effect: Vec<String>, // What is it allowing, like Spell Name, language name
    pub range: Vec<String>, // actual distances for spells and melee, or read, written, and spoken for language.
    pub mechanic: Vec<MechanicLevel>, // explaining the mechanic to be used. amt of damage, attack roll vs. save, etc
    pub availability: Vec<String>,    // Always, 1 per long rest, etc.
}

#[derive(PartialEq, Debug)]
pub struct EffectValueRange {
    pub roll_multiplier: Option<i16>,
    pub roll_die: Option<i16>,
    pub adjustment: Option<i16>,
    pub effect_type: Option<DamageType>,
}

pub fn get_check_proficiency(
    name: String,
    skill_name: String,
    condition: String,
    effect: MechanicCategory,
    level: Option<i16>,
) -> CharacterAbility {
    let derived_level: i16;
    if let Some(value) = level.clone() {
        derived_level = value;
    } else {
        derived_level = 1;
    }
    CharacterAbility {
        ability_name: name.clone(),
        category: "checks".to_string(),
        specific_effect: vec![skill_name.clone()],
        range: vec!["none".to_string()],
        mechanic: vec![MechanicLevel {
            level: derived_level,
            effect_range: None,
            category: effect,
        }],
        availability: vec![condition.clone()],
    }
}

pub fn get_weapon_proficiency(name: String, weapon_name: String) -> CharacterAbility {
    CharacterAbility {
        ability_name: name.clone(),
        category: "weapon".to_string(),
        specific_effect: vec![weapon_name.clone()],
        range: vec!["all".to_string()],
        mechanic: vec![MechanicLevel {
            level: 1,
            effect_range: None,
            category: MechanicCategory::WeaponProficiency,
        }],
        availability: vec!["always".to_string()],
    }
}

pub fn get_tool_proficiency(name: String, tool_name: String) -> CharacterAbility {
    CharacterAbility {
        ability_name: name.to_string(),
        category: "tool".to_string(),
        specific_effect: vec![tool_name.clone()],
        range: vec!["all".to_string()],
        mechanic: vec![MechanicLevel {
            level: 1,
            effect_range: None,
            category: MechanicCategory::ToolProficiency,
        }],
        availability: vec!["always".to_string()],
    }
}

pub enum Vantage {
    Advantage,
    Normal,
    Disadvantage,
}
pub fn get_vantage(src: Vantage) -> String {
    match src {
        Vantage::Advantage => "advantage".to_string(),
        Vantage::Normal => "normal".to_string(),
        Vantage::Disadvantage => "disadvantage".to_string(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    Thunder,
    None,
}

pub fn get_damage_enum(src: String) -> DamageType {
    match src.as_str() {
        "acid" => DamageType::Acid,
        "bludgeoning" => DamageType::Bludgeoning,
        "cold" => DamageType::Cold,
        "fire" => DamageType::Fire,
        "force" => DamageType::Force,
        "lightning" => DamageType::Lightning,
        "necrotic" => DamageType::Necrotic,
        "piercing" => DamageType::Piercing,
        "poison" => DamageType::Poison,
        "psychic" => DamageType::Psychic,
        "radiant" => DamageType::Radiant,
        "slashing" => DamageType::Slashing,
        "thunder" => DamageType::Thunder,
        _ => DamageType::None,
    }
}

pub fn get_damage_type(src: DamageType) -> String {
    match src {
        DamageType::Acid => "acid".to_string(),
        DamageType::Bludgeoning => "bludgeoning".to_string(),
        DamageType::Cold => "cold".to_string(),
        DamageType::Fire => "fire".to_string(),
        DamageType::Force => "force".to_string(),
        DamageType::Lightning => "lightning".to_string(),
        DamageType::Necrotic => "necrotic".to_string(),
        DamageType::Piercing => "piercing".to_string(),
        DamageType::Poison => "poison".to_string(),
        DamageType::Psychic => "psychic".to_string(),
        DamageType::Radiant => "radiant".to_string(),
        DamageType::Slashing => "slashing".to_string(),
        DamageType::Thunder => "thunder".to_string(),
        DamageType::None => "none".to_string(),
    }
}
#[derive(PartialEq, Debug)]
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
    ToolProficiency,
    DoubleProficiencyBonus,
}
pub struct MechanicLevel {
    pub level: i16,
    pub effect_range: Option<EffectValueRange>,
    pub category: MechanicCategory,
}

pub struct CharacterPreferences {
    pub name: Option<String>,
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
    pub tool_proficiencies: Option<Vec<String>>,
}

impl Default for CharacterPreferences {
    fn default() -> Self {
        CharacterPreferences {
            name: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::character::{CharacterPreferences, get_vantage, MechanicCategory, Vantage};

    #[test]
    fn test_default_preferences() {
        let prefs = CharacterPreferences {
            ..CharacterPreferences::default()
        };
        assert_eq!(prefs.name, Option::None);
        assert_eq!(prefs.ancestry, "None".to_string());
        assert_eq!(prefs.culture, "None".to_string());
        assert_eq!(prefs.age, Option::None);
        assert_eq!(prefs.height, Option::None);
        assert_eq!(prefs.weight, Option::None);
        assert_eq!(prefs.alignment, Option::None);
        assert_eq!(prefs.skin_tone, Option::None);
        assert_eq!(prefs.hair_color, Option::None);
        assert_eq!(prefs.hair_type, Option::None);
        assert_eq!(prefs.eye_color, Option::None);
        let option_match = match prefs.abilities {
            None => None,
            Some(_i) => Some("found something".to_string()),
        };
        assert_eq!(option_match, Option::None);
        assert_eq!(prefs.tool_proficiencies, Option::None);
    }
    #[test]
    fn test_vantages() {
        let adv = Vantage::Advantage;
        let norm = Vantage::Normal;
        let dis = Vantage::Disadvantage;

        assert_eq!(get_vantage(adv),"advantage".to_string());
        assert_eq!(get_vantage(norm), "normal".to_string());
        assert_eq!(get_vantage(dis), "disadvantage".to_string());

    }
    #[test]
    fn test_mechanic_categories() {
        let dmg = MechanicCategory::Damage;
        let ndmg = MechanicCategory::NoDamage;
        let hdmg = MechanicCategory::HalfDamage;
        let adt = MechanicCategory::AdditionalTarget;
        let adv = MechanicCategory::Advantage;
        let dis = MechanicCategory::Disadvantage;
        let sgt = MechanicCategory::Sight;
        let lan = MechanicCategory::Language;
        let hp = MechanicCategory::HitPoints;
        let wp = MechanicCategory::WeaponProficiency;
        let tp = MechanicCategory::ToolProficiency;
        let dpb = MechanicCategory::DoubleProficiencyBonus;
        assert_eq!(dmg,MechanicCategory::Damage);
        assert_eq!(ndmg,MechanicCategory::NoDamage);
        assert_eq!(hdmg,MechanicCategory::HalfDamage);
        assert_eq!(adt,MechanicCategory::AdditionalTarget);
        assert_eq!(adv,MechanicCategory::Advantage);
        assert_eq!(dis,MechanicCategory::Disadvantage);
        assert_eq!(sgt,MechanicCategory::Sight);
        assert_eq!(lan,MechanicCategory::Language);
        assert_eq!(hp,MechanicCategory::HitPoints);
        assert_eq!(wp,MechanicCategory::WeaponProficiency);
        assert_eq!(tp,MechanicCategory::ToolProficiency);
        assert_eq!(dpb,MechanicCategory::DoubleProficiencyBonus);
    }
    #[test]
    fn test_get_damage_type() {
        assert_eq!(get_damage_type(DamageType::Acid), "acid");
        assert_eq!(get_damage_type(DamageType::Bludgeoning), "bludgeoning");
        assert_eq!(get_damage_type(DamageType::Cold), "cold");
        assert_eq!(get_damage_type(DamageType::Fire), "fire");
        assert_eq!(get_damage_type(DamageType::Force), "force");
        assert_eq!(get_damage_type(DamageType::Lightning), "lightning");
        assert_eq!(get_damage_type(DamageType::Necrotic), "necrotic");
        assert_eq!(get_damage_type(DamageType::Piercing), "piercing");
        assert_eq!(get_damage_type(DamageType::Poison), "poison");
        assert_eq!(get_damage_type(DamageType::Psychic), "psychic");
        assert_eq!(get_damage_type(DamageType::Radiant), "radiant");
        assert_eq!(get_damage_type(DamageType::Slashing), "slashing");
        assert_eq!(get_damage_type(DamageType::Thunder), "thunder");
        assert_eq!(get_damage_type(DamageType::None), "none");
    }
    #[test]
    fn test_get_weapon_proficiency() {
        let proficiency = get_weapon_proficiency(
            "Sword proficiency".to_string(),
            "Sword".to_string());
        assert_eq!(proficiency.ability_name, "Sword proficiency");
        assert_eq!(proficiency.category, "weapon");
        assert_eq!(proficiency.specific_effect, vec!["Sword"]);
        assert_eq!(proficiency.range, vec!["all"]);
        assert_eq!(proficiency.mechanic.len(), 1);
        assert_eq!(proficiency.mechanic[0].level, 1);
        assert_eq!(proficiency.mechanic[0].effect_range, None);
        assert_eq!(proficiency.mechanic[0].category, MechanicCategory::WeaponProficiency);
        assert_eq!(proficiency.availability, vec!["always"]);
    }

    #[test]
    fn test_get_check_proficiency_with_level() {
        let ability = get_check_proficiency(
            "Test Ability".to_string(),
            "Acrobatics".to_string(),
            "always".to_string(),
            MechanicCategory::Advantage,
            Some(2),
        );

        assert_eq!(ability.ability_name, "Test Ability");
        assert_eq!(ability.category, "checks");
        assert_eq!(ability.specific_effect, vec!["Acrobatics"]);
        assert_eq!(ability.range, vec!["none"]);
        let match_value = &ability.mechanic[0];
        assert_eq!(match_value.level, 2);
        assert_eq!(match_value.effect_range, None);
        assert_eq!(match_value.category, MechanicCategory::Advantage);
        assert_eq!(ability.availability, vec!["always"]);
    }

    #[test]
    fn test_get_check_proficiency_without_level() {
        let ability = get_check_proficiency(
            "Test Ability".to_string(),
            "Acrobatics".to_string(),
            "always".to_string(),
            MechanicCategory::DoubleProficiencyBonus,
            None,
        );

        assert_eq!(ability.ability_name, "Test Ability");
        assert_eq!(ability.category, "checks");
        assert_eq!(ability.specific_effect, vec!["Acrobatics"]);
        assert_eq!(ability.range, vec!["none"]);
        let match_value = &ability.mechanic[0];
        assert_eq!(match_value.level,1);
        assert_eq!(match_value.effect_range, None);
        assert_eq!(match_value.category, MechanicCategory::DoubleProficiencyBonus);
        assert_eq!(ability.availability, vec!["always"]);
    }



}
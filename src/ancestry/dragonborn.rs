/*
Dragonborn
Common wisdom is that those of dragonborn ancestry descend from real dragons, inheriting the color
of their scales and their affinity for certain elements from those draconic forebears. Dragonborn
culture, however, has little in common with that of dragons, having developed its own distinct
beliefs and traditions.

Ancestral Traits
Your draconic physiology manifests in a variety of traits you share with other dragonborn.

Age. Young dragonborn grow quickly. They walk hours after being born, attain the size and
development of a 10-year-old human child by the age of 3, and reach adulthood by 15. They live
to be around 80.

Size. Dragonborn are taller and heavier than humans, standing well over 6 feet tall and averaging
almost 250 pounds. Your size is Medium.

Speed. Your base walking speed is 30 feet.

Draconic Ancestry. You have draconic ancestry. Choose one type of dragon from the Draconic Ancestry
table. Your breath weapon and damage resistance are determined by the dragon type, as shown in the
table.

Breath Weapon. You can use your action to exhale destructive energy. Your draconic ancestry
determines the size, shape, and damage type of the exhalation.

When you use your breath weapon, each creature in the area of the exhalation must make a saving
throw, the type of which is determined by your draconic ancestry. The DC for this saving throw
equals 8 + your Constitution modifier + your proficiency bonus. A creature takes 2d6 damage on a
failed save, and half as much damage on a successful one. The damage increases to 3d6 at 6th
level, 4d6 at 11th level, and 5d6 at 16th level.

After you use your breath weapon, you can’t use it again until you complete a short or long rest.

Dragon	Damage Type	Breath Weapon
Black	Acid	    5 by 30 ft. line (Dex. save)
Blue	Lightning	5 by 30 ft. line (Dex. save)
Brass	Fire	    5 by 30 ft. line (Dex. save)
Bronze	Lightning	5 by 30 ft. line (Dex. save)
Copper	Acid	    5 by 30 ft. line (Dex. save)
Gold	Fire	    15 ft. cone (Dex. save)
Green	Poison	    15 ft. cone (Con. save)
Red	Fire	        15 ft. cone (Dex. save)
Silver	Cold	    15 ft. cone (Con. save)
White	Cold	    15 ft. cone (Con. save)
Damage Resistance. You have resistance to the damage type associated with your draconic ancestry.

*/

use crate::ancestry::{
    get_i16_some_value, get_lc_some_value, set_ability_bonuses, set_alignments, AncestralTraits,
    BaseAncestralTraits, BaseCulturalTraits, CharacterAbility, CulturalTraits, LanguageTraits,
};
use crate::character::{
    get_check_proficiency, get_damage_enum, CharacterPreferences, EffectValueRange,
    MechanicCategory, MechanicLevel,
};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub enum DragonbornColor {
    Black,
    Blue,
    Brass,
    Bronze,
    Copper,
    Gold,
    Green,
    Red,
    Silver,
    White,
}
impl Distribution<DragonbornColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DragonbornColor {
        match rng.gen_range(0..10) {
            0 => DragonbornColor::Black,
            1 => DragonbornColor::Blue,
            2 => DragonbornColor::Brass,
            3 => DragonbornColor::Bronze,
            4 => DragonbornColor::Copper,
            5 => DragonbornColor::Gold,
            6 => DragonbornColor::Green,
            7 => DragonbornColor::Red,
            8 => DragonbornColor::Silver,
            9 => DragonbornColor::White,
            _ => DragonbornColor::Red,
        }
    }
}

fn confirm_dragonborn_color(src: String) -> DragonbornColor {
    return match src.as_str() {
        "black dragonborn" => DragonbornColor::Black,
        "blue dragonborn" => DragonbornColor::Blue,
        "brass dragonborn" => DragonbornColor::Brass,
        "bronze dragonborn" => DragonbornColor::Bronze,
        "copper dragonborn" => DragonbornColor::Copper,
        "gold dragonborn" => DragonbornColor::Gold,
        "green dragonborn" => DragonbornColor::Green,
        "red dragonborn" => DragonbornColor::Red,
        "silver dragonborn" => DragonbornColor::Silver,
        "white dragonborn" => DragonbornColor::White,
        _ => {
            let color: DragonbornColor = rand::random();
            color
        }
    };
}

pub fn new_dragonborn_ancestry(prefs: &mut CharacterPreferences) -> AncestralTraits {
    let parent_name = "dragonborn".to_string();
    let color = confirm_dragonborn_color(prefs.ancestry.clone());

    let skin_color: String;
    let resistance: String;
    let effect_range: Vec<String>;
    match color {
        DragonbornColor::Black => {
            skin_color = "black scales".to_string();
            resistance = "acid".to_string();
            effect_range = vec![
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::Blue => {
            skin_color = "blue scales".to_string();
            resistance = "lightning".to_string();
            effect_range = vec![
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::Brass => {
            skin_color = "brass scales".to_string();
            resistance = "fire".to_string();
            effect_range = vec![
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::Bronze => {
            skin_color = "bronze scales".to_string();
            resistance = "lightning".to_string();
            effect_range = vec![
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::Copper => {
            skin_color = "copper scales".to_string();
            resistance = "acid".to_string();
            effect_range = vec![
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::Gold => {
            skin_color = "gold scales".to_string();
            resistance = "fire".to_string();
            effect_range = vec![
                "15 foot cone".to_string(),
                "dexterity".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::Green => {
            skin_color = "green scales".to_string();
            resistance = "poison".to_string();
            effect_range = vec![
                "15 foot cone".to_string(),
                "constitution".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::Red => {
            skin_color = "red scales".to_string();
            resistance = "fire".to_string();
            effect_range = vec![
                "15 foot cone".to_string(),
                "dexterity".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::Silver => {
            skin_color = "silver scales".to_string();
            resistance = "cold".to_string();
            effect_range = vec![
                "15 foot cone".to_string(),
                "constitution".to_string(),
                "half damage".to_string(),
            ];
        }
        DragonbornColor::White => {
            skin_color = "white scales".to_string();
            resistance = "cold".to_string();
            effect_range = vec![
                "15 foot cone".to_string(),
                "constitution".to_string(),
                "half damage".to_string(),
            ];
        }
    };

    let combined_name = format!("{:?} {}", color, parent_name.clone()).to_lowercase();

    let mut ancestry_abilities: HashMap<String, HashMap<String, CharacterAbility>> = HashMap::new();

    let resistances = BaseAncestralTraits::add_resistances(vec![resistance.clone()]);

    let damage_enum = get_damage_enum(resistance.clone());
    let mut offensive: HashMap<String, CharacterAbility> = HashMap::new();
    let ability1 = CharacterAbility {
        ability_name: "breath weapon".to_string(),
        category: "offensive".to_string(),
        specific_effect: vec![resistance.clone()],
        range: effect_range,
        mechanic: vec![
            MechanicLevel {
                level: 1,
                effect_range: Some(EffectValueRange {
                    roll_multiplier: Some(2),
                    roll_die: Some(6),
                    adjustment: None,
                    effect_type: Some(damage_enum),
                }),
                category: MechanicCategory::Damage,
            },
            MechanicLevel {
                level: 6,
                effect_range: Some(EffectValueRange {
                    roll_multiplier: Some(3),
                    roll_die: Some(6),
                    adjustment: None,
                    effect_type: Some(damage_enum),
                }),
                category: MechanicCategory::Damage,
            },
            MechanicLevel {
                level: 11,
                effect_range: Some(EffectValueRange {
                    roll_multiplier: Some(4),
                    roll_die: Some(6),
                    adjustment: None,
                    effect_type: Some(damage_enum),
                }),
                category: MechanicCategory::Damage,
            },
            MechanicLevel {
                level: 16,
                effect_range: Some(EffectValueRange {
                    roll_multiplier: Some(5),
                    roll_die: Some(6),
                    adjustment: None,
                    effect_type: Some(damage_enum),
                }),
                category: MechanicCategory::Damage,
            },
        ],
        availability: vec!["short rest".to_string(), "long rest".to_string()],
    };
    offensive.insert("breath weapon".to_string(), ability1);

    ancestry_abilities.insert("resistances".to_string(), resistances);
    ancestry_abilities.insert("offensive".to_string(), offensive);

    let base_values = BaseAncestralTraits {
        name: combined_name.clone(),
        parent_name,
        maturity_age: 15,
        avg_max_age: 80,
        base_walking_speed: 30,
        height_min_inches: 70,
        height_modifier_multiplier: 2,
        height_modifier_die: 6,
        height_modifier_adj: 0,
        weight_min_pounds: 180,
        weight_modifier_multiplier: 6,
        weight_modifier_die: 12,
        weight_modifier_adj: 0,
        base_size: "medium".to_string(),
        skin_tones: {
            vec![skin_color]
        },
        hair_colors: {
            vec!["none".to_string()]
        },
        hair_types: {
            vec!["none".to_string()]
        },
        eye_colors: {
            vec![
                "purple".to_string(),
                "red".to_string(),
                "orange".to_string(),
                "yellow".to_string(),
                "blue".to_string(),
                "gray".to_string(),
                "silver".to_string(),
                "black".to_string(),
            ]
        },
        abilities: ancestry_abilities,
        source_material: { "SRD".to_string() },
        source_credit_url: {
            "https://www.dndbeyond.com/attachments/39j2li89/SRD5.1-CCBY4.0_License_live%20links.pdf"
                .to_string()
        },
        source_credit_comment: { "As of 2023/03/25".to_string() },
    };
    let name = base_values.get_name();
    let parent_name = base_values.get_parent_name();
    let base_size = base_values.get_base_size();
    let base_walking_speed = base_values.get_base_walking_speed();

    AncestralTraits::combiner(prefs, &base_values);

    let wonderful = "wonderful".to_string();

    AncestralTraits {
        name,
        parent_name,
        age: get_i16_some_value(prefs.age, base_values.get_age()),
        base_size,
        base_walking_speed,
        height: get_i16_some_value(prefs.height, base_values.get_height()),
        weight: get_i16_some_value(prefs.weight, base_values.get_weight()),
        skin_tone: get_lc_some_value(prefs.skin_tone.clone(), wonderful.clone()),
        hair_color: get_lc_some_value(prefs.hair_color.clone(), wonderful.clone()),
        hair_type: get_lc_some_value(prefs.hair_type.clone(), wonderful.clone()),
        eye_color: get_lc_some_value(prefs.eye_color.clone(), wonderful.clone()),
        abilities: base_values.abilities,
        source_material: base_values.source_material,
        source_credit_url: base_values.source_credit_url,
        source_credit_comment: base_values.source_credit_comment,
    }
}

/*
Cultural Traits
Dragonborn culture is intense and exciting, leading those raised within it to be striking and
remarkable individuals. Grand festivals and elaborate holidays are frequent, each centered around
a different physical competition or performance.

In general, practices in dragonborn culture and education tend to promote athleticism and personal
character.

Ability Score Increase. Your Strength score increases by 2, and your Charisma score increases by 1.

Alignment. Because dragonborn culture values intense commitments and expression, many raised in
this culture find themselves drawn to one side or the other in the cosmic war between good and evil.
Most dragonborn are good, but those who side with evil can be terrible villains.

Languages. You can speak, read, and write Common and Draconic. Draconic is thought to be one of the
oldest languages and is often used in the study of magic. The language sounds harsh to most other
creatures and includes numerous hard consonants and sibilants.

Dragon Lore. Dragonborn communities are often proud of their draconic heritage. You have advantage
on any Intelligence checks to recall information about dragons.

*/

pub fn new_dragonborn_culture(prefs: &mut CharacterPreferences) -> CulturalTraits {
    let parent_name = "dragonborn".to_string();
    let color = confirm_dragonborn_color(prefs.ancestry.clone());
    let combined_name = format!("{:?} {}", color, parent_name.clone()).to_lowercase();
    let mut cultural_abilities: HashMap<String, HashMap<String, CharacterAbility>> = HashMap::new();

    let languages = BaseCulturalTraits::add_languages(vec![
        LanguageTraits {
            name: "common".to_string(),
            can_read: true,
            can_speak: true,
            can_write: true,
        },
        LanguageTraits {
            name: "draconic".to_string(),
            can_read: true,
            can_speak: true,
            can_write: true,
        },
    ]);

    let mut checks: HashMap<String, CharacterAbility> = HashMap::new();
    let ability1 = get_check_proficiency(
        "dragon lore".to_string(),
        "intelligence".to_string(),
        "specific to dragons".to_string(),
        MechanicCategory::Advantage,
        None,
    );
    checks.insert("intelligence".to_string(), ability1);

    cultural_abilities.insert("languages".to_string(), languages);
    cultural_abilities.insert("checks".to_string(), checks);

    let base_values = BaseCulturalTraits {
        alignments: set_alignments(30, 10, 4, 4, 4, 10, 4, 4, 30),
        tool_proficiency_choices: None,
        ability_bonuses: set_ability_bonuses(2, 0, 0, 0, 0, 1),
        abilities: cultural_abilities,
    };

    CulturalTraits::combiner(prefs, &base_values);

    CulturalTraits {
        name: combined_name,
        parent_name,
        alignment: get_lc_some_value(prefs.alignment.clone(), "true neutral".to_string()),
        ability_bonuses: base_values.ability_bonuses,
        abilities: base_values.abilities,
        source_material: { "SRD".to_string() },
        source_credit_url: {
            "https://www.dndbeyond.com/attachments/39j2li89/SRD5.1-CCBY4.0_License_live%20links.pdf"
                .to_string()
        },
        source_credit_comment: { "As of 2023/03/25".to_string() },
    }
}

#[cfg(test)]
mod tests {
    use crate::ancestry::{AncestralTraits, CulturalTraits};
    use crate::character::CharacterPreferences;

    #[test]
    fn test_black_dragonborn_ancestry() {
        let mut prefs = CharacterPreferences {
            ancestry: "black dragonborn".to_string(),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.name, "black dragonborn".to_string());
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert!(db.age >= 15 && db.age <= 80, "Expected 15, got {}", db.age);
        assert_eq!(db.base_walking_speed, 30);
        assert!(
            db.height > 70 && db.height <= 82,
            "Expected between 70..82, got {}",
            db.height
        );
        assert!(
            db.weight > 180 && db.weight <= 250,
            "Expected between 180..250, got {}",
            db.weight
        );
        assert!(db.skin_tone.len() > 0, "Skin tone is empty");
        assert!(db.hair_color.len() > 0, "Hair color is empty");
        assert!(db.hair_type.len() > 0, "Hair type is empty");
        assert!(db.eye_color.len() > 0, "Eye color is empty");
        let result = db
            .abilities
            .get("resistances")
            .and_then(|b| b.get("acid"))
            .unwrap();

        assert_eq!(result.ability_name, "damage resistance".to_string());
    }
    #[test]
    fn test_blue_dragonborn_ancestry() {
        let mut prefs = CharacterPreferences {
            ancestry: "blue dragonborn".to_string(),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.name, "blue dragonborn".to_string());
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert_eq!(db.abilities.len(), 2);
    }
    #[test]
    fn test_random_dragonborn_ancestry() {
        let mut prefs = CharacterPreferences {
            ancestry: "dragonborn".to_string(),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert!(
            db.name.len() > 12,
            "Expecting anything with dragonborn: found {}",
            db.name
        );
    }
    #[test]
    fn test_default_ancestry() {
        let mut prefs = CharacterPreferences {
            ancestry: "born free".to_string(),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert!(
            db.name.len() > 12,
            "Expecting anything with dragonborn: found {}",
            db.name
        );
    }

    #[test]
    fn test_all_ancestry_prefs() {
        let mut prefs = CharacterPreferences {
            name: Some("chuck".to_string()),
            ancestry: "silver dragonborn".to_string(),
            age: Some(99),
            height: Some(92),
            weight: Some(275),
            alignment: Some("chaotic neutral".to_string()),
            culture: "dragonborn".to_string(),
            skin_tone: Some("silver".to_string()),
            eye_color: Some("black".to_string()),
            hair_type: Some("bald".to_string()),
            hair_color: Some("none".to_string()),
            abilities: None,
            tool_proficiencies: None,
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.name, "silver dragonborn".to_string());
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert_eq!(db.age, 99);
        assert_eq!(db.base_walking_speed, 30);
        assert_eq!(db.height, 92);
        assert_eq!(db.weight, 275);
        assert_eq!(db.base_size, "medium".to_string());
        assert_eq!(db.skin_tone, "silver".to_string());
        assert_eq!(db.hair_color, "none".to_string());
        assert_eq!(db.hair_type, "bald".to_string());
        assert_eq!(db.eye_color, "black".to_string());
    }

    #[test]
    fn test_blue_dragonborn_culture() {
        let mut prefs = CharacterPreferences {
            ancestry: "blue dragonborn".to_string(),
            alignment: Some("chaotic neutral".to_string()),
            ..CharacterPreferences::default()
        };
        let db = CulturalTraits::new(&mut prefs);
        assert_eq!(db.name, "blue dragonborn".to_string());
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert_eq!(db.alignment, "chaotic neutral".to_string());
        assert_eq!(db.abilities.len(), 2);
        let common = &db.abilities["languages"]["common"];
        assert_eq!(common.ability_name, "ancestral language".to_string());
        assert_eq!(common.range.len(), 3);
        assert!(
            common.range.contains(&"read".to_string()),
            "read value was not found"
        );

        let draconic = &db.abilities["languages"]["draconic"];
        assert_eq!(draconic.ability_name, "ancestral language".to_string());
        assert_eq!(draconic.range.len(), 3);
        assert!(
            draconic.range.contains(&"read".to_string()),
            "read value was not found"
        );
    }
}

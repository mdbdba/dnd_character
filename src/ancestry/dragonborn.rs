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

use std::collections::HashMap;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use crate::ancestry::{AncestralTraits, BaseAncestralTraits, CharacterAbility};
use crate::character::CharacterPreferences;

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

pub fn new_dragonborn(prefs: &mut CharacterPreferences) -> AncestralTraits {
    let parent_name = "dragonborn".to_string();
    let ans_string = prefs.ancestry.clone();
    let ans = &ans_string[..];

    let color = match ans {
        "black dragonborn"  => DragonbornColor::Black ,
        "blue dragonborn"   => DragonbornColor::Blue ,
        "brass dragonborn"  => DragonbornColor::Brass ,
        "bronze dragonborn" => DragonbornColor::Bronze ,
        "copper dragonborn" => DragonbornColor::Copper ,
        "gold dragonborn"   => DragonbornColor::Gold ,
        "green dragonborn"  => DragonbornColor::Green ,
        "red dragonborn"    => DragonbornColor::Red,
        "silver dragonborn" => DragonbornColor::Silver ,
        "white dragonborn"  => DragonbornColor::White ,
        _ => {
            let color: DragonbornColor = rand::random();
            color
        },
    };


    let skin_color: String;
    let resistance: String;
    let effect_range: Vec<String>;
    match color {
        DragonbornColor::Black => {
            skin_color = "black scales".to_string();
            resistance = "acid".to_string();
            effect_range = vec! {
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::Blue => {
            skin_color = "blue scales".to_string();
            resistance = "lightning".to_string();
            effect_range = vec! {
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::Brass => {
            skin_color = "brass scales".to_string();
            resistance = "fire".to_string();
            effect_range = vec! {
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::Bronze => {
            skin_color = "bronze scales".to_string();
            resistance = "lightning".to_string();
            effect_range = vec! {
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::Copper => {
            skin_color = "copper scales".to_string();
            resistance = "acid".to_string();
            effect_range = vec! {
                "5 by 30 foot line".to_string(),
                "dexterity".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::Gold => {
            skin_color = "gold scales".to_string();
            resistance = "fire".to_string();
            effect_range = vec! {
                "15 foot cone".to_string(),
                "dexterity".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::Green => {
            skin_color = "green scales".to_string();
            resistance = "poison".to_string();
            effect_range = vec! {
                "15 foot cone".to_string(),
                "constitution".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::Red => {
            skin_color = "red scales".to_string();
            resistance = "fire".to_string();
            effect_range = vec! {
                "15 foot cone".to_string(),
                "dexterity".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::Silver => {
            skin_color = "silver scales".to_string();
            resistance = "cold".to_string();
            effect_range = vec! {
                "15 foot cone".to_string(),
                "constitution".to_string(),
                "half damage".to_string()
            };
        },
        DragonbornColor::White  => {
            skin_color = "white scales".to_string();
            resistance = "cold".to_string();
            effect_range = vec! {
                "15 foot cone".to_string(),
                "constitution".to_string(),
                "half damage".to_string()
            };
        },
    };

    let combined_name = format!("{:?} {}", color, parent_name.clone()).to_lowercase();

    let mut ancestry_abilities: HashMap<String, HashMap<String, CharacterAbility>> = HashMap::new();

    let mut resistances: HashMap<String, CharacterAbility> = HashMap::new();
    let ability1 = CharacterAbility{
        ability_name: "damage resistance".to_string(),
        category: "resistance".to_string(),
        specific_effect: vec! {resistance.clone()},
        range: vec!{"all".to_string()},
        mechanic: vec!{"half damage".to_string()},
        availability: vec!{"always".to_string()}
    };
    resistances.insert(resistance.clone(), ability1);

    let mut languages: HashMap<String, CharacterAbility> = HashMap::new();
    let ability1 = CharacterAbility{
            ability_name: "ancestral language".to_string(),
            category: "language".to_string(),
            specific_effect: vec!{"common".to_string()},
            range: vec!{
                "speak".to_string(),
                "read".to_string(),
                "write".to_string()},
            mechanic: vec!{"none".to_string()},
            availability: vec!{"always".to_string()}
        };
    let ability2 =  CharacterAbility{
        ability_name: "ancestral language".to_string(),
        category: "language".to_string(),
        specific_effect: vec!{"draconic".to_string()},
        range: vec!{
            "speak".to_string(),
            "read".to_string(),
            "write".to_string()},
        mechanic: vec!{"none".to_string()},
        availability: vec!{"always".to_string()}
    };
    languages.insert("common".to_string(), ability1);
    languages.insert("draconic".to_string(), ability2);


    let mut checks: HashMap<String, CharacterAbility> = HashMap::new();
    let ability1 = CharacterAbility{
        ability_name: "dragon lore".to_string(),
        category: "checks".to_string(),
        specific_effect: vec!{
            "intelligence".to_string(),
            "specific to dragons".to_string()
        },
        range: vec!{"none".to_string()},
        mechanic: vec!{"advantage".to_string()},
        availability: vec!{"always".to_string()}
    };
    checks.insert("intelligence".to_string(), ability1);

    let mut offensive: HashMap<String, CharacterAbility> = HashMap::new();
    let ability1 = CharacterAbility{
        ability_name: "breath weapon".to_string(),
        category: "offensive".to_string(),
        specific_effect: vec! {resistance.clone()},
        range: effect_range,
        mechanic: vec!{
            "1:2d6".to_string(),
            "6:3d6".to_string(),
            "11:4d6".to_string(),
            "16:5d6".to_string()
        },
        availability: vec!{
            "short rest".to_string(),
            "long rest".to_string()
        }
    };
    offensive.insert("breath weapon".to_string(), ability1);

    ancestry_abilities.insert("resistances".to_string(), resistances);
    ancestry_abilities.insert("languages".to_string(), languages);
    ancestry_abilities.insert("offensive".to_string(), offensive);
    ancestry_abilities.insert("checks".to_string(), checks);

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
        alignments: {
            let mut alignments = HashMap::new();
            alignments.insert("lawful good".to_string(), 30);
            alignments.insert("neutral good".to_string(), 10);
            alignments.insert("chaotic good".to_string(), 4);
            alignments.insert("lawful neutral".to_string(), 4);
            alignments.insert("true neutral".to_string(), 4);
            alignments.insert("chaotic neutral".to_string(), 10);
            alignments.insert("lawful evil".to_string(), 4);
            alignments.insert("neutral evil".to_string(), 4);
            alignments.insert("chaotic evil".to_string(), 30);
            alignments
        },
        skin_tones:  { vec! {skin_color} },
        hair_colors: { vec! {"none".to_string()} },
        hair_types:  { vec! {"none".to_string()} },
        eye_colors:  {
            vec! {
                "purple".to_string(),
                "red".to_string(),
                "orange".to_string(),
                "yellow".to_string(),
                "blue".to_string(),
                "gray".to_string(),
                "silver".to_string(),
                "black".to_string()
            }
        },
        abilities: ancestry_abilities,
        source_material: { "SRD".to_string() },
        source_credit_url: {
            "https://www.dndbeyond.com/attachments/39j2li89/SRD5.1-CCBY4.0_License_live%20links.pdf".to_string()
        },
        source_credit_comment: {"As of 2023/03/25".to_string() }
    };
    let name= base_values.get_name();
    let parent_name = base_values.get_parent_name();
    let base_size = base_values.get_base_size();
    let base_walking_speed = base_values.get_base_walking_speed();

    AncestralTraits::combiner(prefs, &base_values);

    AncestralTraits {
        name,
        parent_name,
        age: prefs.age,
        base_size,
        base_walking_speed,
        height: prefs.height,
        weight: prefs.weight,
        alignment: prefs.alignment.clone(),
        skin_tone: prefs.skin_tone.clone(),
        hair_color: prefs.hair_color.clone(),
        hair_type: prefs.hair_type.clone(),
        eye_color: prefs.eye_color.clone(),
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


#[cfg(test)]
mod tests {
    use crate::ancestry::AncestralTraits;
    use crate::character::CharacterPreferences;


    #[test]
    fn test_black_dragonborn() {
        let mut prefs = CharacterPreferences {
            ancestry:  "black dragonborn".to_string(),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.name, "black dragonborn".to_string());
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert!(db.age >= 15 && db.age <= 80, "Expected 15, got {}", db.age);
        assert_eq!(db.base_walking_speed, 30);
        assert!(db.height > 70 && db.height <=82, "Expected between 70..82, got {}", db.height);
        assert!(db.weight > 180 && db.weight <=250, "Expected between 180..250, got {}", db.weight);
        assert!(db.skin_tone.len() > 0, "Skin tone is empty" );
        assert!(db.hair_color.len() > 0, "Hair color is empty" );
        assert!(db.hair_type.len() > 0, "Hair type is empty" );
        assert!(db.eye_color.len() > 0, "Eye color is empty" );
        // let result = db.abilities.iter().any(|b| b.ability_name.contains(&String::from("damage resistance")));
        let result = db.abilities.get("resistances")
            .and_then(|b| b.get("acid")).unwrap();

        assert_eq!(result.ability_name, "damage resistance".to_string());
    }
    #[test]
    fn test_blue_dragonborn() {
        let mut prefs = CharacterPreferences {
            ancestry:  "blue dragonborn".to_string(),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.name, "blue dragonborn".to_string());
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert_eq!(db.abilities.len(), 4);
    }
    #[test]
    fn test_random_dragonborn() {

        let mut prefs = CharacterPreferences {
            ancestry:  "dragonborn".to_string(),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert!(db.name.len() > 12, "Expecting anything with dragonborn: found {}", db.name);
    }
    #[test]
    fn test_default_ancestry() {

        let mut prefs = CharacterPreferences {
            ancestry:  "born free".to_string(),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert!(db.name.len() > 12, "Expecting anything with dragonborn: found {}", db.name);
    }

    #[test]
    fn test_all_ancestry_prefs() {
        let mut prefs = CharacterPreferences {
            name: "chuck".to_string(),
            ancestry: "silver dragonborn".to_string(),
            age: 99,
            height: 92,
            weight: 275,
            alignment: "chaotic neutral".to_string(),
            culture: "dragonborn".to_string(),
            skin_tone: "silver".to_string(),
            eye_color: "black".to_string(),
            hair_type: "bald".to_string(),
            hair_color: "none".to_string(),
        };
        let db = AncestralTraits::new(&mut prefs);
        assert_eq!(db.name, "silver dragonborn".to_string());
        assert_eq!(db.parent_name, "dragonborn".to_string());
        assert_eq!(db.age, 99);
        assert_eq!(db.base_walking_speed, 30);
        assert_eq!(db.height, 92);
        assert_eq!(db.weight, 275);
        assert_eq!(db.base_size, "medium".to_string());
        assert_eq!(db.alignment, "chaotic neutral".to_string());
        assert_eq!(db.skin_tone, "silver".to_string());
        assert_eq!(db.hair_color, "none".to_string());
        assert_eq!(db.hair_type, "bald".to_string());
        assert_eq!(db.eye_color, "black".to_string());
    }
}
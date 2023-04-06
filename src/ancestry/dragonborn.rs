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

// pub fn new_dragonborn(color: DragonbornColor) -> AncestralTraits {
pub fn new_dragonborn(prefs: &CharacterPreferences) -> AncestralTraits {
    let parent_name = String::from("dragonborn");
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
            skin_color = String::from("black scales");
            resistance = String::from("acid");
            effect_range = vec! {
                String::from("5 by 30 foot line"),
                String::from("dexterity"),
                String::from("half damage")
            };
        },
        DragonbornColor::Blue => {
            skin_color = String::from("blue scales");
            resistance = String::from("lightning");
            effect_range = vec! {
                String::from("5 by 30 foot line"),
                String::from("dexterity"),
                String::from("half damage")
            };
        },
        DragonbornColor::Brass => {
            skin_color = String::from("brass scales");
            resistance = String::from("fire");
            effect_range = vec! {
                String::from("5 by 30 foot line"),
                String::from("dexterity"),
                String::from("half damage")
            };
        },
        DragonbornColor::Bronze => {
            skin_color = String::from("bronze scales");
            resistance = String::from("lightning");
            effect_range = vec! {
                String::from("5 by 30 foot line"),
                String::from("dexterity"),
                String::from("half damage")
            };
        },
        DragonbornColor::Copper => {
            skin_color = String::from("copper scales");
            resistance = String::from("acid");
            effect_range = vec! {
                String::from("5 by 30 foot line"),
                String::from("dexterity"),
                String::from("half damage")
            };
        },
        DragonbornColor::Gold => {
            skin_color = String::from("gold scales");
            resistance = String::from("fire");
            effect_range = vec! {
                String::from("15 foot cone"),
                String::from("dexterity"),
                String::from("half damage")
            };
        },
        DragonbornColor::Green => {
            skin_color = String::from("green scales");
            resistance = String::from("poison");
            effect_range = vec! {
                String::from("15 foot cone"),
                String::from("constitution"),
                String::from("half damage")
            };
        },
        DragonbornColor::Red => {
            skin_color = String::from("red scales");
            resistance = String::from("fire");
            effect_range = vec! {
                String::from("15 foot cone"),
                String::from("dexterity"),
                String::from("half damage")
            };
        },
        DragonbornColor::Silver => {
            skin_color = String::from("silver scales");
            resistance = String::from("cold");
            effect_range = vec! {
                String::from("15 foot cone"),
                String::from("constitution"),
                String::from("half damage")
            };
        },
        DragonbornColor::White  => {
            skin_color = String::from("white scales");
            resistance = String::from("cold");
            effect_range = vec! {
                String::from("15 foot cone"),
                String::from("constitution"),
                String::from("half damage")
            };
        },
    };

    let combined_name = format!("{:?} {}", color, parent_name.clone()).to_lowercase();
    println!("The combined name: {combined_name}");
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
        base_size: String::from("medium"),
        alignments: {
            let mut alignments = HashMap::new();
            alignments.insert(String::from("Lawful Good"), 30);
            alignments.insert(String::from("Neutral Good"), 10);
            alignments.insert(String::from("Chaotic Good"), 4);
            alignments.insert(String::from("Lawful Neutral"), 4);
            alignments.insert(String::from("True Neutral"), 4);
            alignments.insert(String::from("Chaotic Neutral"), 10);
            alignments.insert(String::from("Lawful Evil"), 4);
            alignments.insert(String::from("Neutral Evil"), 4);
            alignments.insert(String::from("Chaotic Evil"), 30);
            alignments
        },
        skin_tones: { vec! {skin_color} },
        hair_colors: { vec! {String::from("none")} },
        hair_types: { vec! {String::from("none")} },
        eye_colors: {
            vec! {
                String::from("Purple"),
                String::from("Red"),
                String::from("Orange"),
                String::from("Yellow"),
                String::from("Blue"),
                String::from("Gray"),
                String::from("Silver"),
                String::from("Black")
            }
        },
        source_material: { String::from("SRD") },
        source_credit_url: {
            String::from("https://www.dndbeyond.com/attachments/39j2li89/SRD5.1-CCBY4.0_License_live%20links.pdf")
        },
        source_credit_comment: {String::from("As of 2023/03/25") }
    };
    let name = base_values.get_name();
    let parent_name = base_values.get_parent_name();
    let age = base_values.get_age();
    let base_size = base_values.get_base_size();
    let base_walking_speed = base_values.get_base_walking_speed();
    let height = base_values.get_height();
    let weight = base_values.get_weight();
    let alignment = base_values.get_alignment();
    let skin_tone = base_values.get_skin_tone();
    let hair_color = base_values.get_hair_color();
    let hair_type = base_values.get_hair_type();
    let eye_color = base_values.get_eye_color();

    AncestralTraits {
        name,
        parent_name,
        age,
        base_size,
        base_walking_speed,
        height,
        weight,
        alignment,
        skin_tone,
        hair_color,
        hair_type,
        eye_color,
        abilities: vec!{
            CharacterAbility{
                ability_name: String::from("breath weapon"),
                category: String::from("offensive"),
                specific_effect: vec! {resistance.clone()},
                range: effect_range,
                mechanic: vec!{
                    String::from("1:2d6"),
                    String::from("6:3d6"),
                    String::from("11:4d6"),
                    String::from("16:5d6")
                },
                availability: vec!{
                    String::from("short rest"),
                    String::from("long rest")
                }
            },
            CharacterAbility{
                ability_name: String::from("damage resistance"),
                category: String::from("resistance"),
                specific_effect: vec! {resistance.clone()},
                range: vec!{String::from("all")},
                mechanic: vec!{String::from("half damage")},
                availability: vec!{String::from("always")}
            },
            CharacterAbility{
                ability_name: String::from("ancestral language"),
                category: String::from("language"),
                specific_effect: vec!{
                    String::from("common"),
                    String::from("draconic")},
                range: vec!{
                    String::from("speak"),
                    String::from("read"),
                    String::from("write")},
                mechanic: vec!{String::from("none")},
                availability: vec!{String::from("always")}
            },
            CharacterAbility{
                ability_name: String::from("dragon lore"),
                category: String::from("checks"),
                specific_effect: vec!{
                    String::from("intelligence"),
                    String::from("specific to dragons")
                },
                range: vec!{String::from("none")},
                mechanic: vec!{String::from("advantage")},
                availability: vec!{String::from("always")}
            },
        },
        source_material: base_values.source_material,
        source_credit_url: base_values.source_credit_url,
        source_credit_comment: base_values.source_credit_comment,
    }
}

#[cfg(test)]
mod tests {
    use crate::ancestry::AncestralTraits;
    use crate::character::CharacterPreferences;
    // use super::*;

    #[test]
    fn test_black_dragonborn() {
        let prefs = CharacterPreferences {
            ancestry:  String::from("black dragonborn"),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&prefs);
        assert_eq!(db.name, String::from("black dragonborn"));
        assert_eq!(db.parent_name, String::from("dragonborn"));
        assert!(db.age >= 15 && db.age <= 80, "Expected 15, got {}", db.age);
        assert_eq!(db.base_walking_speed, 30);
        assert!(db.height > 70 && db.height <=82, "Expected between 70..82, got {}", db.height);
        assert!(db.weight > 180 && db.weight <=234, "Expected between 180..234, got {}", db.weight);
        assert!(db.skin_tone.len() > 0, "Skin tone is empty" );
        assert!(db.hair_color.len() > 0, "Hair color is empty" );
        assert!(db.hair_type.len() > 0, "Hair type is empty" );
        assert!(db.eye_color.len() > 0, "Eye color is empty" );
        let result = db.abilities.iter().any(|b| b.ability_name.contains(&String::from("damage resistance")));
        assert_eq!(result, true);
    }
    #[test]
    fn test_blue_dragonborn() {
        let prefs = CharacterPreferences {
            ancestry:  String::from("blue dragonborn"),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&prefs);
        assert_eq!(db.name, String::from("blue dragonborn"));
        assert_eq!(db.parent_name, String::from("dragonborn"));
    }
    #[test]
    fn test_random_dragonborn() {

        let prefs = CharacterPreferences {
            ancestry:  String::from("dragonborn"),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&prefs);
        assert_eq!(db.parent_name, String::from("dragonborn"));
        assert!(db.name.len() > 12, "Expecting anything with dragonborn: found {}", db.name);
    }
    #[test]
    fn test_default_ancestry() {

        let prefs = CharacterPreferences {
            ancestry:  String::from("bornfree"),
            ..CharacterPreferences::default()
        };
        let db = AncestralTraits::new(&prefs);
        assert_eq!(db.parent_name, String::from("dragonborn"));
        assert!(db.name.len() > 12, "Expecting anything with dragonborn: found {}", db.name);
    }

}
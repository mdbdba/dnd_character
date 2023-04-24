
/*
Dwarf
The origins of dwarves are shrouded in myth, with some saying that their ancestors were fashioned
from the very stone itself. Dwarven culture reflects this tradition, often celebrating practices
related to the working of stone and metal.


Ancestral Traits
Your dwarf character has an assortment of inborn abilities, part and parcel of dwarven biology.

Age. Dwarves mature at the same rate as humans, but they’re considered young until they reach the
age of 50. On average, they live about 350 years.

Size. Dwarves stand between 4 and 5 feet tall and average about 150 pounds. Your size is Medium.

Speed. Your base walking speed is 25 feet. Your speed is not reduced by wearing heavy armor.

Darkvision. Accustomed to life underground, you have superior vision in dark and dim conditions.
You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if
it were dim light. You can’t discern color in darkness, only shades of gray.

Dwarven Resilience. You have advantage on saving throws against poison, and you have resistance
against poison damage, most likely a feature of you ancestors’ diet.

Dwarven Toughness. Your hit point maximum increases by 1, and it increases by 1 every time you
gain a level, due in large part to the long history of difficult labor required to survive
underground for generations.
*/


use std::collections::HashMap;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use crate::ancestry::{AncestralTraits, BaseAncestralTraits, BaseCulturalTraits, CharacterAbility, CulturalTraits, get_i16_some_value, get_lc_some_value, get_random_string, LanguageTraits, set_ability_bonuses, set_alignments};
use crate::character::{CharacterPreferences, DamageType, get_tool_proficiency, get_weapon_proficiency, MechanicCategory, MechanicLevel, Vantage};

#[derive(Debug)]
pub enum SubClass {
    Hill,
}
impl Distribution<SubClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SubClass {
        match rng.gen_range(0..1) {
            0 => SubClass::Hill,
            _ => SubClass::Hill,
        }
    }
}

fn confirm_sub_class(src: String) -> SubClass {
    return match src.to_lowercase().as_str() {
        "hill dwarf"  => SubClass::Hill ,
        _ => {
            let target:SubClass  = rand::random();
            target
        },
    }
}

pub fn new_dwarven_ancestry(prefs: &mut CharacterPreferences) -> AncestralTraits {
    let parent_name = "dwarf".to_string();
    let sub_class = confirm_sub_class(prefs.ancestry.clone());
    let name: String;
    match sub_class {
        SubClass::Hill => {
            name = "hill dwarf".to_string();
        }
    }

    let mut ancestry_abilities: HashMap<String, HashMap<String, CharacterAbility>> = HashMap::new();

    let mut environmental: HashMap<String, CharacterAbility> = HashMap::new();

    let ability1 = BaseAncestralTraits::get_darkvision(None);

    environmental.insert("darkvision".to_string(), ability1);

    /*
    Dwarven Resilience. You have advantage on saving throws against poison, and you have resistance
    against poison damage, most likely a feature of you ancestors’ diet.
    */
    let mut saving_throws: HashMap<String, CharacterAbility> = HashMap::new();
    let ability1 = BaseAncestralTraits::get_saving_throw(
        "dwarven resilience".to_string(),
        Vantage::Advantage,
        DamageType::Poison);

    saving_throws.insert("poison".to_string(), ability1);

    let resistances =
        BaseAncestralTraits::add_resistances(vec!{"poison".to_string()});


    /*
        Dwarven Toughness. Your hit point maximum increases by 1, and it increases by 1 every time you
    gain a level, due in large part to the long history of difficult labor required to survive
    underground for generations.
    */
    let mut hit_points: HashMap<String, CharacterAbility> = HashMap::new();
    let ability1 = CharacterAbility{
        ability_name: "dwarven toughness".to_string(),
        category: "help".to_string(),
        specific_effect: vec! {"hit_points".to_string()},
        range: vec!{"all".to_string()},
        // mechanic: vec!{"1 hit point".to_string()},
        mechanic: vec!{
            MechanicLevel {
                level: 1,
                roll_multiplier: None,
                roll_die: None,
                adjustment: Some(1),
                category: MechanicCategory::HitPoints,
            }
        },
        availability: vec!{"per level".to_string()}
    };
    hit_points.insert("poison".to_string(), ability1);

    ancestry_abilities.insert("environmental".to_string(), environmental);
    ancestry_abilities.insert("saving_throws".to_string(), saving_throws);
    ancestry_abilities.insert("resistances".to_string(), resistances);
    ancestry_abilities.insert("hit_points".to_string(), hit_points);


    let base_values = BaseAncestralTraits {
        name,
        parent_name,
        maturity_age: 50,
        avg_max_age: 350,
        base_walking_speed: 25,
        height_min_inches: 48,
        height_modifier_multiplier: 2,
        height_modifier_die: 6,
        height_modifier_adj: 0,
        weight_min_pounds: 134,
        weight_modifier_multiplier: 4,
        weight_modifier_die: 12,
        weight_modifier_adj: 0,
        base_size: "medium".to_string(),
        skin_tones: vec! {
            "tan".to_string(),
            "brown".to_string(),
            "beige".to_string(),
            "black".to_string()
        },
        hair_colors: vec! {
            "black".to_string(),
            "brown".to_string(),
            "auburn".to_string(),
            "red".to_string(),
            "grey".to_string()
        },
        hair_types: vec! {
            "curly".to_string(),
            "wavy".to_string(),
            "straight".to_string(),
        },
        eye_colors: vec! {
            "brown".to_string(),
            "black".to_string(),
            "grey".to_string(),
            "green".to_string(),
            "bloodshot".to_string()
        },
        abilities: ancestry_abilities,
        source_material: "SRD".to_string(),
        source_credit_url: {
            "https://www.dndbeyond.com/attachments/39j2li89/SRD5.1-CCBY4.0_License_live%20links.pdf".to_string()
        },
        source_credit_comment: "As of 2023/03/25".to_string()
    };

    let base_size = base_values.get_base_size();
    let base_walking_speed = base_values.get_base_walking_speed();

    AncestralTraits::combiner(prefs, &base_values);

    let just_in_case_age = base_values.get_age();
    let just_in_case_height = base_values.get_height();
    let just_in_case_weight = base_values.get_weight();
    AncestralTraits {
        name: base_values.name,
        parent_name: base_values.parent_name,
        age: get_i16_some_value(prefs.age, just_in_case_age),
        base_size,
        base_walking_speed,
        height: get_i16_some_value(prefs.height, just_in_case_height),
        weight: get_i16_some_value(prefs.weight, just_in_case_weight),
        skin_tone: get_lc_some_value(prefs.skin_tone.clone(), "wonderful".to_string()),
        hair_color: get_lc_some_value(prefs.hair_color.clone(), "wonderful".to_string()),
        hair_type: get_lc_some_value(prefs.hair_type.clone(), "wonderful".to_string()),
        eye_color: get_lc_some_value(prefs.eye_color.clone(), "wonderful".to_string()),
        abilities: base_values.abilities,
        source_material: base_values.source_material,
        source_credit_url: base_values.source_credit_url,
        source_credit_comment: base_values.source_credit_comment,
    }
}

    /*
Cultural Traits
Hill Dwarf
Characters who grows up in a hill dwarven community take on several distinctive cultural traits,
in part due to their long history living underground and valuing of skill with traditional dwarven
weapons and crafts. Dwarven culture values perseverance in labor and the maintenance of their
traditions. Further, respect is shown for their wise elders.

Ability Score Increase. Your Constitution score increases by 2,
and your Wisdom score increases by 1.

Alignment. Dwarven society is well-ordered, with strict laws and customs governing behavior. As a
result, the culture tends to promote lawful values, with a strong sense of fair play and a belief
that everyone deserves to share in the benefits of a just order.

Dwarven Combat Training. You have proficiency with the battleaxe, handaxe, light hammer,
and warhammer.

Tool Proficiency. You gain proficiency with the artisan’s tools of your choice: smith’s tools,
brewer’s supplies, mechanic’s tools, or mason’s tools.

Stonecunning. Whenever you make an Intelligence (History) check related to the origin of stonework,
you are considered proficient in the History skill and add double your proficiency bonus to the
check, instead of your normal proficiency bonus.

Languages. You can speak, read, and write Common and Dwarvish. Dwarvish is full of hard consonants
and guttural sounds, and those characteristics spill over into whatever other language you
might speak.

*/

pub fn new_dwarven_culture(prefs: &mut CharacterPreferences) -> CulturalTraits {

    let parent_name = "dwarf".to_string();
    let sub_class = confirm_sub_class(prefs.ancestry.clone());
    let combined_name = format!("{:?} {}", sub_class, parent_name.clone()).to_lowercase();
    let mut cultural_abilities: HashMap<String, HashMap<String, CharacterAbility>> = HashMap::new();

    let languages = BaseCulturalTraits::add_languages(
        vec![
            LanguageTraits {
                name: "common".to_string(),
                can_read: true,
                can_speak: true,
                can_write: true,
            },
            LanguageTraits {
                name: "dwarvish".to_string(),
                can_read: true,
                can_speak: true,
                can_write: true,
            },
        ]);

    /*
    Dwarven Combat Training. You have proficiency with the battleaxe, handaxe, light hammer,
    and warhammer.

        Tool Proficiency. You gain proficiency with the artisan’s tools of your choice: smith’s tools,
    brewer’s supplies, mechanic’s tools, or mason’s tools.
     */

    let mut proficiencies: HashMap<String, CharacterAbility> = HashMap::new();
    proficiencies.insert("battleaxe".to_string(),
                         get_weapon_proficiency("dwarven combat training".to_string(),
                                                "battleaxe".to_string()));
    proficiencies.insert("handaxe".to_string(),
                         get_weapon_proficiency("dwarven combat training".to_string(),
                                                "handaxe".to_string()));
    proficiencies.insert("light hammer".to_string(),
                     get_weapon_proficiency("dwarven combat training".to_string(),
                                            "light hammer".to_string()));
    proficiencies.insert("war hammer".to_string(),
                     get_weapon_proficiency("dwarven combat training".to_string(),
                                            "war hammer".to_string()));


    let tool_proficiency = get_random_string(vec!{
        "smith’s tools".to_string(),
        "brewer’s supplies".to_string(),
        "mechanic’s tools".to_string(),
        "mason’s tools".to_string()}, "smith's tools".to_string());

    proficiencies.insert(tool_proficiency.clone(),
                         get_tool_proficiency("tool proficiency".to_string(),
                                              tool_proficiency.clone()));

    cultural_abilities.insert("languages".to_string(), languages);
    cultural_abilities.insert("proficiencies".to_string(), proficiencies);

    let base_values = BaseCulturalTraits {
        alignments: set_alignments(45,5,5,15,5,5,10,5,5),
        tool_proficiency_choices: Some(vec!{
        "smith’s tools".to_string(),
        "brewer’s supplies".to_string(),
        "mechanic’s tools".to_string(),
        "mason’s tools".to_string()}),
        ability_bonuses: set_ability_bonuses(0,0,2,0,1,0),
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
            "https://www.dndbeyond.com/attachments/39j2li89/SRD5.1-CCBY4.0_License_live%20links.pdf".to_string()
        },
        source_credit_comment: {"As of 2023/03/25".to_string() }
    }
}

#[cfg(test)]
mod tests {
use crate::ancestry::{AncestralTraits, CulturalTraits};
use crate::character::CharacterPreferences;


#[test]
fn test_hill_dwarf_ancestry() {
let mut prefs = CharacterPreferences {
ancestry: "hill dwarf".to_string(),
..CharacterPreferences::default()
};
let db = AncestralTraits::new(&mut prefs);
assert_eq!(db.name, "hill dwarf".to_string());
assert_eq!(db.parent_name, "dwarf".to_string());
assert!(db.age >= 50 && db.age <= 400, "Expected 50..400, got {}", db.age);
assert_eq!(db.base_walking_speed, 25);
assert!(db.height > 48 && db.height <= 60, "Expected 48..60, got {}", db.height);
assert!(db.weight > 138 && db.weight <= 182, "Expected 138..182, got {}", db.weight);
assert!(db.skin_tone.len() > 0, "Skin tone is empty");
assert!(db.hair_color.len() > 0, "Hair color is empty");
assert!(db.hair_type.len() > 0, "Hair type is empty");
assert!(db.eye_color.len() > 0, "Eye color is empty");
let result = db.abilities.get("resistances")
.and_then(|b| b.get("poison")).unwrap();

assert_eq!(result.ability_name, "damage resistance".to_string());
}

#[test]
fn test_hill_dwarf_culture() {
let mut prefs = CharacterPreferences {
ancestry:  "hill dwarf".to_string(),
alignment: Some("chaotic neutral".to_string()),
..CharacterPreferences::default()
};
let db = CulturalTraits::new(&mut prefs);
assert_eq!(db.name, "hill dwarf".to_string());
assert_eq!(db.parent_name, "dwarf".to_string());
assert_eq!(db.alignment, "chaotic neutral".to_string());
assert_eq!(db.abilities.len(), 2);

let common = &db.abilities["languages"]["common"];
assert_eq!(common.ability_name,
"ancestral language".to_string());
assert_eq!(common.range.len(), 3);
assert!(common.range.contains(&"read".to_string()), "read value was not found");
let dwarvish = &db.abilities["languages"]["dwarvish"];
assert_eq!(dwarvish.ability_name,
"ancestral language".to_string());
assert_eq!(dwarvish.range.len(), 3);
assert!(dwarvish.range.contains(&"read".to_string()), "read value was not found");
}
}

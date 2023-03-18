use std::collections::HashMap;
use std::num::NonZeroI8;
use crate::modifier::Modifier::DropLowest;
use crate::roll;
use crate::roll::roll_die;

pub fn get_ability_score_modifier(score: i8) -> i8 {
    let return_value: i8;
    match score {
        1       => return_value = -5,
        2 | 3   => return_value = -4,
        4 | 5   => return_value = -3,
        6 | 7   => return_value = -2,
        8 | 9   => return_value = -1,
        10 | 11 => return_value = 0,
        12 | 13 => return_value = 1,
        14 | 15 => return_value = 2,
        16 | 17 => return_value = 3,
        18 | 19 => return_value = 4,
        20 | 21 => return_value = 5,
        22 | 23 => return_value = 6,
        24 | 25 => return_value = 7,
        26 | 27 => return_value = 8,
        28 | 29 => return_value = 9,
        30      => return_value = 10,
        _ => return_value = 0,
    }
    return_value
}

pub struct Ability {
    description: String,
    value: roll::Roll
}

fn roll_ability(description: String) -> Ability {
    let six = NonZeroI8::new(6).unwrap();
    let four = NonZeroI8::new(4).unwrap();

    Ability {
        description,
        value: roll_die(six, four, DropLowest(1), 0)
    }
}

pub fn roll_ability_scores() -> HashMap<String, Ability> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Strength"),
                  roll_ability(String::from("approximates physical power")));
    scores.insert(String::from("Dexterity"),
                  roll_ability(String::from("approximates agility")));
    scores.insert(String::from("Constitution"),
                  roll_ability(String::from("approximates endurance")));
    scores.insert(String::from("Intelligence"),
                  roll_ability(String::from("approximates reasoning and memory")));
    scores.insert(String::from("Wisdom"),
                  roll_ability(String::from("approximates perception and insight")));
    scores.insert(String::from("Charisma"),
                  roll_ability(String::from("approximates force of personality")));
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength() {
        let scores = roll_ability_scores();
        let ability = scores.get("Strength");
        assert_eq!(ability.unwrap().description, String::from("approximates physical power"));
        assert_eq!(ability.unwrap().value.get_sides(), i8::from(6));
        assert_eq!(ability.unwrap().value.get_rolls(), i8::from(4));
        assert_eq!(ability.unwrap().value.get_adjustment(), i8::from(0));
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_dexterity() {
        let scores = roll_ability_scores();
        let ability = scores.get("Dexterity");
        assert_eq!(ability.unwrap().description, String::from("approximates agility"));
        assert_eq!(ability.unwrap().value.get_sides(), i8::from(6));
        assert_eq!(ability.unwrap().value.get_rolls(), i8::from(4));
        assert_eq!(ability.unwrap().value.get_adjustment(), i8::from(0));
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_constitution() {
        let scores = roll_ability_scores();
        let ability = scores.get("Constitution");
        assert_eq!(ability.unwrap().description, String::from("approximates endurance"));
        assert_eq!(ability.unwrap().value.get_sides(), i8::from(6));
        assert_eq!(ability.unwrap().value.get_rolls(), i8::from(4));
        assert_eq!(ability.unwrap().value.get_adjustment(), i8::from(0));
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_intelligence() {
        let scores = roll_ability_scores();
        let ability = scores.get("Intelligence");
        assert_eq!(ability.unwrap().description, String::from("approximates reasoning and memory"));
        assert_eq!(ability.unwrap().value.get_sides(), i8::from(6));
        assert_eq!(ability.unwrap().value.get_rolls(), i8::from(4));
        assert_eq!(ability.unwrap().value.get_adjustment(), i8::from(0));
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_wisdom() {
        let scores = roll_ability_scores();
        let ability = scores.get("Wisdom");
        assert_eq!(ability.unwrap().description, String::from("approximates perception and insight"));
        assert_eq!(ability.unwrap().value.get_sides(), i8::from(6));
        assert_eq!(ability.unwrap().value.get_rolls(), i8::from(4));
        assert_eq!(ability.unwrap().value.get_adjustment(), i8::from(0));
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_charisma() {
        let scores = roll_ability_scores();
        let ability = scores.get("Charisma");
        assert_eq!(ability.unwrap().description, String::from("approximates force of personality"));
        assert_eq!(ability.unwrap().value.get_sides(), i8::from(6));
        assert_eq!(ability.unwrap().value.get_rolls(), i8::from(4));
        assert_eq!(ability.unwrap().value.get_adjustment(), i8::from(0));
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_get_modifier_for_one() {
        let score_modifier = get_ability_score_modifier(i8::from(1));
        assert_eq!(score_modifier, -5);
    }
    #[test]
    fn test_get_modifier_for_ten() {
        let score_modifier = get_ability_score_modifier(i8::from(10));
        assert_eq!(score_modifier, 0);
    }
    #[test]
    fn test_get_modifier_for_twenty() {
        let score_modifier = get_ability_score_modifier(i8::from(20));
        assert_eq!(score_modifier, 5);
    }
    #[test]
    fn test_get_modifier_for_thirty() {
        let score_modifier = get_ability_score_modifier(i8::from(30));
        assert_eq!(score_modifier, 10);
    }
}
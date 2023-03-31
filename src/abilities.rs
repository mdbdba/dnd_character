use std::collections::HashMap;
use crate::modifier::Modifier::DropLowest;
use crate::roll;
use crate::roll::roll_die;
use std::convert::TryInto;

pub fn get_ability_score_modifier(score: i16) -> i16 {
    let return_value: i16;
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
    let six = 6;
    let four = 4;

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



fn convert_vector_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn sort_ability_scores(scores: Vec<i16>, order_by: [usize; 6]) -> Vec<i16> {
    /* order_by array is positionally ordered highest to lowest, so if [1,3,2,4,5,6] then
       We want ability scores ordered:
         Strength,
         Constitution,
         Dexterity,
         Intelligence,
         Wisdom,
         Charisma

       The scores vector will be ordered lowest to highest.  So, if we reverse the order array, it
       will allow us to pop values directly.
          [6, 5, 4, 2, 3, 1]
          for each value in the order_by array
             return_vector.push(value)
    */
    let mut return_vector: Vec<i16> = vec![];
    let working_array:[i16;6] = convert_vector_to_array(scores);
    for spec in order_by {
        let derived = spec - 1;
        let stretch: usize = derived.into();
        return_vector.push(working_array[stretch]);
    }

    return_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength() {
        let scores = roll_ability_scores();
        let ability = scores.get("Strength");
        assert_eq!(ability.unwrap().description, "approximates physical power");
        assert_eq!(ability.unwrap().value.get_sides(), 6);
        assert_eq!(ability.unwrap().value.get_rolls(), 4);
        assert_eq!(ability.unwrap().value.get_adjustment(), 0);
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_dexterity() {
        let scores = roll_ability_scores();
        let ability = scores.get("Dexterity");
        assert_eq!(ability.unwrap().description, "approximates agility");
        assert_eq!(ability.unwrap().value.get_sides(), 6);
        assert_eq!(ability.unwrap().value.get_rolls(), 4);
        assert_eq!(ability.unwrap().value.get_adjustment(), 0);
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_constitution() {
        let scores = roll_ability_scores();
        let ability = scores.get("Constitution");
        assert_eq!(ability.unwrap().description, "approximates endurance");
        assert_eq!(ability.unwrap().value.get_sides(), 6);
        assert_eq!(ability.unwrap().value.get_rolls(), 4);
        assert_eq!(ability.unwrap().value.get_adjustment(), 0);
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_intelligence() {
        let scores = roll_ability_scores();
        let ability = scores.get("Intelligence");
        assert_eq!(ability.unwrap().description, "approximates reasoning and memory");
        assert_eq!(ability.unwrap().value.get_sides(), 6);
        assert_eq!(ability.unwrap().value.get_rolls(), 4);
        assert_eq!(ability.unwrap().value.get_adjustment(), 0);
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_wisdom() {
        let scores = roll_ability_scores();
        let ability = scores.get("Wisdom");
        assert_eq!(ability.unwrap().description, "approximates perception and insight");
        assert_eq!(ability.unwrap().value.get_sides(), 6);
        assert_eq!(ability.unwrap().value.get_rolls(), 4);
        assert_eq!(ability.unwrap().value.get_adjustment(), 0);
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_charisma() {
        let scores = roll_ability_scores();
        let ability = scores.get("Charisma");
        assert_eq!(ability.unwrap().description, "approximates force of personality");
        assert_eq!(ability.unwrap().value.get_sides(), 6);
        assert_eq!(ability.unwrap().value.get_rolls(), 4);
        assert_eq!(ability.unwrap().value.get_adjustment(), 0);
        assert_eq!(ability.unwrap().value.get_modify(), "dl1");
    }
    #[test]
    fn test_get_modifier_for_one() {
        let score_modifier = get_ability_score_modifier(1);
        assert_eq!(score_modifier, -5);
    }
    #[test]
    fn test_get_modifier_for_ten() {
        let score_modifier = get_ability_score_modifier(10);
        assert_eq!(score_modifier, 0);
    }
    #[test]
    fn test_get_modifier_for_twenty() {
        let score_modifier = get_ability_score_modifier(20);
        assert_eq!(score_modifier, 5);
    }
    #[test]
    fn test_get_modifier_for_thirty() {
        let score_modifier = get_ability_score_modifier(30);
        assert_eq!(score_modifier, 10);
    }
    #[test]
    fn test_sort_ability_scores() {
        let result = sort_ability_scores(vec![9,10,11,12,13,14], [1,2,3,4,5,6]);
        assert_eq!(result, vec![9,10,11,12,13,14])
    }
    #[test]
    fn test_sort_ability_scores_desc() {
        let result = sort_ability_scores(vec![9,10,11,12,13,14], [6,5,4,3,2,1]);
        assert_eq!(result, vec![14,13,12,11,10,9])
    }
}
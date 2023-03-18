use rand::prelude::*;
use std::num::NonZeroI8;
use crate::modifier::Modifier;


#[derive(Debug, Clone)]
pub struct Roll {
    sides: NonZeroI8,
    rolls: NonZeroI8,
    modify: Modifier<i8>,
    adjustment: i8,
    rawrolls: Vec<i8>,
    performed: Vec<i8>,
    subtotal: i8,
    total: i8
}

impl Roll {
    pub fn get_sides(&self) -> i8 {
        i8::from(self.sides)
    }
    pub fn get_rolls(&self) -> i8 {
        i8::from(self.rolls)
    }
    pub fn get_modify(&self) -> String {
        self.modify.to_string()
    }
    pub fn get_adjustment(&self) -> i8 {
        self.adjustment
    }
    pub fn get_rawrolls(&self) -> Vec<i8> {
        self.rawrolls.clone()
    }
    pub fn get_performed(&self) -> Vec<i8> {
        self.performed.clone()
    }
    pub fn get_subtotal(&self) -> i8 {
        self.subtotal
    }
    pub fn get_total(&self) -> i8 {
        self.total
    }
}

pub fn roll_die (
    sides: NonZeroI8,
    rolls: NonZeroI8,
    modify: Modifier<i8>,
    adjustment: i8) -> Roll
{
    let mut amounts= Vec::new();
    let max_range: i8 = i8::from(sides) + 1;
    for _ in 0..i8::from(rolls) {
        let result = thread_rng().gen_range(1..max_range);
        amounts.push(result);
    }

    amounts.sort_unstable();
    let pre_amounts = amounts.clone();

    match modify {
        Modifier::KeepLowest(i) => {
            amounts.truncate(i as usize);
        }
        Modifier::KeepHighest(i) => {
            amounts.reverse();
            amounts.truncate(i as usize);
        }
        Modifier::DropLowest(i) => {
            amounts.reverse();
            amounts.truncate(amounts.len() - i.min(amounts.len() as i8) as usize);
        }
        Modifier::DropHighest(i) => {
            amounts.truncate(amounts.len() - i.min(amounts.len() as i8) as usize);
        }
        Modifier::None => {}
    }

    if amounts.is_empty() {
        amounts.push(0);
    }
    amounts.sort_unstable();

    let tmp_total = amounts.iter().sum::<i8>() as i8;
    let total = tmp_total + adjustment;

    Roll {
        sides,
        rolls,
        modify,
        adjustment,
        rawrolls: pre_amounts.clone(),
        performed: amounts.clone(),
        subtotal: tmp_total,
        total,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kl() {
        let roll = roll_die(
            NonZeroI8::new(6).unwrap(),
            NonZeroI8::new(6).unwrap(),
            Modifier::KeepLowest(3),
            0
        );

        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), roll.get_performed().len());
        assert_eq!(roll.performed.len(), 3);
        assert_eq!(roll.modify.to_string(), roll.get_modify());
        assert_eq!(roll.modify.to_string(), "kl3");
    }
    #[test]
    fn test_kh() {
        let roll = roll_die(
            NonZeroI8::new(6).unwrap(),
            NonZeroI8::new(4).unwrap(),
            Modifier::KeepHighest(3),
            0
        );

        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), 3);
        assert_eq!(roll.modify.to_string(), "kh3");
    }
    #[test]
    fn test_dl() {
        let roll = roll_die(
            NonZeroI8::new(20).unwrap(),
            NonZeroI8::new(2).unwrap(),
            Modifier::DropLowest(1),
            0
        );

        // println!("{}", roll.modify);
        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), 1);
        assert_eq!(roll.modify.to_string(), "dl1");
    }
    #[test]
    fn test_dh() {
        let roll = roll_die(
            NonZeroI8::new(20).unwrap(),
            NonZeroI8::new(2).unwrap(),
            Modifier::DropHighest(1),
            0
        );

        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), 1);
        assert_eq!(roll.modify.to_string(), "dh1");
    }
    #[test]
    fn test_adj() {
        let roll = roll_die(
            NonZeroI8::new(6).unwrap(),
            NonZeroI8::new(1).unwrap(),
            Modifier::None,
            3
        );

        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.subtotal);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), 1);
        assert_eq!(roll.total, (roll.subtotal + 3 ))
    }
    #[test]
    fn test_gets() {
        let roll = roll_die(
            NonZeroI8::new(6).unwrap(),
            NonZeroI8::new(1).unwrap(),
            Modifier::None,
            3
        );
        assert_eq!(roll.get_sides(), i8::from(6));
        assert_eq!(roll.get_rolls(), i8::from(1));
        assert_eq!(roll.get_adjustment(), i8::from(3));
        assert_eq!(roll.get_modify(), "");
        println!("{:?}", roll.get_rawrolls());
        println!("{:?}", roll.get_performed());
        println!("{}", roll.get_subtotal());
        println!("{}", roll.get_total());
    }
}




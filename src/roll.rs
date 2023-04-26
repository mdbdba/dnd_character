use crate::modifier::Modifier;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Roll {
    sides: i16,
    rolls: i16,
    modify: Modifier<i16>,
    adjustment: i16,
    rawrolls: Vec<i16>,
    performed: Vec<i16>,
    subtotal: i16,
    total: i16,
}

impl Roll {
    pub fn get_sides(&self) -> i16 {
        self.sides
    }
    pub fn get_rolls(&self) -> i16 {
        self.rolls
    }
    pub fn get_modify(&self) -> String {
        self.modify.to_string()
    }
    pub fn get_adjustment(&self) -> i16 {
        self.adjustment
    }
    pub fn get_rawrolls(&self) -> Vec<i16> {
        self.rawrolls.clone()
    }
    pub fn get_performed(&self) -> Vec<i16> {
        self.performed.clone()
    }
    pub fn get_subtotal(&self) -> i16 {
        self.subtotal
    }
    pub fn get_total(&self) -> i16 {
        self.total
    }
}

pub fn roll_die(sides: i16, rolls: i16, modify: Modifier<i16>, adjustment: i16) -> Roll {
    let mut amounts = Vec::new();
    let max_range = sides + 1;
    for _ in 0..rolls {
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
            amounts.truncate(amounts.len() - i.min(amounts.len() as i16) as usize);
        }
        Modifier::DropHighest(i) => {
            amounts.truncate(amounts.len() - i.min(amounts.len() as i16) as usize);
        }
        Modifier::None => {}
    }

    if amounts.is_empty() {
        amounts.push(0);
    }
    amounts.sort_unstable();

    let tmp_total = amounts.iter().sum::<i16>();
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
        let roll = roll_die(6, 6, Modifier::KeepLowest(3), 0);

        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), roll.get_performed().len());
        assert_eq!(roll.performed.len(), 3);
        let t_total = roll.get_total();
        assert!(t_total >= 3, "Expected total >= 3, got {}", t_total);
        assert!(t_total <= 18, "Expected total <= 18, got {}", t_total);
        assert_eq!(roll.modify.to_string(), roll.get_modify());
        assert_eq!(roll.modify.to_string(), "kl3");
    }
    #[test]
    fn test_kh() {
        let roll = roll_die(6, 4, Modifier::KeepHighest(3), 0);

        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), 3);
        assert_eq!(roll.modify.to_string(), "kh3");
    }
    #[test]
    fn test_dl() {
        let roll = roll_die(20, 2, Modifier::DropLowest(1), 0);

        // println!("{}", roll.modify);
        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), 1);
        assert_eq!(roll.modify.to_string(), "dl1");
    }
    #[test]
    fn test_dh() {
        let roll = roll_die(20, 2, Modifier::DropHighest(1), 0);

        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), 1);
        assert_eq!(roll.modify.to_string(), "dh1");
    }
    #[test]
    fn test_adj() {
        let roll = roll_die(6, 1, Modifier::None, 3);

        // println!("{:?}", roll.rawrolls);
        // println!("{:?}", roll.performed);
        // println!("{}", roll.subtotal);
        // println!("{}", roll.total);
        assert_eq!(roll.performed.len(), 1);
        assert_eq!(roll.total, (roll.subtotal + 3))
    }
    #[test]
    fn test_gets() {
        let roll = roll_die(6, 1, Modifier::None, 3);
        assert_eq!(roll.get_sides(), 6);
        assert_eq!(roll.get_rolls(), 1);
        assert_eq!(roll.get_adjustment(), 3);
        assert_eq!(roll.get_modify(), "");
        println!("{:?}", roll.get_rawrolls());
        println!("{:?}", roll.get_performed());
        println!("{}", roll.get_subtotal());
        println!("{}", roll.get_total());
    }
}

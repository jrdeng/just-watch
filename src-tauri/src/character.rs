use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Character {
    pub pos_in_grid: i32, // this is not the index in memory
    pub name: String,
    pub lv: i32,
    pub hp: i32,
    pub hp_max: i32,
    pub mp: i32,
    pub mp_max: i32,
}

impl Character {
    pub fn default() -> Self {
        Self {
            pos_in_grid: -1,
            name: "".to_owned(),
            lv: 0,
            hp: 0,
            hp_max: 0,
            mp: 0,
            mp_max: 0,
        }
    }

    pub fn from(character_info: &str) -> Self {
        // index|name|cuid|?|lv|hp|hp_max|mp|mp_max|cid?|?|?
        let mut character = Character::default();
        let character_info_vec: Vec<&str> = character_info.split('|').collect();
        if character_info_vec.len() < 12 {
            println!("Character info is not enough");
            return character;
        }
        let index = i32::from_str_radix(character_info_vec[0], 16).unwrap();
        // transform index to pos in grid
        character.pos_in_grid = match index {
            14 => 0,
            12 => 1,
            10 => 2,
            11 => 3,
            13 => 4,
            19 => 5,
            17 => 6,
            15 => 7,
            16 => 8,
            18 => 9,
            9 => 10,
            7 => 11,
            5 => 12,
            6 => 13,
            8 => 14,
            4 => 15,
            2 => 16,
            0 => 17,
            1 => 18,
            3 => 19,
            _ => -1,
        };
        character.name = character_info_vec[1].to_owned();
        character.lv = i32::from_str_radix(character_info_vec[4], 16).unwrap();
        character.hp = i32::from_str_radix(character_info_vec[5], 16).unwrap();
        character.hp_max = i32::from_str_radix(character_info_vec[6], 16).unwrap();
        character.mp = i32::from_str_radix(character_info_vec[7], 16).unwrap();
        character.mp_max = i32::from_str_radix(character_info_vec[8], 16).unwrap();
        character
    }
}

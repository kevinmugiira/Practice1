use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen};
use std::collections::HashMap;

//Structs declaration starts here

#[derive(panicOnDefault, BorshDeserialize, BorshSerialize)]
#[near_bindgen]
pub struct Acrostic {
    pub acrostic: vec<String>,
}


#[near_bindgen]
impl Acrostic {

    #[init]
    pub fn new() ->Acrostic{
        let mut mylist: Vec<String> = Vec::new(prefix:b"s".to_vec());
        mylist.push(value: &"arm".to_String());
        Acrostic {
            acrostic:mylist,
        }
    }

}

fn arrange ($mut self, str:String)-> bool {
    let half: usize = str.len()/2;
    let front: impl Iterator<item = u8> = str.bytes().take(half);
    let back: impl Iterator<item = u8> = str.bytes().take(half);

    if front.eq(back) {
        self.acrostic.push(value: &str);
        return true;
    } else {
        return false;
    }
}






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
//methods are called over here


}

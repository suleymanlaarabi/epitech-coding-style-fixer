use crate::prelude::Rule;

use super::fix_c_a3::fix_c_a3;
use super::fix_c_g1::fix_c_g1;
use super::fix_c_g7::fix_c_g7;
use super::fix_c_v3::fix_c_v3;
use super::fix_c_l6::fix_c_l6;
use super::fix_c_l5::fix_c_l5;
use super::fix_c_l3::fix_c_l3;
use super::fix_c_g8::fix_c_g8;
use super::fix_c_g3::fix_c_g3;

pub fn get_rules() -> Vec<Rule> {
    let rules = vec![
        Rule::new("C-G1", "This is a rule", fix_c_g1),
        Rule::new("C-G7", "This is a rule", fix_c_g7),
        Rule::new("C-A3", "This is a rule", fix_c_a3),
        Rule::new("C-V3", "This is a rule", fix_c_v3),
        Rule::new("C-L6", "This is a rule", fix_c_l6),
        Rule::new("C-L5", "This is a rule", fix_c_l5),
        Rule::new("C-L3", "This is a rule", fix_c_l3),
        Rule::new("C-G8", "This is a rule", fix_c_g8),
        Rule::new("C-G3", "This is a rule", fix_c_g3),
    ];
    rules
}

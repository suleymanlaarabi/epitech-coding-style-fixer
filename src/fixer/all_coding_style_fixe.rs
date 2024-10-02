use crate::prelude::Rule;

pub fn get_rules() -> Vec<Rule> {
    let rules = vec![
        Rule::new("C-G1", super::fix_c_g1::fix_c_g1),
        Rule::new("C-G7", super::fix_c_g7::fix_c_g7),
        Rule::new("C-A3", super::fix_c_a3::fix_c_a3),
        Rule::new("C-V3", super::fix_c_v3::fix_c_v3),
        Rule::new("C-L6", super::fix_c_l6::fix_c_l6),
        Rule::new("C-L5", super::fix_c_l5::fix_c_l5),
        Rule::new("C-L3", super::fix_c_l3::fix_c_l3),
        Rule::new("C-G8", super::fix_c_g8::fix_c_g8),
        Rule::new("C-G3", super::fix_c_g3::fix_c_g3),
    ];
    rules
}

use crate::{project_operation::Project, CodingStyleAlert};

pub fn fix_c_g3(coding_alert: &CodingStyleAlert, file_content: &mut String, _: &Project) -> String {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut level_of_indirection = 0;
    let mut new_content = String::new();

    for (index, line) in lines.iter().enumerate() {
        if index == coding_alert.line - 2 {
            let mut spaces = 0;
            for c in line.chars() {
                if c == ' ' {
                    spaces += 1;
                } else {
                    break;
                }
            }
            level_of_indirection = spaces; 
        }
        if index == coding_alert.line  -1 {
            
            let corrected_line = format!("{:indent$}{}", "", line, indent=level_of_indirection + 4);
            new_content.push_str(&corrected_line);

           
        } else {
            new_content.push_str(line);
        }
        new_content.push('\n');
    }
    println!("level_of_indirection: {}", level_of_indirection);
   
    new_content
}

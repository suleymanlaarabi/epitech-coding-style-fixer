use crate::{project_operation::Project, CodingStyleAlert};

pub fn fix_c_l5(coding_alert: &CodingStyleAlert, file_content: &mut String, _: &Project) -> String {
    let  lines: Vec<&str> = file_content.lines().collect();
    let mut new_content = String::new();
    let mut function_start = 0;
    
    let target_line = coding_alert.line - 1;

    for (index, line) in lines.iter().enumerate().rev() {
        if index <= target_line && line.contains("{") {
            function_start = index + 1;
            break;
        }
    }
    

    for (index, line) in lines.iter().enumerate() {
        if index == function_start {
            new_content.push_str(lines[target_line]);
            new_content.push('\n');
        }
        if index == target_line {
            continue;
        }
        new_content.push_str(line);
        new_content.push('\n');
    }
    
    
    new_content
}

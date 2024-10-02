
use crate::{project_operation::Project, CodingStyleAlert};

pub fn fix_c_g8(_: &CodingStyleAlert, file_content: &mut String, _: &Project) -> String {    
    let mut new_content = String::new();
    let lines: Vec<&str> = file_content.lines().collect();
    let mut start = 0;
    let mut end = lines.len();
    for (index, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            start = index + 1;
        } else {
            break;
        }
    }
    for (index, line) in lines.iter().enumerate().rev() {
        if line.trim().is_empty() {
            end = index;
        } else {
            break;
        }
    }
    for (index, line) in lines.iter().enumerate() {
        if index >= start && index < end {
            new_content.push_str(line);
            // is the last line
            if index != end - 1 {
                new_content.push('\n');
                break;
            }
        }
    }
    new_content
}

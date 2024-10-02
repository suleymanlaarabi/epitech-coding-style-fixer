use crate::{project_operation::Project, CodingStyleAlert};

pub fn fix_c_a3(_: &CodingStyleAlert, file_content: &mut String, _: &Project) -> String {
    let new_content = if file_content.ends_with('\n') {
        file_content.to_string()
    } else {
        format!("{}\n", file_content)
    };

    new_content
}

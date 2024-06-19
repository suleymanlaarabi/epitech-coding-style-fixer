use crate::{project_operation::Project, CodingStyleAlert};

pub fn fix_c_l6(_: &CodingStyleAlert, file_content: &mut String, _: &Project) -> String {
  let lines: Vec<&str> = file_content.lines().collect();
  let mut new_content = String::new();
  let mut in_function = false;
  let mut variables_declared = false;

  for (_, line) in lines.iter().enumerate() {
      if line.trim().ends_with("{") {
          in_function = true;
          variables_declared = false;
          new_content.push_str(line);
          new_content.push('\n');
          continue;
      }

      if in_function {
          if line.trim().is_empty() {
              continue;
          }

          if line.trim().ends_with("}") {
              in_function = false;
              new_content.push('\n');
              new_content.push_str(line);
              new_content.push('\n');
              continue;
          }

          if !variables_declared && line.contains(";") && line.trim().split_whitespace().collect::<Vec<&str>>().len() == 3 {
              variables_declared = true;
              new_content.push_str(line);
              new_content.push('\n');
              new_content.push('\n');
              continue;
          }
      }

      new_content.push_str(line);
      new_content.push('\n');
  }

  new_content
}

use clap::Parser;
use coding_style_fixer::fix_coding_style_issues;
use project_operation::Project;
mod all_coding_style_fixe;
mod coding_checker;
mod coding_style_fixer;
mod docker_operation;
mod error;
mod prelude;
mod project_operation;
mod utility_functions;

use crate::prelude::*;
use coding_checker::coding_checker;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "Epitech Coding Style Checker and Fixer",
    version = "1.0",
    author = "Laarabi Suleyman",
    about = "Vérifie et corrige le style de code en respectant les règles de l'Epitech Coding Style."
)]

struct Args {
    #[arg(short, long, default_value = ".", help = "Dossier de livraison")]
    delivery_dir: String,
    #[arg(short, long, default_value = "reports", help = "Dossier de rapports")]
    reports_dir: String,
    #[arg(
        short,
        long,
        default_value = "false",
        help = "initialise un nouveau projet"
    )]
    init: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let delivery_dir = PathBuf::from(&args.delivery_dir).canonicalize()?;
    let reports_dir = PathBuf::from(&args.reports_dir).canonicalize()?;
    let is_init = args.init;

    if !delivery_dir.exists() || !reports_dir.exists() {
        eprintln!("Error: One or both directories do not exist.");
        return Ok(());
    }

    let current_dir = std::env::current_dir()?;

    let project: Option<Project> = match is_init {
        true => {
            let project = Project::from_dir(&current_dir)?;
            project.save(&current_dir)?;
            Some(project)
        }
        false => match project_operation::project_parse(&current_dir) {
            Ok(project) => Some(project),
            Err(error) => {
                match error {
                    Error::NotFound => {
                        println!("Project file not found.");
                    }
                    Error::InvalidData => {
                        println!("Invalid project json.");
                    }
                    _ => {
                        eprintln!("Error: An unexpected error occurred.");
                    }
                }

                None
            }
        },
    };
    println!("start");

    let file_path = coding_checker(&delivery_dir, &reports_dir, false)?;

    if let Some(project) = project {
        println!("Project name: {}", project.name);
        fix_coding_style_issues(&file_path, &delivery_dir, &project);
    }
    fix_coding_style_issues(&file_path, &delivery_dir, &Project::default());

    Ok(())
}

use clap::Parser;
use coding_style_fixer::fix_coding_style_issues;
use project_operation::{init_new_project, print_project, Project};
mod coding_checker;
mod coding_style_fixer;
mod docker_operation;
mod error;
mod fixer;
mod prelude;
mod project_operation;

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
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Affiche les informations du projet"
    )]
    show_project: bool,
}

fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?;

    let args = Args::parse();
    let is_init = args.init;
    let is_show_project = args.show_project;

    if is_show_project {
        match project_operation::project_parse(&current_dir) {
            Ok(project) => print_project(&project),
            Err(error) => match error {
                Error::NotFound => {
                    println!("Project file not found.");
                }
                Error::InvalidData => {
                    println!("Invalid project json.");
                }
                _ => {
                    eprintln!("Error: An unexpected error occurred.");
                }
            },
        }
        return Ok(());
    }

    let project: Option<Project> = match is_init {
        true => {
            let project = match init_new_project(&current_dir) {
                Ok(project) => project,
                Err(error) => {
                    match error {
                        Error::Io(_) => {
                            println!("Project already exists.");
                        }
                        _ => {
                            eprintln!("Error: An unexpected error occurred.");
                        }
                    }
                    return Ok(());
                }
            };
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

    let delivery_dir = match &project {
        Some(project) => project.delivery_dir.clone(),
        None => PathBuf::from(&args.delivery_dir),
    };

    let reports_dir = match &project {
        Some(project) => project.reports_dir.clone(),
        None => PathBuf::from(&args.reports_dir),
    };

    if !delivery_dir.exists() || !reports_dir.exists() {
        eprintln!("Error: One or both directories do not exist.");
        return Ok(());
    }

    if is_init {
        return Ok(());
    }

    let file_path = coding_checker(&delivery_dir, &reports_dir, false)?;

    if let Some(project) = project {
        println!("Project name: {}", project.name);
        fix_coding_style_issues(&file_path, &delivery_dir, &project);
        return Ok(());
    }

    fix_coding_style_issues(&file_path, &delivery_dir, &Project::default());

    Ok(())
}

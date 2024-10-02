use clap::Parser;
use coding_style_fixer::{fix_coding_style_issues, get_alerts};
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
use std::{io::{self, Read}, path::{Path, PathBuf}};

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
        help = "print les erreur et leur position"
    )]
    linter: bool,
    #[arg(
        short,
        long,
        default_value = "false",
        help = "fixe automatiquement les erreur"
    )]
    correct: bool,
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Affiche les informations du projet"
    )]
    show_project: bool,
    #[arg(
        short,
        long,
        default_value = "",
        help = "Affiche les informations du projet"
    )]
    file: String,
}

fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?;

    let args = Args::parse();
    let is_init = args.init;
    let is_show_project = args.show_project;
    let with_fixe = args.correct;
    let for_linter = args.linter;
    let file = args.file;
 
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
                        Error::Io(err) => {
                            println!("err: {}", err.to_string());
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

    let dir = if for_linter {
        let path = Path::new(&file).to_path_buf();
        path.parent().unwrap().to_path_buf()
    } else {
        delivery_dir
    };

    let alerts_path = coding_checker(&dir, &reports_dir, false)?;

    println!("file: {}, dir: {}", dir.to_str().unwrap(), reports_dir.to_str().unwrap());
    let mut alerts = get_alerts(&alerts_path, &dir);
    if let Some(project) = project {
        println!("Project name: {}", project.name);
        if with_fixe {
            fix_coding_style_issues(&mut alerts, &dir, &project);
        }
        if for_linter {
            
            if file == "" {
                println!("incorect file path");
                return Ok(());
            }
            
            for alert in &mut alerts {
                println!("{}:{}: {}", alert.file, alert.line, alert.rule.id);
            }
        }
        return Ok(());
    }

    Ok(())
}

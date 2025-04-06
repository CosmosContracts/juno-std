//! Based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{ env, fs, path::PathBuf };
use pretty_env_logger::init;
use proto_build::{ code_generator::{ CodeGenerator, CosmosProject }, git };
use serde::Deserialize;
use serde_json::from_str;

#[macro_use]
extern crate log;

#[derive(Debug, Deserialize)]
struct RepoConfig {
    name: String,
    repo: String,
    rev: String,
    dir: String,
    #[serde(default)]
    exclude_mods: Vec<String>,
    is_main: bool,
}

fn get_repo_configs_from_env() -> Vec<RepoConfig> {
    let config_json = env
        ::var("REPO_CONFIG")
        .expect("Missing REPO_CONFIG env variable. Supply a JSON config with repository settings.");

    debug!("REPO_CONFIG content: {}", config_json);

    if config_json.trim().is_empty() {
        panic!(
            "REPO_CONFIG environment variable is empty. Please provide a valid JSON configuration."
        );
    }

    from_str(&config_json).expect("Invalid JSON format in REPO_CONFIG")
}

pub fn generate() {
    let repos = get_repo_configs_from_env();

    // Clean up any previous dependencies directory.
    let deps_dir = PathBuf::from("./dependencies/");
    if deps_dir.exists() {
        fs::remove_dir_all(&deps_dir).unwrap();
    }

    // Clone all repositories as defined in the config.
    for config in &repos {
        info!(
            "Cloning {}: repo: {}, rev: {}, dir: {}",
            config.name,
            config.repo,
            config.rev,
            config.dir
        );
        git::clone_repo(&config.repo, &config.dir, &config.rev);
    }

    let tmp_build_dir: PathBuf = "/tmp/tmp-protobuf/".into();
    let out_dir: PathBuf = "../packages/juno-std/src/types/".into();

    // Dynamically pick the main project based on the `is_main` flag.
    let main_project_config = repos
        .iter()
        .find(|r| r.is_main)
        .expect("No main repository designated in configuration");

    let main_project = CosmosProject {
        name: main_project_config.name.clone(),
        version: main_project_config.rev.clone(),
        project_dir: main_project_config.dir.clone(),
        exclude_mods: main_project_config.exclude_mods.clone(),
    };

    info!("Main project: {}", main_project.name.clone());

    // Use all other repos for additional projects.
    let other_projects = repos
        .iter()
        .filter(|r| !r.is_main)
        .map(|config| proto_build::code_generator::CosmosProject {
            name: config.name.clone(),
            version: config.rev.clone(),
            project_dir: config.dir.clone(),
            exclude_mods: config.exclude_mods.clone(),
        })
        .collect::<Vec<_>>();

    let code_generator = CodeGenerator::new(out_dir, tmp_build_dir, main_project, other_projects);

    info!("Starting generation...");
    code_generator.generate();
    debug!("Cleaning up dependencies directory...");
    fs::remove_dir_all(deps_dir).unwrap();
    info!("Finished generation successfully!");
}

fn main() {
    init();
    generate();
}

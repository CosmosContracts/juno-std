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

fn find_repo_root() -> PathBuf {
    let mut current_dir = env::current_dir().expect("Could not determine current directory");
    while !current_dir.join(".git").exists() {
        // If no parent exists, panic.
        current_dir = current_dir
            .parent()
            .expect("Could not find repository root (.git folder not found)")
            .to_path_buf();
    }
    current_dir
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

    let repo_root = find_repo_root();
    let deps_dir = repo_root.join("dependencies");
    if deps_dir.exists() {
        fs::remove_dir_all(&deps_dir).unwrap();
    }

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
    fs::create_dir_all(&deps_dir).unwrap_or_else(|e| {
        panic!("Failed to create dependencies directory: {}", e);
    });

    let tmp_build_dir = env::temp_dir().join("tmp-protobuf/");
    if !tmp_build_dir.exists() {
        info!("Creating temporary build directory: {:?}", tmp_build_dir);
        fs::create_dir_all(&tmp_build_dir).unwrap_or_else(|e| {
            panic!("Failed to create temporary directory: {}", e);
        });
    }

    info!("Using temporary build directory: {:?}", tmp_build_dir);
    let out_dir: PathBuf = "../packages/juno-std/src/types/".into();

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
    code_generator.generate();

    debug!("Cleaning up dependencies directory...");
    fs::remove_dir_all(deps_dir).unwrap();
    info!("Finished generation successfully!");
}

fn main() {
    init();
    generate();
}

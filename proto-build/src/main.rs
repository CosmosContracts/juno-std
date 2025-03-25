//! Based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{ fs, path::PathBuf };

use proto_build::{ code_generator::{ CodeGenerator, CosmosProject }, git };

/// Repository Links
const COSMOS_SDK_REPO: &str = "https://github.com/cosmos/cosmos-sdk.git";
const JUNO_REPO: &str = "https://github.com/CosmosContracts/juno.git";
const WASMD_REPO: &str = "https://github.com/CosmWasm/wasmd.git";
const COMETBFT_REPO: &str = "https://github.com/cometbft/cometbft.git";
const IBC_GO_REPO: &str = "https://github.com/cosmos/ibc-go.git";
const ICS23_REPO: &str = "https://github.com/cosmos/ics23.git";

/// Commits or tags
const COSMOS_SDK_REV: &str = "v0.50.11";
const JUNO_REV: &str = "v28.0.1";
const WASMD_REV: &str = "v0.54.0";
const COMETBFT_REV: &str = "v0.38.17";
const IBC_GO_REV: &str = "v8.5.3";
const ICS23_REV: &str = "go/v0.11.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.
const COSMOS_SDK_DIR: &str = "../dependencies/cosmos-sdk/";
const JUNO_DIR: &str = "../dependencies/juno/";
const WASMD_DIR: &str = "../dependencies/wasmd/";
const COMETBFT_DIR: &str = "../dependencies/cometbft/";
const IBC_GO_DIR: &str = "../dependencies/ibc-go/";
const ICS23_DIR: &str = "../dependencies/ics23/";

/// A temporary directory for repos storing
const TMP_REPOS_DIR: &str = "./dependencies/";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";
/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../packages/juno-std/src/types/";

pub fn generate() {
    let tmp_repos_dir: PathBuf = TMP_REPOS_DIR.parse().unwrap();
    if tmp_repos_dir.exists() {
        fs::remove_dir_all(tmp_repos_dir.clone()).unwrap();
    }

    git::clone_repo(COSMOS_SDK_REPO, COSMOS_SDK_DIR, COSMOS_SDK_REV);
    git::clone_repo(JUNO_REPO, JUNO_DIR, JUNO_REV);
    git::clone_repo(WASMD_REPO, WASMD_DIR, WASMD_REV);
    git::clone_repo(COMETBFT_REPO, COMETBFT_DIR, COMETBFT_REV);
    git::clone_repo(IBC_GO_REPO, IBC_GO_DIR, IBC_GO_REV);
    git::clone_repo(ICS23_REPO, ICS23_DIR, ICS23_REV);

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
        exclude_mods: vec!["reflection".to_string(), "autocli".to_string()],
    };

    let juno_project = CosmosProject {
        name: "juno".to_string(),
        version: JUNO_REV.to_string(),
        project_dir: JUNO_DIR.to_string(),
        exclude_mods: vec![],
    };

    let wasmd_project = CosmosProject {
        name: "wasmd".to_string(),
        version: WASMD_REV.to_string(),
        project_dir: WASMD_DIR.to_string(),
        exclude_mods: vec![],
    };

    let cometbft_project = CosmosProject {
        name: "tendermint".to_string(),
        version: COMETBFT_REV.to_string(),
        project_dir: COMETBFT_DIR.to_string(),
        exclude_mods: vec![],
    };

    let ibc_project = CosmosProject {
        name: "ibc".to_string(),
        version: IBC_GO_REV.to_string(),
        project_dir: IBC_GO_DIR.to_string(),
        exclude_mods: vec![],
    };

    let ics23_project = CosmosProject {
        name: "ics23".to_string(),
        version: ICS23_REV.to_string(),
        project_dir: ICS23_DIR.to_string(),
        exclude_mods: vec![],
    };

    let code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        juno_project,
        vec![cosmos_project, wasmd_project, cometbft_project, ibc_project, ics23_project]
    );

    code_generator.generate();

    fs::remove_dir_all(tmp_repos_dir.clone()).unwrap();
}

fn main() {
    pretty_env_logger::init();
    generate();
}

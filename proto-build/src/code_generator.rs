use std::fs::{ create_dir_all, remove_dir_all };
use std::path::{ Path, PathBuf };
use std::process::Command;
use std::{ env, fs, io };

extern crate protobuf;
extern crate serde_protobuf;

use log::{ debug, info };
use walkdir::WalkDir;

use crate::{ mod_gen, transform };

#[derive(Clone, Debug)]
pub struct CosmosProject {
    pub name: String,
    pub version: String,
    pub project_dir: String,

    /// determines which modules to exclude from the project
    pub exclude_mods: Vec<String>,
}

pub struct CodeGenerator {
    project: CosmosProject,
    root: PathBuf,
    out_dir: PathBuf,
    tmp_build_dir: PathBuf,
    deps: Vec<CosmosProject>,
}

impl CodeGenerator {
    pub fn new(
        out_dir: PathBuf,
        // TODO: remove tmp_build_dir from constructor in favor of generated tmp dir
        tmp_build_dir: PathBuf,
        project: CosmosProject,
        deps: Vec<CosmosProject>
    ) -> Self {
        Self {
            project,
            root: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            out_dir,
            tmp_build_dir,
            deps,
        }
    }

    pub fn generate(&self) {
        info!("Starting generation process for project '{}'", self.project.name);
        info!("Root directory: {:?}", self.root);
        info!("Output directory: {:?}", self.out_dir);
        info!("Temporary build directory: {:?}", self.tmp_build_dir);

        self.prepare_dir();
        self.compile_proto();

        info!(
            "🧪 [{}] Embellishing modules to expose nice API for library user...",
            self.project.name
        );

        self.transform();
        self.generate_mod_file();
        self.fmt();

        info!("✨  [{}] Library is successfully generated!", self.project.name);
    }

    fn prepare_dir(&self) {
        info!("Preparing directories for project '{}'", self.project.name);
        if self.tmp_build_dir.exists() {
            info!("Removing existing temporary build directory: {:?}", self.tmp_build_dir);
            match remove_dir_all(self.tmp_build_dir.clone()) {
                Ok(_) => info!("Successfully removed temporary build directory"),
                Err(e) => {
                    info!("Warning: Failed to remove temporary build directory: {}", e);
                    // Continue anyway as this might not be critical
                }
            }
        }

        let namespaced_dir = self.tmp_namespaced_dir();
        info!("Creating namespaced directory: {:?}", namespaced_dir);
        match create_dir_all(&namespaced_dir) {
            Ok(_) => info!("Successfully created namespaced directory"),
            Err(e) => panic!("Failed to create namespaced directory: {}", e),
        }

        output_version_file(&self.project.name, &self.project.version, &self.tmp_namespaced_dir());
    }

    fn generate_mod_file(&self) {
        mod_gen::generate_mod_file(&self.absolute_out_dir());
    }

    fn transform(&self) {
        transform::copy_and_transform_all(
            &self.tmp_namespaced_dir(),
            &self.absolute_out_dir(),
            &self.file_descriptor_set()
        );
    }

    fn absolute_out_dir(&self) -> PathBuf {
        self.root.join(&self.out_dir)
    }

    fn fmt(&self) {
        let manifest_path = find_cargo_toml(&self.absolute_out_dir());
        let exit_status = Command::new("cargo")
            .arg("fmt")
            .arg("--manifest-path")
            .arg(manifest_path.to_string_lossy().to_string())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        if !exit_status.success() {
            panic!(
                "unable to format with: cargo fmt --manifest-path {}",
                manifest_path.to_string_lossy()
            );
        }
    }

    fn compile_proto(&self) {
        let mut all_related_projects = self.deps.clone();
        all_related_projects.push(self.project.clone());

        let buf_gen_template = self.root.join("buf.gen.yaml");
        info!("Using buf.gen.yaml template at: {:?}", buf_gen_template);

        info!("🧪 [{}] Compiling types from protobuf definitions...", self.project.name);

        // Compile proto files for each file in `protos` variable
        // `buf generate —template {<buf_gen_template} <proto_file>`
        for project in all_related_projects {
            info!("Processing project: {}", project.name);
            let project_dir_path = self.root.join(&project.project_dir);
            info!("Project directory: {:?}", project_dir_path);

            let buf_root = if
                project.name == "cosmos" ||
                project.name == "ics23" ||
                project.name == "admin"
            {
                let path = self.root.join(&project.project_dir).join("proto");
                info!("Special project {} using proto directory: {:?}", project.name, path);
                path
            } else {
                // Try to find buf.yaml/buf.yml but handle the case when it doesn't exist
                info!(
                    "Looking for buf.yaml or buf.yml in {:?}",
                    self.root.join(&project.project_dir)
                );
                match
                    WalkDir::new(&self.root.join(&project.project_dir))
                        .into_iter()
                        .filter_map(|e| {
                            if let Err(err) = &e {
                                info!("Error walking directory: {}", err);
                                return None;
                            }
                            e.ok()
                        })
                        .find(|e| {
                            let is_buf_file = e
                                .file_name()
                                .to_str()
                                .map(|s| (s == "buf.yaml" || s == "buf.yml"))
                                .unwrap_or(false);
                            if is_buf_file {
                                info!("Found buf file at: {:?}", e.path());
                            }
                            is_buf_file
                        })
                        .map(|e| e.path().parent().unwrap().to_path_buf())
                {
                    Some(path) => {
                        info!("Found buf configuration directory for {}: {:?}", project.name, path);
                        path
                    }
                    None => {
                        // If buf.yaml/buf.yml not found, use the project directory itself or proto subdirectory if it exists
                        let proto_dir = self.root.join(&project.project_dir).join("proto");
                        if proto_dir.exists() {
                            info!("Using proto directory for {}: {:?}", project.name, proto_dir);
                            proto_dir
                        } else {
                            info!(
                                "No buf.yaml/buf.yml found for project {}. Using project directory: {:?}",
                                project.name,
                                self.root.join(&project.project_dir)
                            );
                            self.root.join(&project.project_dir)
                        }
                    }
                }
            };

            debug!("buf_root for project {:?}: {:?}", project.name, buf_root);
            info!("Using buf_root for {}: {:?}", project.name, buf_root);

            // Create the directory if it doesn't exist
            if !buf_root.exists() {
                info!("Creating directory: {:?}", buf_root);
                std::fs::create_dir_all(&buf_root).unwrap_or_else(|e| {
                    panic!("Failed to create directory {:?}: {}", buf_root, e);
                });
            }

            let proto_path = &self.root.join(&project.project_dir).join("proto");
            info!("Proto path for {}: {:?}", project.name, proto_path);

            if !proto_path.exists() {
                info!("Warning: Proto path does not exist: {:?}", proto_path);
            }

            let mut cmd = Command::new("buf");
            cmd.arg("generate")
                .arg(buf_root.to_string_lossy().to_string())
                .arg("--template")
                .arg(buf_gen_template.to_string_lossy().to_string())
                .arg("--output")
                .arg(self.tmp_namespaced_dir().to_string_lossy().to_string());

            if !project.exclude_mods.is_empty() {
                info!("Excluding modules for {}: {:?}", project.name, project.exclude_mods);
                for excluded_mod in project.exclude_mods.clone() {
                    let exclude_path = proto_path.join(project.name.clone()).join(&excluded_mod);
                    info!("Excluding path: {:?}", exclude_path);
                    cmd.arg("--exclude-path").arg(exclude_path);
                }
            }

            info!("Running buf generate command: {:?}", cmd);
            let spawn_result = cmd.spawn();

            if let Err(err) = &spawn_result {
                panic!("Failed to spawn buf generate command: {}", err);
            }

            let exit_status = spawn_result.unwrap().wait().unwrap();

            if !exit_status.success() {
                panic!("unable to generate with: {:?}", cmd.get_args().collect::<Vec<_>>());
            }

            let descriptor_file = self
                .tmp_namespaced_dir()
                .join(format!("descriptor_{}.bin", project.name));

            // generate descriptor file with `buf build buf.yaml --as-file-descriptor-set -o {descriptor_file}`
            let mut cmd = Command::new("buf");
            cmd.arg("build")
                .arg(buf_root.to_string_lossy().to_string())
                .arg("--as-file-descriptor-set")
                .arg("-o")
                .arg(descriptor_file.to_string_lossy().to_string());

            if !project.exclude_mods.is_empty() {
                for include_mod in project.exclude_mods {
                    let exclude_path = proto_path.join(project.name.clone()).join(&include_mod);
                    cmd.arg("--exclude-path").arg(exclude_path);
                }
            }

            info!("Running buf build command: {:?}", cmd);
            let spawn_result = cmd.spawn();

            if let Err(err) = &spawn_result {
                panic!("Failed to spawn buf build command: {}", err);
            }

            let exit_status = spawn_result.unwrap().wait().unwrap();

            if !exit_status.success() {
                panic!("unable to build with: {:?}", cmd.get_args().collect::<Vec<_>>());
            }
        }

        info!(
            "✨  [{}] Types from protobuf definitions is compiled successfully!",
            self.project.name
        );
    }

    pub fn file_descriptor_set(&self) -> protobuf::descriptor::FileDescriptorSet {
        // list all files in self.tmp_namespaced_dir()
        let files = fs
            ::read_dir(self.tmp_namespaced_dir())
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        // filter only files that match "descriptor_*.bin"
        let descriptor_files = files
            .iter()
            .filter(|f| { f.file_name().unwrap().to_str().unwrap().starts_with("descriptor_") })
            .collect::<Vec<_>>();

        // read all files and merge them into one FileDescriptorSet
        let mut file_descriptor_set = protobuf::descriptor::FileDescriptorSet {
            file: vec![],
            special_fields: Default::default(),
        };
        for descriptor_file in descriptor_files {
            let descriptor_bytes = &fs::read(descriptor_file).unwrap()[..];
            let mut file_descriptor_set_tmp: protobuf::descriptor::FileDescriptorSet = protobuf::Message
                ::parse_from_bytes(descriptor_bytes)
                .unwrap();
            file_descriptor_set.file.append(&mut file_descriptor_set_tmp.file);
        }

        file_descriptor_set
    }

    fn tmp_namespaced_dir(&self) -> PathBuf {
        self.tmp_build_dir.join(&self.project.name)
    }
}

fn output_version_file(project_name: &str, versions: &str, out_dir: &Path) {
    let path = out_dir.join(format!("{}_COMMIT", project_name.to_uppercase()));
    fs::write(path, versions).unwrap();
}

fn find_cargo_toml(path: &Path) -> PathBuf {
    if path.join("Cargo.toml").exists() {
        path.to_path_buf().join("Cargo.toml")
    } else {
        find_cargo_toml(path.parent().expect("Cargo.toml not found"))
    }
}

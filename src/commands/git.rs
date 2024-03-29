use std::{fs::File, io::Write, path::Path};

use clap::{Args, Subcommand};
use console::Style;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};

use crate::utils::{FileExists, WriteContent};
/// the git utils is used to for handling git operations
/// this IncludeFile Includeing files such as readme, license and .ignore files to the repo.
/// # Example
/// ```sh
/// cargo run -- git repo --branch "master" --remote "https://github.com/opeolluwa/utils" --add-readme
/// ```
///
#[derive(Args, Debug, Deserialize, Serialize)]
pub struct GitCommands {
    /// bears an enum for sub commands
    /// the allowed variants are
    /// - repo - to manage repositories
    /// - add-file
    #[command(subcommand)]
    command: GitSubCommands,
    /// the path to perform the action
    #[clap(short, long, value_parser, default_value = ".")]
    pub path: String,
}
#[derive(Debug, Subcommand, Serialize, Deserialize, Clone)]
enum GitSubCommands {
    /// convert local repo to remote repo
    Repo(RepoArgs),
    IncludeFile(IncludeFileArgs),
}
#[derive(Args, Debug, Deserialize, Serialize, Clone)]
struct RepoArgs {
    // TODO() set this co another command
    #[clap(short, long, value_parser)]
    branch: String,
    #[clap(short, long, value_parser)]
    remote: String,
}

/// manage file added to the repo
#[derive(Args, Debug, Deserialize, Serialize, Clone)]
struct IncludeFileArgs {
    /// include a license
    #[clap(short, long, value_parser)]
    license: bool,
    /// include a .gitignore file
    #[clap(short, long, value_parser)]
    ignore: bool,
    /// include code of conduct
    #[clap(short, long, value_parser)]
    policy: bool,
    /// include a contribution guide
    #[clap(short, long, value_parser)]
    contribution: bool,
}

/// parse the Cli option here
impl GitCommands {
    /// parse the commands
    pub fn parse(&self) {
        match &self.command {
            GitSubCommands::Repo(repo) => {
                println!("repo {:?}", repo);
            }
            GitSubCommands::IncludeFile(file) => {
                println!("IncludeFile {:?}", file);
                /* let cpp_ignore = IncludeFileArgs::cpp_ignore_file();
                println!("{:#?}", cpp_ignore); */

                if file.ignore {
                    let languages = vec!["C", "C++", "Node", "Rust", "Deno"];
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .items(&languages)
                        .default(0)
                        .interact_on_opt(&Term::stderr())
                        .unwrap();

                    if let Some(language) = selection {
                        let binding = Path::new(&self.path);
                        let gitignore_file_path = binding.join(".gitignore");
                        let path = gitignore_file_path.as_path();
                        match language {
                            0 => {
                                println!("C");
                                let c_ignore = IncludeFileArgs::c_ignore_file();
                                if IncludeFileArgs::file_exists(path) {
                                    let error_style = Style::new().for_stderr().red();
                                    println!(
                                        "{}",
                                        error_style
                                            .apply_to("the .gitignore already exist, use the --force flag to overwrite it")
                                    );
                                    return;
                                }
                                println!("{:#?}", c_ignore);
                            }
                            1 => {
                                println!("C++");
                                let cpp_ignore = IncludeFileArgs::cpp_ignore_file();
                                println!("{:#?}", cpp_ignore);
                            }
                            2 => {
                                println!("Node");
                                let node_ignore = IncludeFileArgs::node_ignore_file();
                                println!("{:#?}", node_ignore);
                            }
                            3 => {
                                println!("Rust");
                                let rust_ignore = IncludeFileArgs::rust_ignore_file();
                                println!("{:#?}", rust_ignore);
                            }
                            4 => {
                                println!("Deno");
                                let deno_ignore = IncludeFileArgs::rust_ignore_file();
                                println!("{:#?}", deno_ignore);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

// impl WriteContent for IncludeFileArgs {
//     fn write_content(path: &Path, content: &str) -> Result<(), std::io::Error> {
//         let mut file = File::create(path).unwrap();
//         file.write_all(content.as_bytes()).unwrap();
//         Ok(())
//     }
// }

impl FileExists for IncludeFileArgs {
    fn file_exists(path: &Path) -> bool {
        path.exists()
    }
}
impl IncludeFileArgs {
    pub fn c_ignore_file() -> &'static str {
        r#"# Prerequisites
*.d

# Object files
*.o
*.ko
*.obj
*.elf

# Linker output
*.ilk
*.map
*.exp

# Precompiled Headers
*.gch
*.pch

# Libraries
*.lib
*.a
*.la
*.lo

# Shared objects (inc. Windows DLLs)
*.dll
*.so
*.so.*
*.dylib

# Executables
*.exe
*.out
*.app
*.i*86
*.x86_64
*.hex

# Debug files
*.dSYM/
*.su
*.idb
*.pdb

# Kernel Module Compile Results
*.mod*
*.cmd
.tmp_versions/
modules.order
Module.symvers
Mkfile.old
dkms.conf"#
    }

    pub fn node_ignore_file() -> &'static str {
        r#"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
lerna-debug.log*
.pnpm-debug.log*

# Diagnostic reports (https://nodejs.org/api/report.html)
report.[0-9]*.[0-9]*.[0-9]*.[0-9]*.json

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Directory for instrumented libs generated by jscoverage/JSCover
lib-cov

# Coverage directory used by tools like istanbul
coverage
*.lcov

# nyc test coverage
.nyc_output

# Grunt intermediate storage (https://gruntjs.com/creating-plugins#storing-task-files)
.grunt

# Bower dependency directory (https://bower.io/)
bower_components

# node-waf configuration
.lock-wscript

# Compiled binary addons (https://nodejs.org/api/addons.html)
build/Release

# Dependency directories
node_modules/
jspm_packages/

# Snowpack dependency directory (https://snowpack.dev/)
web_modules/

# TypeScript cache
*.tsbuildinfo

# Optional npm cache directory
.npm

# Optional eslint cache
.eslintcache

# Optional stylelint cache
.stylelintcache

# Microbundle cache
.rpt2_cache/
.rts2_cache_cjs/
.rts2_cache_es/
.rts2_cache_umd/

# Optional REPL history
.node_repl_history

# Output of 'npm pack'
*.tgz

# Yarn Integrity file
.yarn-integrity

# dotenv environment variable files
.env
.env.development.local
.env.test.local
.env.production.local
.env.local

# parcel-bundler cache (https://parceljs.org/)
.cache
.parcel-cache

# Next.js build output
.next
out

# Nuxt.js build / generate output
.nuxt
dist

# Gatsby files
.cache/
# Comment in the public line in if your project uses Gatsby and not Next.js
# https://nextjs.org/blog/next-9-1#public-directory-support
# public

# vuepress build output
.vuepress/dist

# vuepress v2.x temp and cache directory
.temp
.cache

# Docusaurus cache and generated files
.docusaurus

# Serverless directories
.serverless/

# FuseBox cache
.fusebox/

# DynamoDB Local files
.dynamodb/

# TernJS port file
.tern-port

# Stores VSCode versions used for testing VSCode extensions
.vscode-test

# yarn v2
.yarn/cache
.yarn/unplugged
.yarn/build-state.yml
.yarn/install-state.gz
.pnp.*"#
    }

    pub fn rust_ignore_file() -> &'static str {
        r#"# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb"#
    }
    pub fn cpp_ignore_file() -> &'static str {
        r#"
        # Prerequisites
*.d

# Compiled Object files
*.slo
*.lo
*.o
*.obj

# Precompiled Headers
*.gch
*.pch

# Compiled Dynamic libraries
*.so
*.dylib
*.dll

# Fortran module files
*.mod
*.smod

# Compiled Static libraries
*.lai
*.la
*.a
*.lib

# Executables
*.exe
*.out
*.app
        "#
    }
}

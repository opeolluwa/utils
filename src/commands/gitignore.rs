use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use console::Style;

use serde::Serialize;

use crate::{
    style::PrintColoredText,
    utils::{FileExists, WriteContent},
    SOURCE_DIR,
};
use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

#[derive(clap::Args, Debug, Serialize)]
pub struct GitIgnoreCommands {
    /// the path to generate the .gitignore file
    #[clap(short, long, value_parser, default_value = ".")]
    pub path: String,
    /// overwrite existing
    #[clap(short, long, value_parser)]
    pub force: bool,
    /// overwrite existing
    #[clap(short, long, value_parser)]
    pub backup: bool,
}

impl GitIgnoreCommands {
    /// parse the commands
    pub fn parse(&self) {
        let path_binding = Path::new(&self.path);
        let binding = path_binding.join(".gitignore");
        let path = binding.as_path();
        if path.exists() && !self.force {
            let error_style = Style::new().for_stderr().red();
            println!(
                "{}",
                error_style
                    .apply_to("the .gitignore already exist, use the --force flag to overwrite it")
            );
            return;
        }
        // delete the existing file and create a new one
        if self.file_exists() && self.force {
            fs::remove_file(path).unwrap();
        }

        // backup existing .gitignore
        if self.file_exists() && self.backup {
            let backup_path = Path::new(&self.path).join(".gitignore.bak");
            fs::copy(path, backup_path).unwrap();
        }

        if self.file_exists() && (self.backup && self.force) {
            let backup_path = Path::new(&self.path).join(".gitignore.bak");
            fs::copy(path, backup_path).unwrap();
            fs::remove_file(path).unwrap();
        }

        let technologies = vec![
            "AL",
            "Actionscript",
            "Ada",
            "Agda",
            "Android",
            "AppEngine",
            "AppceleratorTitanium",
            "ArchLinuxPackages",
            "Autotools",
            "C++",
            "C",
            "CFWheels",
            "CMake",
            "CUDA",
            "CakePHP",
            "ChefCookbook",
            "Clojure",
            "CodeIgniter",
            "CommonLisp",
            "Composer",
            "Concrete5",
            "Coq",
            "CraftCMS",
            "D",
            "DM",
            "Dart",
            "Delphi",
            "Drupal",
            "EPiServer",
            "Eagle",
            "Elisp",
            "Elixir",
            "Elm",
            "Erlang",
            "ExpressionEngine",
            "ExtJs",
            "Fancy",
            "Finale",
            "FlaxEngine",
            "ForceDotCom",
            "Fortran",
            "FuelPHP",
            "GWT",
            "Gcov",
            "GitBook",
            "Go",
            "Godot",
            "Gradle",
            "Grails",
            "Haskell",
            "IGORPro",
            "Idris",
            "JBoss",
            "JENKINS_HOME",
            "Java",
            "Jekyll",
            "Joomla",
            "Julia",
            "KiCad",
            "Kohana",
            "Kotlin",
            "LICENSE",
            "LabVIEW",
            "Laravel",
            "Leiningen",
            "LemonStand",
            "Lilypond",
            "Lithium",
            "Lua",
            "Magento",
            "Maven",
            "Mercury",
            "MetaProgrammingSystem",
            "Nanoc",
            "Nim",
            "Node",
            "OCaml",
            "Objective-C",
            "Opa",
            "OpenCart",
            "OracleForms",
            "Packer",
            "Perl",
            "Phalcon",
            "PlayFramework",
            "Plone",
            "Prestashop",
            "Processing",
            "PureScript",
            "Python",
            "Qooxdoo",
            "Qt",
            "R",
            "ROS",
            "Racket",
            "Rails",
            "Raku",
            "RhodesRhomobile",
            "Ruby",
            "Rust",
            "SCons",
            "Sass",
            "Scala",
            "Scheme",
            "Scrivener",
            "Sdcc",
            "SeamGen",
            "SketchUp",
            "Smalltalk",
            "Stella",
            "SugarCRM",
            "Swift",
            "Symfony",
            "SymphonyCMS",
            "TeX",
            "Terraform",
            "Textpattern",
            "TurboGears2",
            "TwinCAT3",
            "Typo3",
            "Unity",
            "UnrealEngine",
            "VVVV",
            "VisualStudio",
            "Waf",
            "WordPress",
            "Xojo",
            "Yeoman",
            "Yii",
            "ZendFramework",
            "Zephir",
        ];
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&technologies)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        if let Some(index) = selection {
            println!("User selected item : {}", technologies[index]);
            let src_path = format!("gitignore/{}.gitignore", technologies[index]);
            let file_path = SOURCE_DIR.get_file(src_path);

            if let Some(file_content) = file_path {
                let mut file = File::create(path).unwrap();
                file.write_all(file_content.contents()).unwrap();
            } 
        } 
    }
}

impl FileExists for GitIgnoreCommands {
    /// check if the file exist
    fn file_exists(&self) -> bool {
        let path_binding = Path::new(&self.path);
        let path_binding = path_binding.join(".gitignore");
        let path = path_binding.as_path();
        path.exists()
    }
}

impl WriteContent for GitIgnoreCommands {
    fn write_content(&self, _content: &str) -> std::io::Result<()> {
        let path_binding = Path::new(&self.path);
        let binding = path_binding.join(".gitignore");
        let path = binding.as_path();
        if self.file_exists() {
            PrintColoredText::error(
                "the .gitignore already exist, use the --force flag to overwrite it",
            );
            return Ok(());
        }
        if self.file_exists() && self.force {
            // delete the existing file and create a new one
            fs::remove_file(path).unwrap();
        }

        Ok(())
    }
}

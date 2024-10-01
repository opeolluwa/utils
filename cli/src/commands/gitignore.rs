use crate::constants::SOURCE_DIR;
use crate::pkg::{FileExists, WriteContent};
use console::Style;
use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use serde::Serialize;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use utils_style::style::LogMessage;

#[derive(clap::Args, Debug, Serialize, Clone)]
pub struct GitIgnoreCommands {
    /// the tool/programming language to use
    #[clap(short, long, value_parser)]
    pub lang: Option<String>,
    /// the path to generate the .gitignore file
    #[clap(short, long, value_parser, default_value = ".")]
    pub path: String,
    /// overwrite existing
    #[clap(short, long, value_parser)]
    pub force: bool,
    /// back up the existing ignorefile
    #[clap(short, long, value_parser)]
    pub backup: bool,
}

impl GitIgnoreCommands {
    /// parse the commands
    pub fn parse(&self) {
        let current_or_selected_path_option = Path::new(&self.path);

        let possible_existing_ignore_file_path = current_or_selected_path_option.join(".gitignore");

        let gitignore_file_path = possible_existing_ignore_file_path.as_path();

        if gitignore_file_path.exists() && !self.force {
            return Self::throw_conflict_error();
        }
        // delete the existing file and create a new one before other operations
        if self.file_exists() && self.force {
            fs::remove_file(gitignore_file_path).unwrap();
        }

        // backup existing .gitignore
        if self.file_exists() && self.backup {
            let backup_path = Path::new(&self.path).join(".gitignore.bak");
            fs::copy(gitignore_file_path, backup_path).unwrap();
        }

        if self.file_exists() && (self.backup && self.force) {
            let backup_path = Path::new(&self.path).join(".gitignore.bak");
            fs::copy(gitignore_file_path, backup_path).unwrap();
            fs::remove_file(gitignore_file_path).unwrap();
        }

        match &self.lang {
            Some(lang) => Self::parse_received_language_argument(lang, gitignore_file_path),
            _ => Self::parse_fuzzy_selection(gitignore_file_path),
        }
    }

    fn parse_received_language_argument(lang: &String, path: &Path) {
        let programming_languages = Self::supported_technologies();

        let selected_language = programming_languages
            .into_iter()
            .map(|tech| tech.to_ascii_lowercase())
            .find(|tech| tech == lang);

        match selected_language {
            Some(language) => Self::generate_ignore_file_to_path(&language, path),
            _ => {
                LogMessage::error("Oops! An unexpected error happened while parsing the selected technology, continue tying to find it\n\n");
                Self::parse_fuzzy_selection(path);
            }
        }
    }
    fn throw_conflict_error() {
        let error_style = Style::new().for_stderr().red();
        println!(
            "{}",
            error_style.apply_to(".gitignore already exist, use the --force flag to overwrite it")
        );
    }

    fn parse_fuzzy_selection(path: &Path) {
        let programming_languages = Self::supported_technologies();

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&programming_languages)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        if let Some(index) = selection {
            println!("User selected item : {}", programming_languages[index]);
            Self::generate_ignore_file_to_path(programming_languages[index], path);
        }
    }

    /// generate the ignore file for a language to the indicated path
    fn generate_ignore_file_to_path(technology: &str, path: &Path) {
        let src_path = format!("gitignore/{}.gitignore", technology);
        let file_path = SOURCE_DIR.get_file(src_path);

        if let Some(file_content) = file_path {
            let mut file = File::create(path).unwrap();
            file.write_all(file_content.contents()).unwrap();
        }
    }
    fn supported_technologies() -> Vec<&'static str> {
        vec![
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
        ]
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
            LogMessage::error("the .gitignore already exist, use the --force flag to overwrite it");
            return Ok(());
        }
        if self.file_exists() && self.force {
            // delete the existing file and create a new one
            fs::remove_file(path).unwrap();
        }

        Ok(())
    }
}

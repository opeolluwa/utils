//! give color to the output text
//!
use console::Style;

/// return colored text to stdout or std err
/// ### Example
/// ```rust
/// use utils_style::style::LogMessage;
/// LogMessage::error("the readme already exists");
/// ```
///
pub struct LogMessage;
#[allow(unused)]
impl LogMessage {
    /// print text with red
    pub fn error(message: &str) {
        let error_style = Style::new().for_stderr().red();
        println!("{}", error_style.apply_to(message));
    }
    /// stdout text-color = green
    pub fn success(message: &str) {
        let error_style = Style::new().for_stderr().green();
        println!("{}", error_style.apply_to(message));
    }
    /// stdout text-color = yellow
    pub fn warning(message: &str) {
        let error_style = Style::new().for_stderr().yellow();
        println!("{}", error_style.apply_to(message));
    }
    /// stdout text-color = black
    pub fn neutral(message: &str) {
        let error_style = Style::new().for_stderr().black();
        println!("{}", error_style.apply_to(message));
    }
}

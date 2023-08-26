use console::Style;

pub struct PrintColoredText;
/// return colored text to stdout or std err
impl PrintColoredText {
    pub fn error(message: &str) {
        let error_style = Style::new().for_stderr().red();
        println!("{}", error_style.apply_to(message));
    }

    pub fn success(message: &str) {
        let error_style = Style::new().for_stderr().green();
        println!("{}", error_style.apply_to(message));
    }
    pub fn warning(message: &str) {
        let error_style = Style::new().for_stderr().yellow();
        println!("{}", error_style.apply_to(message));
    }

    pub fn neutral(message: &str) {
        let error_style = Style::new().for_stderr().black();
        println!("{}", error_style.apply_to(message));
    }
}

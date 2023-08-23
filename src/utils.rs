use std::path::Path;

trait WriteContent {
    fn write_content(&self, path: &Path, content: &str) -> std::io::Result<()>;
}

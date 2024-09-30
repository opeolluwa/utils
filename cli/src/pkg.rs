#[allow(unused)]

pub trait WriteContent {
    fn write_content(&self, content: &str) -> std::io::Result<()>;
}

/// see if file exists in the given path
pub trait FileExists {
    fn file_exists(&self) -> bool;
}

#[allow(unused)]
/// delete a file
pub trait DeleteFile {
    fn delete_file() -> std::io::Result<()>;
}

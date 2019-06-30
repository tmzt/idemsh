
use super::errors::Result as ExecResult;

pub enum FileContents {
    StaticString(String)
}

pub trait Exec {
    fn change_directory(&mut self, dir: &str) -> ExecResult<()>;
    fn ensure_directory(&mut self, local_part: &str) -> ExecResult<()>;
    fn ensure_file_exists(&mut self, local_part: &str) -> ExecResult<()>;
    fn ensure_file_contents(&mut self, local_part: &str, contents: FileContents) -> ExecResult<()>;
    fn get_cwd(&mut self) -> ExecResult<String>;
}

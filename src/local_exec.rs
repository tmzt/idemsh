
use std::path::{Path, PathBuf};
use std::io::Result as IOResult;
use std::env::{current_dir};
use std::fs;

use super::ast::*;
use super::parser::*;
use super::traits::*;
use super::errors::Result as ExecResult;

#[derive(Debug, PartialEq, Clone)]
pub struct LocalExec {
    cwd: PathBuf,
}

impl Default for LocalExec {
    fn default() -> Self {
        LocalExec {
            cwd: current_dir().expect("Failed to get cwd, this should never happen.")
        }
    }
}

impl LocalExec {
    fn with_new_relative_working_dir<P: AsRef<Path>>(dir: P) -> Self {
        let cwd =  current_dir().expect("Failed to get cwd, this should never happen.")
                .join(dir);
        if !cwd.exists() { fs::create_dir_all(&cwd); }

        LocalExec {
            cwd: cwd.to_path_buf()
        }
    }
}

impl Exec for LocalExec {
    fn change_directory(&mut self, dir: &str) -> ExecResult<()> {
        Ok(())
    }

    fn ensure_directory(&mut self, local_part: &str) -> ExecResult<()> {
        let dir = self.cwd.join(local_part);
        eprintln!("Checking for path {:?}", dir);
        if !dir.exists() {
            eprintln!("Creating path {:?}", dir);
            fs::create_dir(dir)?;
        };

        Ok(())
    }

    fn ensure_file_exists(&mut self, local_part: &str) -> ExecResult<()> {
        Ok(())
    }

    fn ensure_file_contents(&mut self, local_part: &str, contents: FileContents) -> ExecResult<()> {
        Ok(())
    }

    fn get_cwd(&mut self) -> ExecResult<String> {
        Ok(self.cwd.to_str().unwrap().to_string())
    }
}

#[inline(always)]
fn find_single_path<'a>(obj: &'a IdemRawCommandWithPaths) -> Option<&'a IdemPath> {
    if obj.paths.len() != 1 { return None; }
    obj.paths.iter().next()
}

#[inline(always)]
fn find_single_flag_keyword(params: &[IdemParamType]) -> Option<&str> {
    if params.len() != 1 { return None; }

    match params[0] {
        IdemParamType::FlagKeyword(ref s) => {
            Some(s)
        }

        _ => None
    }
}

fn create_ignore_existing<T: AsRef<Path>>(path: T) -> IOResult<()> {
    fs::OpenOptions::new().create(true).write(true).open(path).map(|_| ())
}

#[cfg(test)]
mod tests {
    use nom::types::CompleteStr;

    #[macro_use]
    use super::*;

    macro_rules! local_exec {
        () => (LocalExec::with_new_relative_working_dir(&Path::new("./testing")));
    }

    #[test]
    fn test_ensure_file_exists() {
        let mut local_exec = local_exec!();

        assert!(local_exec.ensure_file_exists("./afile").ok().is_some());

        // Assert result
        assert!(Path::new("./testing/afile").is_file(), "./testing/afile does not exist or is not a file.");
    }

    #[test]
    fn test_ensure_directory_exists() {
        let mut local_exec = local_exec!();

        assert!(local_exec.ensure_directory("./adir").ok().is_some());

        // Assert result
        assert!(Path::new("./testing/adir").is_dir(), "./testing/adir does not exist or is not a directory.");
    }
}

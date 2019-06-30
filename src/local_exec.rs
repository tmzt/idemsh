
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

    // macro_rules!  parse (
    //     ($code: expr) => ({
    //         let expr = parse_raw_script(CompleteStr($code)).unwrap().1;
    //         expr
    //     });
    // );

    #[test]
    fn test_directory_exists() {
        // Execute script
        let mut local_exec = local_exec!();

        // Assert result
        assert!(local_exec.ensure_directory("./afile").ok().is_some());
    }

//     #[test]
//     fn test_directory_exists() {
//         let script = parse!(r#"
// ./adir/ (exists)
// "#);

//         // Verify script
//         assert_eq!(script, vec![
//             IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
//                 paths: vec![
//                     IdemPath(None, IdemPathLocalPartType::Directory("./adir".to_string())),
//                 ],
//                 params: vec![
//                     IdemParamType::FlagKeyword("exists".to_string()),
//                 ],
//             })
//         ]);

//         // Execute script
//         let local_exec = local_exec!();
//         local_exec.execute_raw_script_command(&script[0]).unwrap();

//         // Assert result
//         assert!(Path::new("./testing/adir").is_dir(), "./testing/adir does not exist or is not a directory.");
//     }

}

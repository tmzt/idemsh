
use std::path::{Path, PathBuf};
use std::io::Result as IOResult;
use std::env::{current_dir};
use std::fs;

use super::ast::*;
use super::parser::*;

pub type LocalResult = IOResult<()>;

#[derive(Debug, PartialEq, Clone)]
pub struct LocalExec {
    cwd: PathBuf
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

impl LocalExec {
    pub fn execute_raw_script_command(&self, cmd: &IdemRawCommandType) -> LocalResult {
        match cmd {
            IdemRawCommandType::WithPaths(obj) => {
                let flag = find_single_flag_keyword(&obj.params);
                if flag.is_some() {
                    eprintln!("Found single flag keyword: {:?}", flag);
                    match flag.unwrap() {
                        "exists" => {
                            let path = find_single_path(obj);
                            assert!(path.is_some(), "exists flag expects to follow a single path");
                            let IdemPath(_, path) = path.unwrap();

                            match path {
                                IdemPathLocalPartType::Directory(ref dir) => {
                                    let dir = self.cwd.join(dir);
                                    eprintln!("Checking for path {:?}", dir);
                                    if !dir.exists() {
                                        eprintln!("Creating path {:?}", dir);
                                        return fs::create_dir(dir);
                                    }
                                },
                                IdemPathLocalPartType::File(ref path) => {
                                    let path = self.cwd.join(path);
                                    eprintln!("Creating file {:?}, ignoring existing", path);
                                    return create_ignore_existing(&path)
                                },
                            }
                        }

                        _ => unimplemented!("Flag keyword not implemented")
                    }
                };

                Ok(())
            }

            _ => unimplemented!("Not implemented")
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::types::CompleteStr;

    #[macro_use]
    use super::*;

    macro_rules! local_exec {
        () => (LocalExec::with_new_relative_working_dir(&Path::new("./testing")));
    }

    macro_rules!  parse (
        ($code: expr) => ({
            let expr = parse_raw_script(CompleteStr($code)).unwrap().1;
            expr
        });
    );

    #[test]
    fn test_file_exists() {
        let script = parse!(r#"
./afile (exists)
"#);

        // Verify script
        assert_eq!(script, vec![
            IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
                paths: vec![
                    IdemPath(None, IdemPathLocalPartType::File("./afile".to_string())),
                ],
                params: vec![
                    IdemParamType::FlagKeyword("exists".to_string()),
                ],
            })
        ]);

        // Execute script
        let local_exec = local_exec!();
        local_exec.execute_raw_script_command(&script[0]).unwrap();

        // Assert result
        assert!(Path::new("./testing/afile").is_file(), "./testing/afile does not exist or is not a file.");
    }

    #[test]
    fn test_directory_exists() {
        let script = parse!(r#"
./adir/ (exists)
"#);

        // Verify script
        assert_eq!(script, vec![
            IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
                paths: vec![
                    IdemPath(None, IdemPathLocalPartType::Directory("./adir".to_string())),
                ],
                params: vec![
                    IdemParamType::FlagKeyword("exists".to_string()),
                ],
            })
        ]);

        // Execute script
        let local_exec = local_exec!();
        local_exec.execute_raw_script_command(&script[0]).unwrap();

        // Assert result
        assert!(Path::new("./testing/adir").is_dir(), "./testing/adir does not exist or is not a directory.");
    }

}


use super::ast::*;
use super::parser::*;
use super::traits::*;
use super::errors::Result as ExecResult;

#[derive(Debug, PartialEq)]
pub struct HandleExec<'e, E: Exec> {
    driver: &'e mut E,
    cwd: String
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

impl<'e, E: Exec> HandleExec<'e, E> {
    pub fn new(driver: &'e mut E) -> Self {
        HandleExec {
            driver: driver,
            cwd: "./".to_string(),
        }
    }

    pub fn execute_raw_script_command(&mut self, cmd: &IdemRawCommandType) -> ExecResult<()> {
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
                                    self.driver.ensure_directory(dir)
                                },
                                IdemPathLocalPartType::File(ref path) => {
                                    self.driver.ensure_file_exists(path)
                                },
                            }
                        },

                        _ => unimplemented!("Flag keyword not implemented")
                    };
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

    #[derive(Debug, PartialEq, Clone)]
    pub struct TestExec {
        cwd: String,
        pub created_dirs: Vec<String>,
        pub created_files: Vec<String>,
    }

    impl TestExec {
        pub fn new(cwd: &str) -> Self {
            TestExec {
                cwd: cwd.to_string(),
                created_dirs: vec![],
                created_files: vec![],
            }
        }
    }

    #[inline]
    fn join_paths(a: &str, b: &str) -> String {
        let a = a.trim_end_matches("/").replace("./", "");
        let b = b.trim_start_matches("/").replace("./", "");
        format!("{}/{}", a, b)
    }

    impl Exec for TestExec {
        fn change_directory(&mut self, dir: &str) -> ExecResult<()> {
            Ok(())
        }

        fn ensure_directory(&mut self, local_part: &str) -> ExecResult<()> {
            let dir = join_paths(&self.cwd, local_part);
            self.created_dirs.push(dir);
            Ok(())
        }

        fn ensure_file_exists(&mut self, local_part: &str) -> ExecResult<()> {
            let filepath = join_paths(&self.cwd, local_part);
            self.created_files.push(filepath);
            Ok(())
        }

        fn ensure_file_contents(&mut self, local_part: &str, contents: FileContents) -> ExecResult<()> {
            let filepath = join_paths(&self.cwd, local_part);
            self.created_files.push(filepath);
            Ok(())
        }

        fn get_cwd(&mut self) -> ExecResult<String> {
            Ok(self.cwd.to_string())
        }
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
        let mut test_exec = TestExec::new("./testing");
        let mut handle_exec = HandleExec::new(&mut test_exec);
        handle_exec.execute_raw_script_command(&script[0]).unwrap();

        // Assert result
        assert_eq!(test_exec.created_files, vec!["testing/afile"]);
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
        let mut test_exec = TestExec::new("./testing");
        let mut handle_exec = HandleExec::new(&mut test_exec);
        handle_exec.execute_raw_script_command(&script[0]).unwrap();

        // Assert result
        assert_eq!(test_exec.created_dirs, vec!["testing/adir"]);
    }

}

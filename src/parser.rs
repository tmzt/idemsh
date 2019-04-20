
use nom::types::CompleteStr;
use nom::anychar;

#[macro_use]
use super::ast::*;

named!(parse_identifier<CompleteStr, CompleteStr>,
    recognize!(
        do_parse!(
            one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
            many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
            ()
        )
    )
);

named!(parse_path<CompleteStr, IdemPath>,
    do_parse!(
        s: many1!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.\\/")) >>
        ({
            let s: String = s.into_iter().collect();
            eprintln!("Got path: '{}'", s);

            if s.ends_with("/") {
                IdemPath(None, IdemPathLocalPartType::Directory(s.trim_end_matches('/').to_string()))
            } else {
                IdemPath(None, IdemPathLocalPartType::File(s))
            }
        })
    )
);

named!(parse_resource<CompleteStr, IdemResourceType>,
    alt_complete!(
        map!(parse_path, |IdemPath(_, s)| {
            match s {
                IdemPathLocalPartType::File(s) => IdemResourceType::File(s),
                IdemPathLocalPartType::Directory(s) => IdemResourceType::Directory(s),
            }
        })
    )
);

named!(parse_value_litstring<CompleteStr, IdemValueType>,
    do_parse!(
        ws!(tag!("\"")) >>
        value: take_until_either!("\"") >>
        tag!("\"") >>
        (IdemValueType::LitString(value.0.to_string()))
    )
);

named!(parse_value_path_spec<CompleteStr, IdemValueType>,
    do_parse!(
        path: ws!(parse_path) >>
        (IdemValueType::PathSpec(path))
    )
);

named!(parse_value<CompleteStr, IdemValueType>,
    alt_complete!(
        parse_value_litstring |
        parse_value_path_spec
    )
);

named!(parse_param_key_value<CompleteStr, IdemParamType>,
    do_parse!(
        key: ws!(parse_identifier) >>
        ws!(tag!("=")) >>
        value: ws!(parse_value) >>
        (IdemParamType::KeyValue(key.to_string(), value))
    )
);

named!(parse_param_flag_keyword<CompleteStr, IdemParamType>,
        alt_complete!(
            map!(ws!(recognize!(tag!("copied"))), |s| IdemParamType::FlagKeyword(s.to_string()))
            | map!(ws!(recognize!(tag!("exists"))), |s| IdemParamType::FlagKeyword(s.to_string()))
        )
);

named!(parse_param<CompleteStr, IdemParamType>,
    alt_complete!(
        ws!(parse_param_key_value)
        | ws!(parse_param_flag_keyword)
    )
);

named!(parse_raw_command_with_paths<CompleteStr, IdemRawCommandWithPaths>,
    do_parse!(
        paths: many1!(ws!(parse_path)) >>
        ws!(tag!("(")) >>
        params: separated_list!(ws!(tag!(",")), ws!(parse_param)) >>
        ws!(tag!(")")) >>
        ({
            IdemRawCommandWithPaths {
                paths: paths,
                params: params,
            }
        })
    )
);

named!(parse_raw_statements<CompleteStr, Vec<IdemRawCommandType>>,
    many0!(ws!(parse_raw_command))
);

named!(parse_raw_command_each<CompleteStr, IdemRawCommandType>,
    do_parse!(
        ws!(tag!("each")) >>
        key: ws!(parse_identifier) >>
        ws!(tag!("in")) >>
        coll: ws!(parse_value) >>
        statements: ws!(map_res!(
            take_until!("end"),
            parse_raw_statements
        )) >>
        ws!(tag!("end")) >>
        ({
            let statements = statements.1.into_iter().map(|s| Box::new(s)).collect();
            IdemRawCommandType::Each(key.0.to_string(), coll, statements)
        })
    )
);

named!(parse_raw_command_with_block<CompleteStr, IdemRawCommandType>,
    do_parse!(
        ws!(tag!("with")) >>
        resource: ws!(parse_resource) >>

        as_: opt!(
            do_parse!(
                ws!(tag!("as")) >>
                as_: ws!(parse_identifier) >>
                (as_.to_string())
            )
        ) >>

        statements: ws!(map_res!(
            take_until!("end"),
            parse_raw_statements
        )) >>
        ws!(tag!("end")) >>
        ({
            let statements = statements.1.into_iter().map(|s| Box::new(s)).collect();
            IdemRawCommandType::WithBlock(
                resource,
                as_.map(|s| s.to_string()),
                statements,
            )
        })
    )
);

named!(parse_raw_command<CompleteStr, IdemRawCommandType>,
    alt_complete!(
        parse_raw_command_each |
        parse_raw_command_with_block |
        map!(parse_raw_command_with_paths, |p| IdemRawCommandType::WithPaths(p))
    )
);

named!(pub parse_raw_script<CompleteStr, Vec<IdemRawCommandType>>,
    call!(parse_raw_statements)
);

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;

    macro_rules!  test_parser (
        ($code: expr, $parser: ident, $expected: expr) => {
        let expr = $parser($code).unwrap().1;
        assert_eq!(
                expr,
                $expected
            );
        };
    );

    #[test]
    fn test_parse_path1() {
        test_parser!(
            CompleteStr(&r"path/"),
            parse_path,
            IdemPath(None, IdemPathLocalPartType::Directory("path".to_string()))
        );
    }

    #[test]
    fn test_parse_path2() {
        test_parser!(
            CompleteStr(&r"path"),
            parse_path,
            IdemPath(None, IdemPathLocalPartType::File("path".to_string()))
        );
    }

    #[test]
    fn test_parse_path3() {
        test_parser!(
            CompleteStr(&r"path/to/path/"),
            parse_path,
            IdemPath(None, IdemPathLocalPartType::Directory("path/to/path".to_string()))
        );
    }

    #[test]
    fn test_parse_path4() {
        test_parser!(
            CompleteStr(&r"path/to/path"),
            parse_path,
            IdemPath(None, IdemPathLocalPartType::File("path/to/path".to_string()))
        );
    }

    #[test]
    fn test_parse_value_litstring() {
        test_parser!(
            CompleteStr(&r#""value""#),
            parse_value_litstring,
            IdemValueType::LitString("value".to_string())
        );
    }

    #[test]
    fn test_parse_value_path_spec1() {
        test_parser!(
            CompleteStr(&r#"./path"#),
            parse_value_path_spec,
            IdemValueType::PathSpec(
                IdemPath(None, IdemPathLocalPartType::File("./path".to_string()))
            )
        );
    }

    #[test]
    fn test_parse_value_path_spec2() {
        test_parser!(
            CompleteStr(&r#"./path/"#),
            parse_value_path_spec,
            IdemValueType::PathSpec(
                IdemPath(None, IdemPathLocalPartType::Directory("./path".to_string()))
            )
        );
    }

    #[test]
    fn test_parse_param_key_value() {
        test_parser!(
            CompleteStr(&r#"key="value""#),
            parse_param_key_value,
            IdemParamType::KeyValue("key".to_string(), IdemValueType::LitString("value".to_string()))
        );
    }

    #[test]
    fn test_parse_param_flag_keyword() {
        test_parser!(
            CompleteStr(&"copied"),
            parse_param_flag_keyword,
            IdemParamType::FlagKeyword("copied".to_string())
        );
    }

    #[test]
    fn test_parse_raw_command_with_paths1() {
        test_parser!(
            CompleteStr(&r#"./path1 ./path2 (key="value")"#),
            parse_raw_command_with_paths,
            IdemRawCommandWithPaths {
                paths: vec![
                    IdemPath(None, IdemPathLocalPartType::File("./path1".to_string())),
                    IdemPath(None, IdemPathLocalPartType::File("./path2".to_string())),
                ],
                params: vec![
                    IdemParamType::KeyValue("key".to_string(), IdemValueType::LitString("value".to_string()))
                ]
            }
        );
    }

    #[test]
    fn test_parse_raw_command_with_paths2() {
        test_parser!(
            CompleteStr(&r#"./path1/ ./path2/ (copied)"#),
            parse_raw_command_with_paths,
            IdemRawCommandWithPaths {
                paths: vec![
                    IdemPath(None, IdemPathLocalPartType::Directory("./path1".to_string())),
                    IdemPath(None, IdemPathLocalPartType::Directory("./path2".to_string())),
                ],
                params: vec![
                    IdemParamType::FlagKeyword("copied".to_string()),
                ]
            }
        );
    }

    #[test]
    fn test_parse_raw_command_each() {
        test_parser!(
            CompleteStr(&r#"
each i in ./dir/
    ./a (mode="755")
    ./b (mode="600")
end
"#),
            parse_raw_command_each,
            IdemRawCommandType::Each(
                "i".to_string(),
                IdemValueType::PathSpec(
                    IdemPath(None, IdemPathLocalPartType::Directory("./dir".to_string()))
                ),
                vec![
                    Box::new(IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
                        paths: vec![
                            IdemPath(None, IdemPathLocalPartType::File("./a".to_string())),
                        ],
                        params: vec![
                            IdemParamType::KeyValue("mode".to_string(), IdemValueType::LitString("755".to_string()))
                        ]
                    })),
                    Box::new(IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
                        paths: vec![
                            IdemPath(None, IdemPathLocalPartType::File("./b".to_string())),
                        ],
                        params: vec![
                            IdemParamType::KeyValue("mode".to_string(), IdemValueType::LitString("600".to_string()))
                        ]
                    })),
                ]
            )
        );
    }

    #[test]
    fn test_parse_raw_command_with_block() {
        test_parser!(
            CompleteStr(&r#"
with ./dir/
    ./child/ (exists)
end
"#),
            parse_raw_command_with_block,
            IdemRawCommandType::WithBlock(
                IdemResourceType::Directory("./dir".to_string()),
                None,
                vec![
                    Box::new(IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
                        paths: vec![
                            IdemPath(None, IdemPathLocalPartType::Directory("./child".to_string())),
                        ],
                        params: vec![
                            IdemParamType::FlagKeyword("exists".to_string()),
                        ]
                    }))
                ]
            )
        );
    }

    #[test]
    fn test_parse_raw_script1() {
        test_parser!(
            CompleteStr(&r#"
each i in ./dir/
    ./a (mode="755")
    ./b (mode="600")
end
./x/ ./y/ (copied)
"#),
            parse_raw_script,
            vec![
                IdemRawCommandType::Each(
                    "i".to_string(),
                    IdemValueType::PathSpec(
                        IdemPath(None, IdemPathLocalPartType::Directory("./dir".to_string()))
                    ),
                    vec![
                        Box::new(IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
                            paths: vec![
                                IdemPath(None, IdemPathLocalPartType::File("./a".to_string())),
                            ],
                            params: vec![
                                IdemParamType::KeyValue("mode".to_string(), IdemValueType::LitString("755".to_string()))
                            ]
                        })),
                        Box::new(IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
                            paths: vec![
                                IdemPath(None, IdemPathLocalPartType::File("./b".to_string())),
                            ],
                            params: vec![
                                IdemParamType::KeyValue("mode".to_string(), IdemValueType::LitString("600".to_string()))
                            ]
                        })),
                    ]
                ),
                IdemRawCommandType::WithPaths(IdemRawCommandWithPaths {
                    paths: vec![
                        IdemPath(None, IdemPathLocalPartType::Directory("./x".to_string())),
                        IdemPath(None, IdemPathLocalPartType::Directory("./y".to_string())),
                    ],
                    params: vec![
                        IdemParamType::FlagKeyword("copied".to_string()),
                    ]
                }),
            ]
        );
    }

}

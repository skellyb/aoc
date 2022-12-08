use deno_bindgen::deno_bindgen;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, newline, not_line_ending, space1},
    combinator::map,
    multi::many1,
    sequence::{preceded, terminated, tuple},
    Finish, IResult, Parser,
};
use std::collections::HashMap;

#[deno_bindgen]
pub fn disk_usage(input: &str) -> i32 {
    let (_, output) = parse(input).finish().unwrap();
    let list = directory_list(&output);
    list.into_iter()
        .filter(|(_, size)| *size < 100000)
        .fold(0, |acc, (_, size)| acc + size)
}

#[deno_bindgen]
pub fn disk_free(input: &str) -> i32 {
    let total = 70000000;
    let required = 30000000;
    let (_, output) = parse(input).finish().unwrap();
    let list = directory_list(&output);
    let root = "/".to_string();
    let &used = list.get(&root).unwrap_or(&0);
    let min = required - (total - used);
    list.iter().fold(
        used,
        |acc, (_, &size)| if size >= min && size < acc { size } else { acc },
    )
}

fn directory_list(logs: &Vec<Output>) -> DirectoryList {
    let mut files = vec![];
    let mut list = HashMap::new();
    list.insert("/".to_string(), 0);
    let mut path: Vec<String> = vec![];
    for log in logs {
        match log {
            Output::CmdList => {}
            Output::CmdGoToRoot => {
                path.clear();
                path.push("/".to_string());
            }
            Output::CmdGoUp => {
                path.pop();
            }
            Output::CmdGoTo { name } => {
                path.push(name.clone());
            }
            Output::Directory { name } => {
                let mut p = path.to_vec();
                p.push(name.clone());
                list.insert(p.join("/"), 0);
            }
            Output::File { size, name } => {
                files.push(File {
                    name: name.clone(),
                    size: *size,
                    path: path.join("/"),
                });
            }
        }
    }
    for (path, size) in list.iter_mut() {
        *size = files
            .iter()
            .filter(|f| f.path.contains(path))
            .fold(0, |acc, f| acc + f.size);
    }
    list
}

type DirectoryList = HashMap<String, i32>;

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: i32,
    path: String,
}

#[derive(Debug, PartialEq)]
enum Output {
    CmdList,
    CmdGoToRoot,
    CmdGoUp,
    CmdGoTo { name: String },
    Directory { name: String },
    File { name: String, size: i32 },
}

type Logs = Vec<Output>;

fn parse(input: &str) -> IResult<&str, Logs> {
    many1(alt((command, dir, file))).parse(input)
}

fn command(input: &str) -> IResult<&str, Output> {
    preceded(tag("$ "), alt((list, change_dir))).parse(input)
}

fn change_dir(input: &str) -> IResult<&str, Output> {
    preceded(
        tag("cd "),
        alt((
            map(terminated(tag(".."), newline), |_| Output::CmdGoUp),
            map(terminated(tag("/"), newline), |_| Output::CmdGoToRoot),
            map(terminated(alphanumeric1, newline), |n: &str| {
                Output::CmdGoTo { name: n.to_owned() }
            }),
        )),
    )
    .parse(input)
}

fn list(input: &str) -> IResult<&str, Output> {
    map(terminated(tag("ls"), newline), |_| Output::CmdList).parse(input)
}

fn dir(input: &str) -> IResult<&str, Output> {
    preceded(
        tag("dir "),
        map(terminated(alphanumeric1, newline), |s: &str| {
            Output::Directory { name: s.to_owned() }
        }),
    )
    .parse(input)
}

fn file(input: &str) -> IResult<&str, Output> {
    map(
        tuple((
            terminated(digit1, space1),
            terminated(not_line_ending, newline),
        )),
        |(size, name): (&str, &str)| Output::File {
            name: name.to_owned(),
            size: size.parse().unwrap(),
        },
    )
    .parse(input)
}

#[test]
fn test_parse() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
    match parse(input).finish() {
        Ok((_, res)) => assert_eq!(res.len(), 23),
        Err(err) => panic!("{:?}", err.code),
    }
}

#[test]
fn test_cmd() {
    let (rest, result) = command(
        "$ cd ..
",
    )
    .finish()
    .unwrap();
    assert_eq!(result, Output::CmdGoUp);
    assert_eq!(rest, "");

    let (rest, result) = command(
        "$ cd aaa
",
    )
    .finish()
    .unwrap();
    assert_eq!(
        result,
        Output::CmdGoTo {
            name: "aaa".to_string()
        }
    );
    assert_eq!(rest, "");

    let (rest, result) = command(
        "$ cd /
",
    )
    .finish()
    .unwrap();
    assert_eq!(result, Output::CmdGoToRoot);
    assert_eq!(rest, "");

    let (rest, result) = command(
        "$ ls
",
    )
    .finish()
    .unwrap();
    assert_eq!(result, Output::CmdList);
    assert_eq!(rest, "");
}

#[test]
fn test_file() {
    let (rest, result) = file(
        "8033020 d.log
",
    )
    .finish()
    .unwrap();
    assert_eq!(
        result,
        Output::File {
            name: "d.log".to_string(),
            size: 8033020
        }
    );
    assert_eq!(rest, "");
}

#[test]
fn test_dir() {
    let (rest, result) = dir("dir abc
")
    .finish()
    .unwrap();
    assert_eq!(
        result,
        Output::Directory {
            name: "abc".to_string()
        }
    );
    assert_eq!(rest, "");
}

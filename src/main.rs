use failure::Error;
use std::env;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
use std::str;
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn write_output(bytes: &[u8], output_file: Option<&str>) -> std::io::Result<()> {
    if output_file.is_none() {
        let path = env::current_dir().unwrap();
        let output_file = format!(
            "{}.wasm",
            String::from(path.file_name().unwrap().to_str().unwrap())
        );
        let mut buffer = File::create(output_file)?;
        buffer.write_all(bytes)?;
    }
    Ok(())
}

mod ast;
mod compiler;
mod parser;

fn run(content: &str) -> Result<Vec<u8>, Error> {
    let app = parser::parse(content)?;
    compiler::compile(app)
}

fn main() -> Result<(), Error> {
    let matches = App::new("wasp")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(VERSION)
        .about("A lisp for web assembly")
        .author("Richard Anaya")
        .subcommand(
            SubCommand::with_name("build")
                .about("compile a wasp file")
                .arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short("v")
                        .help("Sets the level of verbosity"),
                )
                .arg(
                    Arg::with_name("emscripten")
                        .long("emscripten")
                        .short("e")
                        .help("Sets the level of verbosity"),
                ),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("initialize a directory to be a wasp project")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of project to create folder for")
                        .required(true),
                )
                .arg(
                    Arg::with_name("no-std")
                        .long("no-std")
                        .help("don't add the standard library"),
                ),
        )
        .subcommand(SubCommand::with_name("vendor").about("fetch dependencies"))
        .subcommand(
            SubCommand::with_name("add")
                .about("adds a dependency package to this project")
                .arg(
                    Arg::with_name("NAME")
                        .help("Sets the name of the dependency package")
                        .required(true),
                )
                .arg(
                    Arg::with_name("LOCATION")
                        .help("A git repo url or folder path")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("build") {
        use walkdir::WalkDir;

        let mut files = vec![];
        for entry in WalkDir::new("./") {
            let entry = entry.unwrap();
            let f = entry.path().display().to_string();
            if f.ends_with(".w") {
                let md = metadata(f.clone()).unwrap();
                if md.is_file() {
                    files.push(f);
                }
            }
        }

        let mut packages = vec![];

        if std::path::Path::new("project.wasp").exists() {
            let file = File::open("project.wasp")?;
            for line in BufReader::new(file).lines() {
                let l = line?;
                let v: Vec<&str> = l.split(' ').collect();
                packages.push(v[0].to_string())
            }
        }

        files.sort_by(|a, b| {
            if a.starts_with("./vendor/") {
                if b.starts_with("./vendor/") {
                    let sa = a.split('/').collect::<Vec<&str>>()[2];
                    let sb = b.split('/').collect::<Vec<&str>>()[2];
                    let pa = packages
                        .iter()
                        .position(|r| r == sa)
                        .unwrap_or(std::usize::MAX);
                    let pb = packages
                        .iter()
                        .position(|r| r == sb)
                        .unwrap_or(std::usize::MAX);
                    return pa.cmp(&pb);
                }
                return std::cmp::Ordering::Less;
            }
            std::cmp::Ordering::Equal
        });

        let mut contents = "".to_string();
        for file in files {
            let c = std::fs::read_to_string(&file).unwrap();
            contents = format!("{}\n{}", &contents, &c).to_string();
        }

        let output = run(&contents)?;
        write_output(&output, None)?;
        return Ok(());
    };

    if let Some(matches) = matches.subcommand_matches("init") {
        let folder = matches.value_of("NAME");
        if let Some(f) = folder {
            if !std::path::Path::new(&f).exists() {
                std::fs::create_dir(f)?;
                let mut file = File::create(format!("{}/{}", f, "main.w"))?;
                file.write_all(include_bytes!("static/main.w"))?;
                let mut file = File::create(format!("{}/{}", f, "project.wasp"))?;
                file.write_all(include_bytes!("static/project.wasp"))?;
                let mut file = File::create(format!("{}/{}", f, "index.html"))?;
                let mut idx = include_str!("static/index.html").to_string();
                idx = idx.replace("PROJECT_NAME", &f);
                file.write_all((&idx).as_bytes())?;
                let no_std = matches.is_present("no-std");
                if !no_std {
                    std::process::Command::new("git")
                        .args(&[
                            "clone",
                            "git@github.com:wasplang/std.git",
                            &format!("{}/vendor/{}", f, "std"),
                        ])
                        .output()
                        .expect("failed to execute process");
                    println!("added standard library");
                }
                println!("created package");
            } else {
                println!("directory \"{}\" already exists", f);
                std::process::exit(1);
            }
        }
        return Ok(());
    };

    if let Some(matches) = matches.subcommand_matches("add") {
        let name = matches.value_of("NAME").expect("no name");
        let location = matches.value_of("LOCATION").expect("no location");
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("project.wasp")
            .unwrap();

        if let Err(e) = writeln!(file, "{} {}", name, location) {
            eprintln!("Couldn't write to file: {}", e);
        }
        std::process::Command::new("git")
            .args(&["clone", location, &format!("vendor/{}", name)])
            .output()
            .expect("failed to execute process");
        println!("added dependency");
    }

    if matches.subcommand_matches("vendor").is_some() {
        std::fs::remove_dir_all("vendor")?;
        let file = File::open("project.wasp")?;
        for line in BufReader::new(file).lines() {
            let l = line?;
            let v: Vec<&str> = l.split(' ').collect();
            std::process::Command::new("git")
                .args(&["clone", v[1], &format!("vendor/{}", v[0])])
                .output()
                .expect("failed to execute process");
            println!("vendoring \"{}\"", v[0]);
        }
    }

    Ok(())
}

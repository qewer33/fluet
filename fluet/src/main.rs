/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate clap;

use std::io::{Read, Write};
use std::{fs::File, path::Path};

use anyhow::Result;
use clap::App;
use colored::*;
use common::value::Value;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let matches = App::new("fluet")
        .version("0.1")
        .author("TheOddGarlic")
        .arg(arg!([FILE] "File to be run"))
        .get_matches();

    let mut interpreter = Interpreter::new();
    if let Some(file) = matches.value_of("FILE") {
        if let Err(err) = run_file(file, &mut interpreter) {
            eprintln!("{}", err);
        }
    } else {
        if let Err(err) = run_prompt(&mut interpreter) {
            eprintln!("{}", err);
        }
    }
}

fn run_file<P>(path: P, interpreter: &mut Interpreter) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    match run(
        contents,
        path.to_str()
            .unwrap_or(&"<unknown>".green().italic())
            .to_string(),
        interpreter,
    ) {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    };
    Ok(())
}

fn run_prompt(interpreter: &mut Interpreter) -> Result<()> {
    let mut contents = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut contents)?;

        match run(
            contents.trim().to_string(),
            "<repl>".green().italic().to_string(),
            interpreter,
        ) {
            Ok(value) => println!("{}", value),
            Err(err) => eprintln!("{}", err),
        }

        contents.clear();
    }
}

fn run(code: String, filename: String, interpreter: &mut Interpreter) -> Result<Value> {
    let mut lexer = Lexer::new(code, filename.clone());
    let tokens = lexer.scan_tokens();

    let mut parser = Parser::new(tokens.to_vec());
    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(err) => {
            bail!(err);
        }
    };

    match interpreter.interpret(statements) {
        Ok(value) => Ok(value),
        Err(err) => bail!(err),
    }
}

use std::{
    fs::{create_dir, read_to_string, remove_dir_all, remove_file, File},
    io::Error,
    process::{self, Command},
};

use clap::Parser;
use compiler::{
    lexer::{lex, SymbolToken, Token},
    parser::parse_program,
};

mod compiler;

static TEMPORARY_FILE_DIR: &str = "./.temp";
static TEMPORARY_FILE_NAME: &str = "temp";

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(required = true)]
    input_file: String,

    #[clap(long)]
    lex: bool,

    #[clap(long)]
    parse: bool,

    #[clap(long)]
    codegen: bool,
}

fn main() {
    let args = Args::parse();

    // create folder for temporary files
    match create_dir(format!("{TEMPORARY_FILE_DIR}")) {
        Ok(_) => (),
        Err(_) => {
            remove_dir_all(format!("{TEMPORARY_FILE_DIR}")).unwrap();
            create_dir(format!("{TEMPORARY_FILE_DIR}")).unwrap();
        }
    }

    // call preprocessor
    match preprocess(&args) {
        Ok(_) => (),
        Err(_) => graceful_exit(10),
    };

    // compile (currently a stub!)
    match compile(&args) {
        Ok(_) => (),
        Err(_) => graceful_exit(10),
    };

    if !(args.lex || args.parse || args.codegen) {
        // call assembler and linker
        match assemble_and_link(&args) {
            Ok(_) => (),
            Err(_) => graceful_exit(10),
        };
    }

    graceful_exit(0);
}

fn graceful_exit(code: i32) {
    match remove_dir_all(format!("{TEMPORARY_FILE_DIR}")) {
        Ok(_) => (),
        Err(e) => eprintln!("Error occurred during cleanup. {e}"),
    }
    process::exit(code);
}

fn preprocess(args: &Args) -> Result<String, Error> {
    // preprocess and create the preprocessed file
    match Command::new("gcc")
        .args([
            "-E",
            "-P",
            &args.input_file,
            "-o",
            &format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.i"),
        ])
        .spawn()
    {
        Ok(mut child) => {
            child
                .wait()
                .expect("GCC child process failed while preprocessing for some reason");
        }
        Err(e) => {
            eprintln!("Preprocessing by gcc failed. Error: {e}");
            return Result::Err(e);
        }
    }
    Ok("Preprocess complete".to_string())
}

fn compile(args: &Args) -> Result<String, Error> {
    let code = read_to_string(&args.input_file).unwrap();
    // create the assembly file
    match File::create(format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.s")) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create the assembly file. Error: {e}");
            return Result::Err(e);
        }
    }

    let tokens = lex(code);
    eprintln!("{:?}", tokens);

    if args.lex {
        return Ok("Lexing only complete!".to_string());
    }

    fn is_not_comment_or_whitespace(token: &Token) -> bool {
        if let Token::Comment(_) = token {
            false
        } else if let Token::Symbol(SymbolToken::Whitespace) = token {
            false
        } else {
            true
        }
    }

    let mut tokens_to_parse = tokens
        .iter()
        .filter(|token| is_not_comment_or_whitespace(token));

    let _syntax_tree = parse_program(&mut tokens_to_parse);

    if args.parse {
        return Ok("Parsing only complete!".to_string());
    }

    // delete the preprocessed file
    match remove_file(format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.i")) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to delete the preprocessed file. Error: {e}");
            return Result::Err(e);
        }
    }
    Ok("Compilation complete!".to_string())
}

fn assemble_and_link(args: &Args) -> Result<String, Error> {
    // assemble and link the assembly file
    match Command::new("gcc")
        .args([
            &format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.s"),
            "-o",
            "./output",
        ])
        .spawn()
    {
        Ok(mut child) => {
            child
                .wait()
                .expect("GCC child process failed while assembling for some reason");
        }
        Err(e) => {
            eprintln!("Assembly and linking by gcc failed. Error: {e}");
            return Result::Err(e);
        }
    }

    // delete the assembly file
    match remove_file(format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.s")) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to delete the assembly file. Error: {e}");
            return Result::Err(e);
        }
    }
    Ok("Assembly and Linking complete".to_string())
}

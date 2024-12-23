use std::{
    fs::{create_dir, read_to_string, remove_dir_all, remove_file, File},
    io::{Error, Write},
    process::{self, Command},
};

use clap::Parser;
use compiler::{
    emitter::emit_program,
    generator::generate_program,
    lexer::{lex, SymbolToken, Token},
    parser::parse_program,
};
use tracing::{debug, error, info, warn};

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
    tracing_forest::init();

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

// derive the name of the executable from the path of the input file, with ".c" removed, ensure that there is only one ".c" in path otherwise fail
fn get_executable_name(src_path: &String) -> String {
    let filenames: Vec<&str> = src_path.split(".c").collect();
    assert_eq!(filenames.len(), 2);
    return String::from(filenames[0]);
}

#[tracing::instrument]
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
            info!("awaiting gcc preprocess");
            child
                .wait()
                .expect("GCC child process failed while preprocessing for some reason");
        }
        Err(e) => {
            error!("error in calling gcc: {e}");
            return Result::Err(e);
        }
    }
    info!("preprocessing complete");
    Ok("Preprocess complete".to_string())
}

#[tracing::instrument]
fn compile(args: &Args) -> Result<String, Error> {
    let code = read_to_string(&args.input_file).unwrap();
    // create the assembly file
    let mut assembly_file =
        match File::create(format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.s")) {
            Ok(f) => f,
            Err(e) => {
                error!("error in creating assembly file: {e}");
                return Result::Err(e);
            }
        };

    let tokens = lex(code);

    if args.lex {
        warn!("stopping at lex");
        debug!("Tokens lexed: {:?}", tokens);
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

    let syntax_tree = parse_program(&mut tokens_to_parse);

    if args.parse {
        warn!("stopping at parse");
        debug!("tree parsed: {:?}", syntax_tree);
        return Ok("Parsing only complete!".to_string());
    }

    let codegen = generate_program(syntax_tree);

    if args.codegen {
        warn!("stopping at codegen");
        debug!("code generated: {:?}", codegen);
        return Ok("Code Generation only complete!".to_string());
    }

    let mut buffer = String::new();
    emit_program(codegen, &mut buffer);

    assembly_file.write(buffer.as_bytes())?;

    // delete the preprocessed file
    match remove_file(format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.i")) {
        Ok(_) => (),
        Err(e) => {
            error!("error in deleting preprocessed file: {e}");
            return Result::Err(e);
        }
    }
    info!("compilation complete");
    Ok("Compilation complete!".to_string())
}

#[tracing::instrument]
fn assemble_and_link(args: &Args) -> Result<String, Error> {
    // assemble and link the assembly file
    let executable_name = get_executable_name(&args.input_file);
    match Command::new("gcc")
        .args([
            &format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.s"),
            "-o",
            &format!("{executable_name}"),
        ])
        .spawn()
    {
        Ok(mut child) => {
            child
                .wait()
                .expect("GCC child process failed while assembling for some reason");
        }
        Err(e) => {
            error!("error in gcc assembly step: {e}");
            return Result::Err(e);
        }
    }

    // delete the assembly file
    match remove_file(format!("{TEMPORARY_FILE_DIR}/{TEMPORARY_FILE_NAME}.s")) {
        Ok(_) => (),
        Err(e) => {
            error!("error while deleting the assembly file: {e}");
            return Result::Err(e);
        }
    }
    info!("assembly and linking complete");
    Ok("Assembly and Linking complete".to_string())
}

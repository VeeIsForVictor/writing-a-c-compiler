#[macro_use]
extern crate lazy_static;

use std::{
    fs::{create_dir, read_to_string, remove_dir_all, remove_file, File},
    io::{Error, Write},
    process::{self, Command},
};

use clap::Parser;
use compiler::{
    emitter::emit_program, generator::generate_program, lexer::lex, parser::parse_program,
    tacker::tack_program,
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

    #[clap(short('S'), help("Keep the emitted assembly (.s) file"))]
    keep_assembly: bool,

    #[clap(long, help("Compile only until the lexing stage"))]
    lex: bool,

    #[clap(long, help("Compile only until the parsing stage"))]
    parse: bool,

    #[clap(long, help("Compile only until the TACKY generation stage"))]
    tacky: bool,

    #[clap(long, help("Compile only until the code generation stage"))]
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

    if !(args.lex || args.parse || args.codegen || args.tacky) {
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
#[tracing::instrument(ret)]
fn get_executable_name(src_path: &String) -> String {
    let filenames: Vec<&str> = src_path.split(".c").collect();
    assert_eq!(filenames.len(), 2);
    return String::from(filenames[0]);
}

#[tracing::instrument]
fn preprocess(args: &Args) -> Result<String, Error> {
    // preprocess and create the preprocessed file
    let executable_name = get_executable_name(&args.input_file);

    match Command::new("gcc")
        .args([
            "-E",
            "-P",
            &args.input_file,
            "-o",
            &format!("{executable_name}.i"),
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

#[tracing::instrument(skip_all)]
fn compile(args: &Args) -> Result<String, Error> {
    let executable_name = get_executable_name(&args.input_file);
    let preprocessed_name = format!("{executable_name}.i");
    let code = read_to_string(preprocessed_name).unwrap();

    let tokens = lex(code);

    if args.lex {
        warn!("stopping at lex");
        debug!("Tokens lexed: {:?}", tokens);
        return Ok("Lexing only complete!".to_string());
    }

    let syntax_tree = parse_program(&mut tokens.iter().peekable());

    if args.parse {
        warn!("stopping at parse");
        debug!("tree parsed: {:?}", syntax_tree);
        return Ok("Parsing only complete!".to_string());
    }

    let tacky = tack_program(syntax_tree);

    if args.tacky {
        warn!("stopping at tacking");
        debug!("tacky generated: {:?}", tacky);
        return Ok("Tacky Generation only complete!".to_string());
    }

    let codegen = generate_program(tacky);

    if args.codegen {
        warn!("stopping at codegen");
        debug!("code generated: {:?}", codegen);
        return Ok("Code Generation only complete!".to_string());
    }

    let mut buffer = String::new();
    emit_program(codegen, &mut buffer);

    // create the assembly file
    let assembly_filename = get_executable_name(&args.input_file);

    let mut assembly_file = match File::create(format!("{assembly_filename}.s")) {
        Ok(f) => f,
        Err(e) => {
            error!("error in creating assembly file: {e}");
            return Result::Err(e);
        }
    };

    warn!("assembly file {assembly_filename}.s created");
    assembly_file.write(buffer.as_bytes())?;

    // delete the preprocessed file
    match remove_file(format!("{assembly_filename}.i")) {
        Ok(_) => (),
        Err(e) => {
            error!("error in deleting preprocessed file: {e}");
            return Result::Err(e);
        }
    }
    info!("compilation complete");
    Ok("Compilation complete!".to_string())
}

#[tracing::instrument(skip_all)]
fn assemble_and_link(args: &Args) -> Result<String, Error> {
    // assemble and link the assembly file
    let executable_name = get_executable_name(&args.input_file);
    match Command::new("gcc")
        .args([
            &format!("{executable_name}.s"),
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

    if !args.keep_assembly {
        // delete the assembly file
        warn!("deleting assembly file");
        match remove_file(format!("{executable_name}.s")) {
            Ok(_) => (),
            Err(e) => {
                error!("error while deleting the assembly file: {e}");
                return Result::Err(e);
            }
        }
    }

    info!("assembly and linking complete");
    Ok("Assembly and Linking complete".to_string())
}

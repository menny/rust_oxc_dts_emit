use std::error::Error;
use std::{env, path::Path};

use oxc::allocator::Allocator;
use oxc::codegen::CodeGenerator;
use oxc::isolated_declarations::{IsolatedDeclarations, IsolatedDeclarationsOptions};
use oxc::parser::Parser;
use oxc::span::SourceType;

fn _emit_dts(
    allocator: &Allocator,
    contents: &str,
    source_type: SourceType,
) -> Result<String, std::io::Error> {
    let ret = Parser::new(allocator, contents, source_type).parse();
    if !ret.errors.is_empty() {
        let mut error_messages = String::new();
        for error in ret.errors {
            error_messages.push_str(&error.with_source_code(contents.to_owned()).to_string());
        }
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_messages,
        ));
    }

    let id_ret = IsolatedDeclarations::new(allocator, IsolatedDeclarationsOptions {
        strip_internal: true,
    })
    .build(&ret.program);
    let dts_content = CodeGenerator::new().build(&id_ret.program).code;

    if !id_ret.errors.is_empty() {
        let mut error_messages = String::new();
        for error in id_ret.errors {
            error_messages.push_str(&error.with_source_code(contents.to_owned()).to_string());
        }
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_messages,
        ));
    }

    Ok(dts_content)
}

fn main() -> Result<(), Box<dyn Error>> {
    // the first cell is the path to the executable
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Usage: oxc_dts_emit [<input_file>:<output_file>]...");
        return Err("Incorrect number of arguments".into());
    }

    let allocator = Allocator::default();
    let success = args
        .iter()
        .map(|input_output_pair| input_output_pair.split_once(":"))
        .map(|op| {
            op.expect("Must delimit with ':'! Usage: oxc_dts_emit [<input_file>:<output_file>]...")
        })
        .map(|chunk| (Path::new(chunk.0), Path::new(chunk.1))) // Convert to &str
        .map(|(input_path, output_path)| {
            std::fs::read_to_string(input_path)
                .map(|contents| (contents, SourceType::from_path(input_path).unwrap()))
                .and_then(|data| _emit_dts(&allocator, &data.0, data.1))
                .and_then(|dts_contents| std::fs::write(output_path, dts_contents))
                .map_err(|e| {
                    eprintln!("Error processing {}: {}", input_path.display(), e);
                    e
                })
        })
        .all(|r| r.is_ok());

    if success {
        Ok(())
    } else {
        Err("dts emit failure".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_dts_with_valid_typescript() {
        let allocator = Allocator::default();
        let contents = String::from("export const pony: string = 'Pinkie Pie';");
        let source_type = SourceType::from_path(Path::new("pony.ts")).unwrap();

        let result = _emit_dts(&allocator, &contents, source_type);
        assert!(result.is_ok(), "Expected successful dts generation");

        let dts_content = result.unwrap();
        assert!(
            dts_content.contains("export declare const pony: string"),
            "Generated .d.ts should contain the exported declaration"
        );
    }

    #[test]
    fn test_emit_dts_with_untype_typescript() {
        let allocator = Allocator::default();
        let contents = String::from("export const pony = getUnknownObject();");
        let source_type = SourceType::from_path(Path::new("pony.ts")).unwrap();

        let result = _emit_dts(&allocator, &contents, source_type);
        assert!(result.is_err(), "Expected error for invalid input");
    }
}

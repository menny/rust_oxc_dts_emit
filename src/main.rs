use std::env;
use std::error::Error;

fn _emit_dts(contents: &str) -> Result<String, std::io::Error> {
    // Perform modification and return the modified string or error
    let modified_contents = contents.replace("A", "B"); // Example modification
    Ok(modified_contents)

    // Example of returning an error:
    // Err("Something went wrong!".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    // the first cell is the path to the executable
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 || args.len() % 2 != 0 {
        println!("Usage: oxc_dts_emit [pairs of <input_file> <output_file>]");
        return Err("Incorrect number of arguments".into());
    }

    let success = args.chunks_exact(2) // Iterate over pairs directly
        .map(|chunk| (&chunk[0], &chunk[1])) // Convert to &str
        .map(|(input_path, output_path)| {
            std::fs::read_to_string(input_path)
              .and_then(|contents| _emit_dts(&contents))
              .and_then(|dts_contents| std::fs::write(&output_path, dts_contents))
              .map_err(|e| {
                  eprintln!("Error processing {}: {}", input_path, e);
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

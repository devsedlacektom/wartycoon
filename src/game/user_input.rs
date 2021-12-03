use std::io::stdin;

/// Retrieve a line from the stdin
///
/// Returns
/// ---
/// - String: line from the input
pub fn get_line() -> String {
    let mut input_text = String::new();

    loop {
        match stdin().read_line(&mut input_text) {
            Ok(_) => break,
            Err(_) => println!("\nCannot read from command line, try again!\n"),
        }
    }

    input_text
}

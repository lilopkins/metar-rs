use std::io::{self, Write};

use metar::Metar;

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    loop {
        print!("METAR> ");
        io::stdout().flush().expect("failed to flush");

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => {
                // EOF
                break;
            }
            Ok(_) => {
                let input = line.trim();
                if input.eq_ignore_ascii_case("exit") {
                    break;
                }

                match Metar::parse(input) {
                    Ok(metar) => println!("{metar:#?}"),
                    Err(es) => {
                        for e in es {
                            eprintln!("{e}");
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading: {err}");
                break;
            }
        }
    }

    Ok(())
}

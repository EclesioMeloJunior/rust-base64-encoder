mod alphabet;
mod encoder;
mod decoder;

use std::io::{self, Read};

enum CLIError {
    TooFewArguments,
    InvalidSubCommand(String),
    StdInUnreadable,
    DecodingError,
}

impl std::fmt::Debug for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::TooFewArguments => write!(f, "Not enough arguments provided"),
            Self::InvalidSubCommand(cmd) => write!(f, "Invalid subcommand provided: \"{}\"", cmd),
            Self::StdInUnreadable => write!(f, "Unable to read stdin"),
            Self::DecodingError => write!(f, "An error occured while decoding the data"),
        }
    }
}

fn read_stdin() -> Result<String, CLIError> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|_| CLIError::StdInUnreadable)?;

    Ok(input.trim().to_string())
}

fn encode(input: &String) -> String {
    encoder::encode(input.as_bytes())
}

fn decode(input: &String) -> Result<String, CLIError> {
    let decoded = decoder::decode(input)
        .map_err(|_| CLIError::DecodingError)?;
    
    let decoded_as_string = std::str::from_utf8(&decoded)
        .map_err(|_| CLIError::DecodingError)?;

    Ok(decoded_as_string.to_owned())
}

fn main() -> Result<(), CLIError>{
    if std::env::args().count() < 2 {
        return Err(CLIError::TooFewArguments);
    }

    let subcommand = std::env::args()
        .nth(1)
        .ok_or_else(|| CLIError::TooFewArguments)?;

    let input = read_stdin()?;

    let output = match subcommand.as_str() {
        "encode" => Ok(encode(&input)),
        "decode" => Ok(decode(&input)?),
        cmd => Err(CLIError::InvalidSubCommand(cmd.to_string()))
    }?;

    print!("{}", output);
    Ok(())
}

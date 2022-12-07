use std::fs::File;
use std::io::*;
use std::result::Result;
use crate::errors::PhlParseError;
use crate::phl::tokenizer::{Definition, tokenize_definition};

fn parse_iter(statements: impl Iterator<Item=String>) -> Result<Vec<Definition>, PhlParseError>{
    let results: Vec<_> = statements
        .filter(|l| l.trim().len() > 0)
        .map(|l| tokenize_definition(l.as_str()))
        .collect();
    for problem in results.iter().filter_map(|d| d.as_ref().err()) {
        return Err(problem.clone());
    }
    let definitions: Vec<_> = results
        .into_iter()
        .filter_map(|d| d.ok())
        .filter_map(|d| d)
        .collect();
    Ok(definitions)
}

pub(crate) fn parse_lines(lines: &Vec<&str>) -> Result<Vec<Definition>, PhlParseError> {
    let results: Vec<_> = lines
        .into_iter()
        .map(|l| tokenize_definition(l))
        .collect();
    for problem in results.iter().filter_map(|d| d.as_ref().err()) {
        return Err(problem.clone())
    }
    let definitions = lines
        .iter()
        .filter_map(|l| tokenize_definition(l).ok())
        .filter_map(|d| d)
        .collect();
    Ok(definitions)
}

pub(crate) fn parse_file(filepath: &str) -> Result<Vec<Definition>, PhlParseError> {
    let file = match File::open(filepath) {
        Ok(f) => Ok(f),
        Err(e) => Err(PhlParseError::FileReadError(e.to_string()))
    }?;
    let reader = BufReader::new(file);
    parse_buffer(reader)
}

fn parse_buffer<T: std::io::Read>(reader: BufReader<T>) -> Result<Vec<Definition>, PhlParseError> {
    let lines = reader.lines().filter_map(|l| l.ok());
    parse_iter(lines)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::phl::parser::{parse_file, parse_lines};

    #[test]
    fn test_parse_lines() {
        let test_cases = [
            (
                vec![
                    "<default> = [0labial, 0labiodental, 0coronal, 0dorsal, 0lateral, 0sonorant, 0approximant, 0continuant, 0syllabic, 0consonantal, 0delayedrelease, 0voice]",
                    "<labiodental> = [+labial, +labiodental, -coronal, -dorsal, -lateral]",
                ],
                ()
            )
        ];
        for (lines, ()) in test_cases {
            let parsed = parse_lines(&lines).unwrap();
            assert_eq!(parsed.len(), 2)
        }
    }

    #[test]
    fn test_parse_file() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test_file = path.join("../../../assets/systems/hayes-ipa-arpabet.phl");
        let test_file =  test_file.to_str().unwrap();

        let parsed = parse_file(test_file).unwrap();
        assert_eq!(parsed.len(), 220);
    }
}
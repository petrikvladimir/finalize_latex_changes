extern crate failure;

use failure::Error;

use std::path::Path;
use std::fs::File;

enum Steps {
    DeleteUntilOpeningBracket,
    DeleteUntilClosingBracket,
    KeepUntilClosingBracket,
}

#[derive(Default)]
pub struct Filter {
    counter_added: usize,
    counter_replaced: usize,
    counter_deleted: usize,

    open_brackets: usize,
    reversed_steps: Vec<Steps>,
}

impl Filter {
    pub fn new() -> Filter {
        Filter { ..Default::default() }
    }

    pub fn process(&mut self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());

        for (i, c) in text.chars().enumerate() {
            let step = self.reversed_steps.pop();
            match step {
                None => {
                    if text[i..].starts_with("\\added") {
                        self.counter_added += 1;
                        self.reversed_steps.push(Steps::KeepUntilClosingBracket);
                        self.reversed_steps.push(Steps::DeleteUntilOpeningBracket);
                    } else if text[i..].starts_with("\\deleted") {
                        self.counter_deleted += 1;
                        self.reversed_steps.push(Steps::DeleteUntilClosingBracket);
                        self.reversed_steps.push(Steps::DeleteUntilOpeningBracket);
                    } else if text[i..].starts_with("\\replaced") {
                        self.counter_replaced += 1;
                        self.reversed_steps.push(Steps::DeleteUntilClosingBracket);
                        self.reversed_steps.push(Steps::DeleteUntilOpeningBracket);
                        self.reversed_steps.push(Steps::KeepUntilClosingBracket);
                        self.reversed_steps.push(Steps::DeleteUntilOpeningBracket);
                    } else {
                        out.push(c);
                    }
                }
                Some(Steps::DeleteUntilOpeningBracket) => {
                    if c != '{' {
                        self.reversed_steps.push(step.unwrap());
                    }
                    self.open_brackets = 0;
                }
                Some(Steps::KeepUntilClosingBracket) => {
                    if c == '{' {
                        self.open_brackets += 1;
                    } else if c == '}' {
                        if self.open_brackets == 0 {
                            continue; //mission completed
                        }
                        self.open_brackets -= 1;
                    }
                    self.reversed_steps.push(step.unwrap());
                    out.push(c);
                }
                Some(Steps::DeleteUntilClosingBracket) => {
                    if c == '{' {
                        self.open_brackets += 1;
                    } else if c == '}' {
                        if self.open_brackets == 0 {
                            continue; //mission completed
                        }
                        self.open_brackets -= 1;
                    }
                    self.reversed_steps.push(step.unwrap());
                }
            }
        }
        out
    }

    pub fn process_file(&mut self, input_file: &Path, output_file: &Path) -> Result<(), Error> {
        use std::io::{BufReader, BufWriter, BufRead, Write};

        if input_file.eq(output_file) {
            return Err(failure::err_msg("Input and output file cannot be equal."));
        }

        let reader = BufReader::new(File::open(input_file)?);
        let mut wbuf = BufWriter::new(File::create(output_file)?);
        reader.lines().try_for_each(|l| {
            writeln!(wbuf, "{}", self.process(&l?)).map(|_| ())
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let f = Filter { ..Default::default() };
        assert_eq!(f.counter_added, 0);
        assert_eq!(f.counter_replaced, 0);
        assert_eq!(f.counter_deleted, 0);
    }

    #[test]
    fn test_added() {
        let mut f = Filter { ..Default::default() };
        assert_eq!(f.process("\\added{added_text}"), "added_text");
        assert_eq!(f.process("asdf\\added{added_text}adsf"), "asdfadded_textadsf");
        assert_eq!(f.process("asdf \\added{added_text}adsf"), "asdf added_textadsf");
        assert_eq!(f.process("asdf \\add ed{added_text}adsf"), "asdf \\add ed{added_text}adsf");
        assert_eq!(f.process("asdf \\added{added{}_text\\italic{adfs}}adsf"), "asdf added{}_text\\italic{adfs}adsf");
        assert_eq!(f.process("\\added{added_text}\\added{added_text}"), "added_textadded_text");
        assert_eq!(f.counter_added, 6);
    }

    #[test]
    fn test_deleted() {
        let mut f = Filter { ..Default::default() };
        assert_eq!(f.process("\\deleted{wrong text}"), "");
        assert_eq!(f.process("\\deleted{wrong text}goal"), "goal");
        assert_eq!(f.process("goal\\deleted{wrong text}"), "goal");
        assert_eq!(f.process("go\\deleted{wrong text}al"), "goal");
        assert_eq!(f.process("\\deleted{wrong \\textit{another of \\textbf{another}} text}goal"), "goal");
        assert_eq!(f.counter_deleted, 5);
    }

    //
    #[test]
    fn test_replaced() {
        let mut f = Filter { ..Default::default() };
        assert_eq!(f.process("\\replaced{good}{wrong}"), "good");
        assert_eq!(f.process("\\replaced{goo}{wrong}d"), "good");
        assert_eq!(f.process("g\\replaced{ood}{wrong}"), "good");
        assert_eq!(f.process("\\replaced{good}{wr\\textit{asdf}ong}"), "good");
        assert_eq!(f.process("\\replaced{g\\textit{asd}ood}{wr\\textit{asdf}ong}"), "g\\textit{asd}ood");
        assert_eq!(f.counter_replaced, 5);
    }

    #[test]
    fn test_process_file_same_file() {
        let mut f = Filter::new();
        let out = f.process_file(Path::new("asdf"), Path::new("asdf"));
        assert!(out.is_err());
    }
}

extern crate failure;

use failure::Error;

use std::path::Path;
use std::fs::File;

#[cfg(test)]
mod tests;

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
    /// Create a filter instance.
    pub fn new() -> Filter {
        Filter { ..Default::default() }
    }

    /// Return an string with all 'changes' artifacts removed from the input text.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut f = finalize_latex_changes::Filter::new();
    /// let out = f.process("\\added{asdf}");
    /// assert_eq!(out, "asdf");
    /// ```
    pub fn process(&mut self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        for (i, c) in text.chars().enumerate() {
            let step = self.reversed_steps.pop();
            match step {
                None => {
                    if !self.create_steps_for_command(&text[i..]) {
                        out.push(c);
                    }
                }
                Some(Steps::DeleteUntilOpeningBracket) => {
                    if c != '{' {
                        self.reversed_steps.push(step.unwrap());
                    } else {
                        self.open_brackets = 1;
                    }
                }
                Some(Steps::KeepUntilClosingBracket) => {
                    if self.count_open_brackets(c) != 0 {
                        self.reversed_steps.push(step.unwrap());
                        out.push(c);
                    }
                }
                Some(Steps::DeleteUntilClosingBracket) => {
                    if self.count_open_brackets(c) != 0 {
                        self.reversed_steps.push(step.unwrap());
                    }
                }
            }
        }
        out
    }

    /// If the slice s starts with 'changes' command, then:
    ///     - create steps for the command processing
    ///     - increase counter
    ///     - return true
    ///
    /// Possible commands: "\added", "\deleted", "\replaced"
    fn create_steps_for_command(&mut self, s: &str) -> bool {
        if s.starts_with("\\added") {
            self.counter_added += 1;
            self.reversed_steps.push(Steps::KeepUntilClosingBracket);
            self.reversed_steps.push(Steps::DeleteUntilOpeningBracket);
            return true;
        } else if s.starts_with("\\deleted") {
            self.counter_deleted += 1;
            self.reversed_steps.push(Steps::DeleteUntilClosingBracket);
            self.reversed_steps.push(Steps::DeleteUntilOpeningBracket);
            return true;
        } else if s.starts_with("\\replaced") {
            self.counter_replaced += 1;
            self.reversed_steps.push(Steps::DeleteUntilClosingBracket);
            self.reversed_steps.push(Steps::DeleteUntilOpeningBracket);
            self.reversed_steps.push(Steps::KeepUntilClosingBracket);
            self.reversed_steps.push(Steps::DeleteUntilOpeningBracket);
            return true;
        }
        false
    }

    /// Count number of open brackets in a sequence.
    fn count_open_brackets(&mut self, c: char) -> usize {
        match c {
            '{' => self.open_brackets += 1,
            '}' if self.open_brackets == 0 => {}
            '}' => self.open_brackets -= 1,
            _ => {}
        }
        self.open_brackets
    }

    /// Read input file line by line and write processed line into output.
    /// Input and output file must not be equal.
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

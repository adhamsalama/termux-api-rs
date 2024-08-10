#[derive(Debug)]
pub enum TermuxError {
    IOError(std::io::Error),
    Output(std::process::Output),
}

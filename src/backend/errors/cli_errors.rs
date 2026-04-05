use std::fmt::Display;

#[derive(Debug)]
pub enum CommandLineError {
    BuildHasJustTwoArg,
    NoFileSpecifiedForBuild,
    NoSuchCommand,
    ErrorCreatingDirectory,
    ErrorCreatingFile,
    InvalidCommand,
    ErrorFindingDirectory,
}

impl Display for CommandLineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandLineError::BuildHasJustTwoArg => write!(f, "Build only has two arguments"),
            CommandLineError::NoFileSpecifiedForBuild => write!(f, "No file specified for build"),
            CommandLineError::NoSuchCommand => write!(f, "Command does not exist"),
            CommandLineError::ErrorCreatingDirectory => {
                write!(f, "Error occured while creating directory")
            }
            CommandLineError::ErrorCreatingFile => write!(f, "Error occured while creating file"),
            CommandLineError::InvalidCommand => write!(
                f,
                "Command not formmated correctly, use 'vertex help' for more information"
            ),
            CommandLineError::ErrorFindingDirectory => {
                write!(f, "Error occured while finding directory")
            }
        }
    }
}

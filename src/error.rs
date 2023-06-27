#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Cannot interact with user!")]
    CannotInteract,

    #[error("Cannot read user input!")]
    CannotReadUserInput,

    #[error("Invalid arguments")]
    InvalidArguments,

    #[error("Invalid digit")]
    InvalidDigit,

    #[error("No items to choose from!")]
    NoItemsToChooseFrom,

    #[error(transparent)]
    Generic(#[from] anyhow::Error)
}

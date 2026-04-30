use thiserror::Error;

#[derive(Debug, Error)]
pub enum LinkerError {
    #[error("Linker error: Cyclic import with{imported}->{from}")]
    CyclicImport { imported: String, from: String },
    #[error("Linker error: Missing lib import at {imported}->{from}")]
    MissingImport { imported: String, from: String },
}

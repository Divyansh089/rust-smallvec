#[cfg(feature = "defmt")]
use defmt::{
    Format as DeFormat,
    Formatter as DeFormatter,
    write as dewrite
};
use {
    alloc::alloc::handle_alloc_error,
    core::{
        alloc::Layout,
        error::Error,
        fmt::{
            Debug,
            Display,
            Formatter,
            Result as Format
        }
    }
};

#[derive(Debug)]
pub enum SmallVecError {
    CapacityOverflow,
    AllocationError(Layout)
}

impl Display for SmallVecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Format {
        write!(f, "Allocation error: {self:?}")
    }
}

impl Error for SmallVecError {}

// `Layout` doesn't implement `defmt::Format`, so this can't be derived.
#[cfg(feature = "defmt")]
impl DeFormat for SmallVecError {
    fn format(&self, fmt: DeFormatter) {
        match self {
            SmallVecError::CapacityOverflow => dewrite!(fmt, "CapacityOverflow"),
            SmallVecError::AllocationError(layout) => dewrite!(
                fmt,
                "AllocationError(Layout {{ size: {=usize}, align: {=usize} }})",
                layout.size(),
                layout.align()
            )
        }
    }
}

impl SmallVecError {
    #[cold]
    #[inline(never)]
    #[track_caller]
    pub fn handle<Type>(self) -> Type {
        match self {
            SmallVecError::CapacityOverflow => panic!("capacity overflow"),
            SmallVecError::AllocationError(layout) => handle_alloc_error(layout)
        }
    }
}

//! Common traits module


/// From env feature module
#[cfg(feature = "from_env")]
pub mod from_env {

    use std::convert::TryFrom;
    use std::env::var;

    /// Build an object from an environment value content
    pub trait FromEnv: TryFrom<String> {
        /// Default environment value to use
        fn default_env() -> &'static str;

        /// Try building object from a variable
        fn from_env(variable: &str) -> Result<Self, Self::Error> {
            Self::try_from(var(variable)
                .unwrap_or_else(|_| panic!("Variable {:?} is not set", variable)))
        }

        /// Try building object from the default variable
        fn from_default_env() -> Result<Self, Self::Error> {
            Self::from_env(Self::default_env())
        }
    }
}
use std::any::Any;
use std::fmt::Debug;

#[allow(unused_macros)]
macro_rules! match_value {
    ($value:expr, $f:expr, $(rule = $type:ty;)+) => {
        $(
            if let Some(v) = $value.downcast_ref::<$type>() {
                write!($f, " | Default: `{}`", v)?;
            } else
        )+ 
        {
            write!($f, " | Default: [unknown type]")?;
        }
    };
}

pub mod dyna {
    use super::*;

    #[allow(dead_code)]
    pub enum FlagError {
        NoValue,
    }

    #[allow(unreachable_patterns)]
    impl Debug for FlagError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::NoValue => {
                    write!(f, "Found no value in Flag")
                }
                _ => Ok(()),
            }
        }
    }

    pub struct Flag<'a> {
        pub name: &'a str,
        pub args: &'a [&'a str],
        pub desc: &'a str,
        pub notes: Option<&'a str>,
        pub value: Option<Box<dyn Any>>,
    }

    impl<'a> Debug for Flag<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}\n\t", self.name)?;
            for arg in self.args {
                write!(f, "{} ", arg)?;
            }
            write!(f, "| {}", self.desc)?;
            if let Some(notes) = self.notes {
                write!(f, " | {}", notes)?;
            }
            if let Some(value) = self.value.as_ref() {
                match_value!( value, f,
                    rule = &'static str;
                    rule = &'static i32;
                    rule = &'static f32;
                )
            }
            Ok(())
        }
    }

    #[allow(dead_code)]
    impl<'a> Flag<'a> {
        pub fn new(
            name: &'a str,
            args: &'a [&'a str],
            desc: &'a str,
            notes: Option<&'a str>,
            value: Option<Box<dyn Any>>,
        ) -> Self {
            Self {
                name,
                args,
                desc,
                notes, // Note examples: `To be deprecated`, `Not implimented`, `Developer use only`
                value,
            }
        }

        pub fn get_name(&self) -> &'a str {
            self.name
        }

        pub fn get_args(&self) -> &'a [&'a str] {
            self.args
        }

        pub fn get_value(&self) -> Option<&Box<dyn Any>> {
            self.value.as_ref()
        }

        pub fn set_value(&mut self, value: &'static dyn Any) -> Result<(), FlagError> {
            if self.value.is_none() {
                return Err(FlagError::NoValue);
            }
            self.value = Some(Box::new(value));
            Ok(())
        }

        pub fn is_in(&self, s: &str) -> bool {
            self.args.contains(&s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import the library components

    #[test]
    fn test_flag_creation() {
        let flag = dyna::Flag {
            name: "test",
            args: &["-t", "--test"],
            desc: "A test flag",
            notes: Some("Optional notes"),
            value: Some(Box::new(42) as Box<dyn Any>),
        };

        assert_eq!(flag.name, "test");
        assert_eq!(flag.args, &["-t", "--test"]);
        assert_eq!(flag.desc, "A test flag");
        assert_eq!(flag.notes, Some("Optional notes"));

        // Check the value
        if let Some(value) = flag.value.as_ref() {
            if let Some(v) = value.downcast_ref::<i32>() {
                assert_eq!(*v, 42);
            } else {
                panic!("Flag value type mismatch!");
            }
        } else {
            panic!("Flag value is missing!");
        }
    }

    #[test]
    fn test_flag_without_value() {
        let flag = dyna::Flag {
            name: "help",
            args: &["-h", "--help"],
            desc: "Display help information",
            notes: None,
            value: None,
        };

        assert_eq!(flag.name, "help");
        assert_eq!(flag.args, &["-h", "--help"]);
        assert_eq!(flag.desc, "Display help information");
        assert!(flag.notes.is_none());
        assert!(flag.value.is_none());
    }

    #[test]
    fn test_flag_debug_format() {
        let flag = dyna::Flag {
            name: "output",
            args: &["-o", "--output"],
            desc: "Specify the output file",
            notes: None,
            value: Some(Box::new("output.txt") as Box<dyn Any>),
        };

        let debug_output = format!("{:?}", flag);

        dbg!(&debug_output);

        assert!(debug_output.contains("output"));
        assert!(debug_output.contains("-o"));
        assert!(debug_output.contains("--output"));
        assert!(debug_output.contains("Specify the output file"));
        assert!(debug_output.contains("Default: `output.txt`"));
    }

    #[test]
    fn test_flag_with_bool_value() {
        let flag = dyna::Flag {
            name: "verbose",
            args: &["-v", "--verbose"],
            desc: "Enable verbose output",
            notes: Some("Useful for debugging"),
            value: Some(Box::new(true) as Box<dyn Any>),
        };

        if let Some(value) = flag.value.as_ref() {
            if let Some(v) = value.downcast_ref::<bool>() {
                assert_eq!(*v, true);
            } else {
                panic!("Flag value type mismatch!");
            }
        } else {
            panic!("Flag value is missing!");
        }
    }
}


use core::fmt::Display;
use core::str::FromStr;

use clap::ValueEnum;
use semver::{BuildMetadata, Prerelease, Version};

use crate::{error::ErrorKind, Error};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum IncrementLevel {
    #[default]
    Patch,
    Minor,
    Major,
}

impl Display for IncrementLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IncrementLevel::Patch => "patch",
                IncrementLevel::Minor => "minor",
                IncrementLevel::Major => "major",
            }
        )
    }
}

impl FromStr for IncrementLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "patch" => Ok(Self::Patch),
            "minor" => Ok(Self::Minor),
            "major" => Ok(Self::Major),
            _ => Err(Error::new(ErrorKind::InvalidIncrementLevel(s.to_string()))),
        }
    }
}

pub trait Increment {
    fn increment(&mut self, level: IncrementLevel) -> &mut Self;
}

impl Increment for Version {
    fn increment(&mut self, level: IncrementLevel) -> &mut Self {
        self.build = BuildMetadata::EMPTY;
        self.pre = Prerelease::EMPTY;
        self.patch += 1;
        if level > IncrementLevel::Patch {
            self.patch = 0;
            self.minor += 1;
        }
        if level > IncrementLevel::Minor {
            self.minor = 0;
            self.major += 1;
        }
        self
    }
}

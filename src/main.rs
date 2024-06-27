use std::{
    error,
    io::{stdin, IsTerminal},
};

use semver_extra::{semver::Version, Increment, IncrementLevel};

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

const BINARY_NAME: &str = "semver";

#[derive(Debug, Parser)]
#[command(name = BINARY_NAME, author, version)]
/// A Rust implementation of the https://semver.org/ specification
struct Cli {
    /// The input semantic version. If omitted, input is taken from stdin.
    #[arg(name = "VERSION")]
    version: Option<Version>,

    #[command(subcommand)]
    command: Option<SemverCommand>,
}

#[derive(Debug, Subcommand)]
enum SemverCommand {
    /// Increment a component of the version, resetting those of lower significance.
    #[command(visible_alias = "i")]
    Increment {
        #[arg(default_value_t = IncrementLevel::Patch)]
        level: IncrementLevel,
    },

    /// Output a specific component of the version.
    #[command(visible_alias = "g")]
    Get { component: VersionComponent },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum VersionComponent {
    Major,
    Minor,
    Patch,
    Prerelease,
    BuildMetadata,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    let mut version: Version = if let Some(version) = cli.version {
        version
    } else {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        buffer.trim().parse()?
    };

    println!(
        "{}",
        match cli.command {
            None => version.to_string(),
            Some(SemverCommand::Increment { level }) => version.increment(level).to_string(),
            Some(SemverCommand::Get {
                component: VersionComponent::Major,
            }) => version.major.to_string(),
            Some(SemverCommand::Get {
                component: VersionComponent::Minor,
            }) => version.minor.to_string(),
            Some(SemverCommand::Get {
                component: VersionComponent::Patch,
            }) => version.patch.to_string(),
            Some(SemverCommand::Get {
                component: VersionComponent::Prerelease,
            }) => version.pre.to_string(),
            Some(SemverCommand::Get {
                component: VersionComponent::BuildMetadata,
            }) => version.build.to_string(),
        }
    );

    Ok(())
}

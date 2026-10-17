//! Convert one value from standard input to standard output.

use std::io::{self, Write};
use std::process::ExitCode;

use anyhow::Context;
use clap::Parser;
use conv::Format;

/// Usage, then the description, then the arguments. Same order as `cat` and `tr`.
const HELP_TEMPLATE: &str = "\
{usage-heading} {usage}

{about}

{all-args}{after-help}";

const AFTER_HELP: &str = "\
FROM and TO name the input and output encodings.  Names are
case-insensitive.  yaml accepts the alias yml.  url accepts the
alias www-form.

json output is compact.  Text encodings end with a newline.  cbor and
msgpack are raw bytes.  Input is one value.  A second value or other
trailing data is an error, and nothing is written.  Comments are not
preserved.  toml output must be a table, and null is an error.  url is
a query string, and its values are strings.

Examples:
  conv json toml    convert JSON on standard input to TOML
  conv yaml json    convert YAML on standard input to JSON

Exit status:
  0  if the conversion succeeded
  1  if the value could not be converted
  2  if there was a usage error";

#[derive(Parser)]
#[command(
    version,
    about = "Convert one value from standard input to standard output.",
    override_usage = "conv [OPTION]... FROM TO",
    help_template = HELP_TEMPLATE,
    after_help = AFTER_HELP,
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    /// input encoding
    #[arg(value_name = "FROM", value_enum, ignore_case = true)]
    from: Format,

    /// output encoding
    #[arg(value_name = "TO", value_enum, ignore_case = true)]
    to: Format,

    /// display this help and exit
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// output version information and exit
    #[arg(short = 'V', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };
    match run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("conv: {err:#}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: &Cli) -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    conv::convert(cli.from, cli.to, stdin.lock(), &mut output)?;
    output.flush().context("writing output")?;
    Ok(())
}

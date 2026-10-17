use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some(from), Some(to)) = (args.next(), args.next()) else {
        eprintln!("usage: conv FROM TO");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("usage: conv FROM TO");
        return ExitCode::from(2);
    }
    eprintln!("conv: {from} -> {to} is not implemented");
    ExitCode::from(1)
}

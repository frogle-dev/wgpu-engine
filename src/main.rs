use std::process::ExitCode;

use wgpu_engine::run;

fn report(err: &dyn std::error::Error) {
    eprintln!("error: {err}");
    let mut source = err.source();
    while let Some(cause) = source {
        eprintln!("| caused by: {cause}");
        source = cause.source();
    }
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        report(&err);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

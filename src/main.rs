use std::process::ExitCode;

fn main() -> ExitCode {
    match ping_pong::run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("some error occurred\n{e}");
            ExitCode::FAILURE
        }
    }
}

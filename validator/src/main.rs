extern crate clap;
extern crate cpython;
#[macro_use]
extern crate log;

mod pylogger;
mod server;

use server::cli;
use cpython::Python;

fn main() {
    let gil = Python::acquire_gil();
    let py = &mut gil.python();

    let args = cli::parse_args();

    let verbosity: u64 = args.occurrences_of("verbose");

    pylogger::set_up_logger(verbosity, py);

    let pydict = cli::wrap_in_pydict(py, &args)
        .map_err(|err| err.print(*py))
        .unwrap();

    let cli = py.import("sawtooth_validator.server.cli")
        .map_err(|err| err.print(*py))
        .unwrap();
    cli.call(*py, "main", (pydict,), None)
        .map_err(|err| err.print(*py))
        .unwrap();
}

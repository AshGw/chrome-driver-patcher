use std::fs::read as read_file;

#[cfg(target_os = "linux")]
mod buffer;
#[cfg(target_os = "linux")]
mod patch;
#[cfg(target_os = "linux")]
use structopt::StructOpt;

#[cfg(target_os = "linux")]
#[derive(StructOpt, Debug)]
#[structopt(name = "fucc12", about = "Patches Chromedriver binaries")]
struct Opt {
    #[structopt(parse(from_os_str))]
    executable: std::path::PathBuf,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let mut binary: Vec<u8> = read_file(&opt.executable).expect(&format!("Failed to read {:?}", opt.executable));
    eprintln!("Patching: {:?}", opt.executable);
    patch::randomize(&mut binary, b"$cdc_asdjflasutopfhvcZLmcfl_");
    patch::randomize(&mut binary, b"addScriptToEvaluateOnNewDocument");
    buffer::execute(&binary, std::env::args().skip(1))
}

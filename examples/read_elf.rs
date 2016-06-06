extern crate elf;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} /some/elf/file", args[0]);
        return;
    }

    let mut contents = Vec::new();
    File::open(&args[1]).expect("readable file").read_to_end(&mut contents).expect("read succeeds");

    let obj = elf::Object::new(&contents).expect("valid elf file");

    println!("{} {} {} {} {}:{}", obj.object_type(), obj.machine(), obj.bits(), obj.endian(), obj.abi(), obj.abi_version());
    println!("version: {}", obj.version());
}

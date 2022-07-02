use avdrive::Avd;
use std::env::args;
use std::fs::{read, write};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("usage: avdrw drive {{r|w|n}} [blknum file]");
        return;
    }

    match args[2].as_str() {
        "r" => {
            if args.len() != 5 {
                println!("usage: avdrw drive w blknum file");
                return;
            }
            let blknum = u16::from_str_radix(&args[3], 16).expect("could not parse blknum!");
            let drive = Avd::from_host_drive(&args[1]).expect("could not load drive!");
            let block = drive.get_block(blknum).unwrap_or([0; 256]);
            write(&args[4], block).unwrap()
        }
        "w" => {
            if args.len() != 5 {
                println!("usage: avdrw drive w blknum file");
                return;
            }
            let blknum = u16::from_str_radix(&args[3], 16).expect("could not parse blknum!");
            let mut drive = Avd::from_host_drive(&args[1]).expect("could not load drive!");
            let mut file = read(&args[4]).expect("could not load host file!");
            if file.len() > 256 {
                println!("host file too long!")
            }
            file.resize(256, 0);
            let f_arr = file.try_into().unwrap();
            drive.set_block(blknum, &f_arr);
            drive.save(&args[1]).unwrap()
        }
        "n" => {
            if args.len() != 3 {
                println!("usage: avdrw drive n");
                return;
            }
            let drive = Avd::new();
            drive.save(&args[1]).unwrap()
        }
        _ => println!("usage: avdrw drive {{r|w|n}} [blknum file]"),
    }
}

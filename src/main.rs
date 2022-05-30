use avdrive::Avd;
use std::env::args;
use std::fs::{read, write};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 5 {
        println!("usage: avdrw drive blknum {{r|w}} file");
        return;
    }

    // avdrw drive blknum {r|w} file
    let mut drive = Avd::from_host_drive(&args[1]).expect("could not load drive!");
    let blknum = u16::from_str_radix(&args[2], 16).expect("could not parse blknum!");
    match args[3].as_str() {
        "r" => {
            let block = drive.get_block(blknum).unwrap_or([0; 256]);
            write(&args[4], block).unwrap()
        }
        "w" => {
            let mut file = read(&args[4]).expect("could not load host file!");
            if file.len() > 256 {
                println!("host file too long!")
            }
            file.resize(256, 0);
            let f_arr = file.try_into().unwrap();
            drive.set_block(blknum, &f_arr)
        }
        _ => println!("must be r or w!"),
    }
}

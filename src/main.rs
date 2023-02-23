use clap::Parser;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Parser)]
struct Arguments {
    path: std::path::PathBuf,
    #[arg(default_value = "x")]
    hash: String,
}

fn main() {
    let args = Arguments::parse();
    let f = File::open(args.path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = [0u8; std::mem::size_of::<u128>()];
    let mut hash: u128 = 0;
    let mut word: u128 = 0;
    let mut n: u32 = 0;
    let mut eof: bool = false;

    loop {
        use std::io::ErrorKind;
        //reading next word
        if !eof {
            let res = reader.read_exact(&mut buffer);
            match res {
                Err(error) if error.kind() == ErrorKind::UnexpectedEof => eof = true,
                _ => {}
            }
            word = u128::from_le_bytes(buffer);
        }

        //hashing
        n = u32::wrapping_add(n, 1);
        hash = u128::wrapping_add(hash, word);
        word = u128::rotate_right(word, n);
        hash ^= word;

        //do some stuff to the first word (and once n wraps around) to make sure even really small files are properly hashed
        if n == 1 {
            word = u128::wrapping_mul(word, 0xE426C1558D241855065F12A268B0DD89);
            word = u128::rotate_right(word, u32::try_from(word % u128::from(u32::MAX)).unwrap());
        } else if eof {
            break;
        }
    }

    //printing result
    println!("{:X}", hash);
    if args.hash != "x" {
        let arg_hash: u128 = u128::from_str_radix(&args.hash, 16).unwrap();
        eprint!("hashes match: ");
        println!("{}", arg_hash == hash);
    }
}

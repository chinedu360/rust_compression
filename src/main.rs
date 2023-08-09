use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use::std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main(){
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    let _ = copy(&mut input, &mut encoder);
    let output = encoder.finish().unwrap();
    println!("Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Elasped: {:?}", start.elapsed());
}

// how to use!
//RUN
//cargo run `name_of_file_to_be_compressed` `file_name_for_the_compressed_file` 
use bardecoder;

use image;

fn main() {
    println!("Hello, world!");
    let img = image::open("/Users/prayog/Development/barcode-parser/src/test.png").unwrap();

    let decoder = bardecoder::default_decoder();

    let results = decoder.decode(&img);
    for result in results {
        println!("{}", result.unwrap());
    }

}

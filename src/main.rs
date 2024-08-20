use image::io::Reader as ImageReader; // it is sort of conflicting with standard io read "std::io::Read"

fn resizee()
fn main()  -> Result <(), ()>{

    let imge: Reader<BufReader<File>> = ImageReader::open(path: "")?.decode()?;
    println!("Hello, world!");
}

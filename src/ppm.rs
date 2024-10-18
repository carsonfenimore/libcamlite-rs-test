use std::fs::File;
use std::io::Write;

#[allow(non_snake_case, deprecated)]
pub fn writePPM(bytes: &[u8], width: u32, height: u32, filename: &str){
  let mut f = File::options()
      .write(true)
      .create(true)
      .open(filename.to_string())
      .unwrap();
  writeln!(&mut f, "P6\n{} {}\n255\n", width, height).unwrap();
  f.write_all( bytes ).expect("Failed to write/open h264 file");
}

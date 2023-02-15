use jenkins_hash::lookup2;

fn main() {
  let data = "Hello, this is a test string. It is just used to be hashed.".as_bytes();
  let initial = 0x3875682; // start value - like a seed

  println!("{}", lookup2(&data[..], initial));
}
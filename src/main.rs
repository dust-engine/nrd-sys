fn main() {
    let instance = nrd::Instance::new(&[]).unwrap();
    let desc = instance.desc();
    println!("{:#?}", desc);
}

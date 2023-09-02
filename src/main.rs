fn main() {
    let desc = unsafe { nrd::ffi::GetLibraryDesc() };
    println!("{:#?}", desc);
}

use phot::library::Library;

fn main() {
    let lib = Library::new();
    lib.create().expect("oops, that didn't work");
    println!("Created a library at {}", lib.get_path().to_str().expect("no library path"));
}

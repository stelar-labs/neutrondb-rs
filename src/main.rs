mod lib;

fn main() {

    let store = lib::store("blocks");
    println!(" * store: {:?}", store.name);

}
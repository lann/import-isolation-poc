wit_bindgen::generate!({
    world: "parent",
    path: "../wit",
});

fn main() {
    println!("parent here");
    component::printer::printer::print("child here");
}

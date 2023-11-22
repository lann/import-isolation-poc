wit_bindgen::generate!({
    world: "child",
    path: "../wit",
    exports: {
        "component:printer/printer": Guest
    }
});

struct Guest;

impl exports::component::printer::printer::Guest for Guest {
    fn print(s: String) {
        println!("{s}");
    }
}

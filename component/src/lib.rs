wit_bindgen::generate!("myapp");

struct Host;
export_myapp!(Host);

impl Myapp for Host {
    fn do_the_thing(Foo { bar, baz }: Foo) {
        let string = format!("{bar} then {baz}");
        some_host_func(&string);
    }
}

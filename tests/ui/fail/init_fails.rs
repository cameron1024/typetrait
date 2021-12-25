fn main() {
    let enabled = Enabled;
}

fn foo() -> Enabled {
    unimplemented!()
}

typetrait::union! {
    pub Status = Enabled | Disabled
}



typetrait::union! {
    #[derive(Debug, Clone)]
    pub Status = Enabled | Disabled
}

struct Foo<T: Status> {
    _marker: std::marker::PhantomData<T>,
}

fn main() {}


fn main() {
    assert_safe(SafeStruct::default());
}

typetrait::blanket!{ Safe = Send + Sync + 'static }

fn assert_safe<T: Safe>(_t: T) {}

#[derive(Default)]
struct SafeStruct {
    inner: String,
}


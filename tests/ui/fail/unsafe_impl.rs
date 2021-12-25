fn main() {}

typetrait::union! {
    Status = Enabled | Disabled
}

struct Illegal;
impl Status for Illegal {}

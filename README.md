# typetrait

Helper macro for generating boilerplate for more type safe APIs.

For example, consider an API that receives user input:
```rust
struct Data {
  email: String,
  age: u8,
}

fn get_data() -> Data {
  todo!()
}
```

We might want to make sure that we have validated that `data.email` contains a valid email address before we use it.
This is possible by making `Data` generic over "whether or not it has been validated":
```rust
// generates a trait called Status, and types Validated and Unvalidated that implement Status
union! {
  pub Status = Validated | Unvalidated
}

struct Data<T: Status> {
  _marker: std::marker::PhantomData<T>,
  email: String,
  age: u8,
}
```
With this setup, we can now prevent unvalidated data from being used at compile time:
```rust
// data received from user input is considered unvalidated
fn get_data() -> Data<Unvalidated> {
  todo!()
}

// only validated data should be used by the rest of the application
fn handle_data(data: Data<Validated>) {
  todo!()
}

// convert unvalidated data to validated data (by validating it!)
fn validate(Data { email, age, .. }: Data<Unvalidated>) -> Result<Data<Validated>, &'static str> {
  if !is_valid_email(&email) {
    return Err("invalid email");
  }
  
  Ok(Data {
    email,
    age,
    _marker: std::marker::PhantomData,
  })
}
```

This API is now significantly harder to misuse. Instead of requiring the programmer to keep track of which data is validated and which isn't, this work is offloaded to the compiler.

This pattern is widely applicable, for example:
 - Various data might be `Validated` or `Unvalidated`
   - this could be especially useful in security contexts, for example a `SessionToken<Validated>` could be required to make a particular database request
 - GPIO pins could be `Enabled` or `Disabled`, as well as `Input` or `Output` (see https://docs.rust-embedded.org/book/design-patterns/hal/gpio.html for a more thorough example)
 - When making a safe Rust interface to some FFI code, you may have to encode the state of that data (for example `Initialized`/`Uninitialized`/`Disposed`)
 
 ## Details
 
 Each trait generated by `union!` is "sealed", meaning it cannot be implemented outside of the module they are defined in. They generate a private module with a supertrait which is implemented for only the types given in the macro.
 
 Each type is an empty enum (i.e. an enum with no variants) and so cannot be instantiated. Note that these types are almost exclusively used with `PhantomData`, since they only ever exist as concrete values for type parameters, but never as values themselves (there are no possible values of an empty enum).
 
 
# extend-ref

A wrapper struct that implements `Extend` for mutable references.


```rust
use extend_ref::ExtendRef;

fn unzip_on_refs(
    mut squares: &mut impl Extend<i32>,
    mut cubes: &mut impl Extend<i32>,
    mut tesseracts: &mut impl Extend<i32>
) {
    // Create an iterator of a 3-tuple
    let iter = (0i32..10).map(|i| (i * i, i.pow(3), i.pow(4)));

    // Unzip the iterator into the three referenced collections
    (ExtendRef(squares), ExtendRef(cubes), ExtendRef(tesseracts)).extend(iter);
}
```



## License

Licensed under Apache License, Version 2.0 ([LICENSE](LICENSE) or https://www.apache.org/licenses/LICENSE-2.0).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

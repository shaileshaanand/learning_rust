# Rules of references

### 1. We can have any number of immutable references.

```rust
    fn main() {
        let s=String::from("Hello");
        let r1=&s;
        let r2=&s;
        let r3=&s;
    }
```

### 2. We can create only one mutable reference in a scope.

```rust
    fn main() {
        let mut s = String::from("Hello");
        let r1 = &mut s;
        let r2 = &mut s; // ERROR
        let r3 = &mut s; // ERROR
    }
```

We can have more refs in a separate scope:

```rust
    fn main() {
        let mut s = String::from("Hello");
        {
            let r1 = &mut s;
        } // r1 is deleted here so only one mut ref exists.
        let r2 = &mut s;
    }
```

_This is because 2 mutable refs writing at the same time in separate threads may cause inrace condition._

### 3. We cannot create mutable reference if the program uses any immutable reference.

```rust
    fn main() {
        let mut s = String::from("Hello");
        let r1 = &mut s;
        let r2 = &s; // ERROR
    }
```

But this valid because of scope:

```rust
    fn main() {
        let mut s = String::from("Hello");
        {
            let r1 = &mut s;
        } // r1 is deleted here so only one mut ref exists.
        let r2 = &s;
    }
```

_This is because immutable ref may try to read while in a separate thread mutable ref may to write to the same causing a race condition._

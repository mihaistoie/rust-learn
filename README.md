# Rust by exemple
I'm trying to learn Rust !!!

### Mutability
```rust
let _immutable_binding = 1;  // in Javascript const _immutable_binding = 1;
let mut mutable_binding = 1; // in Javascript let _immutable_binding = 1;
```

### Scope and Shadowing
```rust
let shadowed_binding = 1;
{
  let shadowed_binding = "abc";
}
let shadowed_binding = true;
 ```
  

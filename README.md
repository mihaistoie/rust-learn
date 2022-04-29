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

### Arrays and Slices
An array is a collection of objects of the same type
```rust
xs: [i32; 5] = [1, 2, 3, 4, 5]; // array, length is known at compile time  
```

Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice.
```rust
 // Arrays can be automatically borrowed as slices
 analyze_slice(&xs);
```

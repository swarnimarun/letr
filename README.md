# `letr`

The lazy way out.

```rust
fn get<T: Default>(value: Option<T>) -> T {
    letr![ Some(x) = value, T::default() ];
    x
}
```

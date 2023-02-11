# `letr`

The lazy way out.

```rust
fn simple() {
    enum Items {
        RevItem(Option<i32>),
        ForItem(u32),
    }

    fn foo(x: Items) -> i32 {
        letr! {
            Items::RevItem(ret) = x, else -1;
            Some(ret) = ret, else -1;
        };
        ret
    }
    assert_eq!(foo(Items::RevItem(Some(0))), 0);
    assert_eq!(foo(Items::ForItem(0)), -1);
}
```

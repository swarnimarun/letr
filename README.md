# `letr`

The lazy way out.

```rust
fn simple() {
    enum Items {
        RevItem(Option<i32>),
        ForItem(Option<Option<Option<Option<i32>>>>),
    }

    fn foo(x: Items) -> i32 {
        letr! {
            Items::RevItem(ret) = x, else -1;
            Some(ret) = ret, else -2;
        };
        ret
    }
    assert_eq!(foo(Items::RevItem(Some(0))), 0);
    assert_eq!(foo(Items::ForItem(Some(Some(Some(Some(0)))))), -1);
    assert_eq!(foo(Items::RevItem(None)), -2);

    fn bar(x: Items) -> Option<i32> {
        letr! {
            Items::ForItem(ret) = x, else Some(-1);
            Some(ret) = ret, else Some(-2);
            Some(ret) = ret, else Some(-3);
            Some(ret) = ret, else Some(-4);
            ret = ret?; // returns none by default
        };
        Some(ret)
    }
    assert_eq!(bar(Items::ForItem(Some(Some(Some(Some(0)))))), Some(0));
    assert_eq!(bar(Items::RevItem(Some(0))), Some(-1));
    assert_eq!(bar(Items::ForItem(None)), Some(-2));
    assert_eq!(bar(Items::ForItem(Some(None))), Some(-3));
    assert_eq!(bar(Items::ForItem(Some(Some(None)))), Some(-4));
    assert_eq!(bar(Items::ForItem(Some(Some(Some(None))))), None);
}
```

A simpler alternative to [if_chain](https://crates.io/crates/if_chain) crate, but using let-else semantics.
Generally works better with rust-analyzer.

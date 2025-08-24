# wikisort-rs

> I have no allocator, and I must do stable sort, in O(n log n).\
> \- [wikisort](https://github.com/BonzaiThePenguin/WikiSort), maybe.

Fuck-your-allocator sorting al gore rhythm from a bloody-fucking-hellfire

don't even try to think to ask me how i implemented this, i'm in therapy.

Licence? I was about to release it in public domain even if the og was GPLv69 but the og is Unlicense yay

## What it does

- Allocatorless sort
- in O(n log n) time complexity
- stable, safe, unlike my nightly, unsafe mental health
- make me fucking broke

## Contribution

don't, this cursed al gore rhythm will destroy you(except comments or assertion failure or CVE)

## wtf is grailsort

I wanted to implement Grailsort at first but [check this out](https://www.youtube.com/watch?v=k4ZkHJvGKhE)

## How to use

```sh
cargo add wikisort
```

```rust
use wikisort::*;
use core::cmp::Ordering;

// if you want to use it like a slice.sort();
trait Wikisort<T> {
    fn wikisort(&mut self) where T: Ord;
    fn wikisort_by<F>(&mut self, cmp: F) where F: Fn(&T, &T) -> Ordering;
    fn wikisort_by_key<F, K>(&mut self, f: F) where F: Fn(&T) -> K, K: Ord;
}

impl<T> Wikisort<T> for Vec<T> {
    fn wikisort(&mut self) where T: Ord {
        wikisort(self, |a, b| a.cmp(b));
    }

    fn wikisort_by<F>(&mut self, cmp: F) where F: Fn(&T, &T) -> Ordering {
        wikisort(self, cmp);
    }

    fn wikisort_by_key<F, K>(&mut self, f: F) where F: Fn(&T) -> K, K: Ord {
        wikisort(self, |a, b| f(a).cmp(&f(b)));
    }
}

fn main() {
    // ... fuck around with the data vector ...
    let mut data = vec![5, 2, 8, 1, 9, 3, 7, 4, 6, 0];
    data.wikisort();
    assert!(data.is_sorted()); // ill kill myself if it breaks
}
```

fuck you

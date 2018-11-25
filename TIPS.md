# Examples of rust tips I have compilated day in-out

### Removing duplicate element from vector

```rust
fn remove_duplicate_elements<T: Ord>(elements: &mut Vec<T>) {
    elements.sort; // use elements.sort_unstable(); is the ordering does not matters
    elements.dedup();
}

fn main() {
    let mut sample_elements = vec![0, 0, 1, 1, 2, 3, 2];
    println!("Orginal : {:?}", sample_elements);
    remove_duplicate_elements(&mut sample_elements);
    println!("Deduced : {:?}", sample_elements);
}
```

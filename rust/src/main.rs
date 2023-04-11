#[derive(Default)]
struct Node {
    value: i32,
    children: Vec<Node>,
}
fn main() {
    let values: Vec<i32> = vec![];
    let mut root: Node = Default::default();
    let mut current_node = &mut root;

    'outer: for value in values {
        for (i, element) in current_node.children.iter_mut().enumerate() {
            if element.value == value {
                // current_node = &mut current_node.children[i]; // This one works
                current_node = element; // This one doesn't
                continue 'outer;
            }
        }
    }
}

// error[E0499]: cannot borrow `current_node.children` as mutable more than once at a time
//   --> src/main.rs:12:29
//    |
// 12 |         for (i, element) in current_node.children.iter_mut().enumerate() {
//    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//    |                             |
//    |                             `current_node.children` was mutably borrowed here in the previous iteration of the loop
//    |                             first borrow used here, in later iteration of loop

// For more information about this error, try `rustc --explain E0499`.
// warning: `rust` (bin "rust") generated 1 warning
// error: could not compile `rust` due to previous error; 1 warning emitted

Done on 12/8/24.

## Things Learned

 * The prefix `_xyz` indicates that the variable `xyz` is going intentionally unused in your code.
 * The `:` syntax after a variable name is used to specify type.
 * There is no direct function `.parse<i32>` for an iterator; you have to use the `.map` or `.filter_map` functions. Parse methods are specific to item type, and Rust prioritizes explicitness, with parse type functions being rather ambiguous. This avoids ambiguity in how errors should be handled.
 * The `.filter_map` function will handle errors and skip invalid values
 * If you're sure the input will be valid, you can just use `.map` directly (with `.unwrap`)? But if there is an error the function will panic.

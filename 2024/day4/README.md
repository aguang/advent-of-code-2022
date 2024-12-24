## Things Learned

 * `isize` is signed integers, `usize` is unsigned. [Source](https://doc.rust-lang.org/reference/types/numeric.html#:~:text=The%20usize%20type%20is%20an,as%20the%20platform's%20pointer%20type.)
 * Best practice is to return Results instead of bool (unless the true/false matters) [Source](https://www.reddit.com/r/rust/comments/dzn0p5/returning_bool_vs_result/)
 * To then use the Results enum in an if-else function, you would either use `if let Ok(value)` if you need the value, or `if fn.is_ok()` if you just need to check if the function returned Ok. You can also use `match`, which is what is recommended if you want detailed/explicit handling for both `Ok` and `Err`.
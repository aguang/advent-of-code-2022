Done on 12/21/24.

## Things Learned

 * The `regex` package (and you install things thru `cargo add`)
 * To find the index at which your regex expressions are, you use the `.find` function. This will return either `Some(T)` or `None` if match is found / not
  * `Some(T)` represents the presence of the value T
  * This is confusing to me, but `let Some(caps) = x` will assign the value `x` to `caps` but also seems to be an if/else type expression [from regex crate](https://docs.rs/regex/latest/regex/)
 * To index a string from `i` to the default ending index (end of string): `s[i..]` i.e. that's what the `..` means
 * A lot about mutability of values... it's hurting my head. [There are no mutable parameters in Rust](https://www.snoyman.com/blog/2020/05/no-mutable-parameters-in-rust/)
 * Got some serious practice in with using mutable references/dereferencing in order to return/change the input variable within a function. References: [Expected type `bool`, found type `&bool`](https://stackoverflow.com/questions/44788026/expected-type-bool-found-type-bool) and [how to use a function that returns and modifies its input](https://users.rust-lang.org/t/how-to-use-a-function-that-returns-and-modifies-its-input-value/26200/2).
 * You can specify return type in a function as such: `fn func(input) -> return_type {}`
 * Generally, we want to write functions taking references to strings (i.e. `&str`) instead of consuming them (i.e. `String`) because we don't want to copy or own the data.
   * Consuming (in Rust) means value's ownership is transferred from one part of program to another, and original owner can no longe ruse the value.
   * Ownership is a big concept in Rust with regards to safety guarantees, as this ensures the string isn't going to be altered or moved since we are just referencing it so we don't need to think about garbage collection. This is a little confusing for me because with C++ if you reference a string I believe you can change what's in it. Or maybe that was with pointers? Definitely with pointers at least.
   * Not owning the data means the caller can keep using it after calling the function because it still controls it.
   * I guess it also allows a function to accept both string slices and heap-allocated strings, but I am not sure what heap-allocated strings are and am too tired to look them up right now.
   * You would take ownership (i.e. call `String`) if you *do* want to change the string, or you're storing the string in something that is going to outlive the scope of the caller.
 * re: references&pointers, it was not just me being confused, they are different between Rust and C++, primarily with safety (according to ChatGPT):

Key Differences Between Rust and C++ References
| Feature | Rust References	| C++ References |
| --- | --- | --- |
| Nullability | Cannot be null; use Option<&T> instead |	Cannot be null directly|
|Safety	|Compiler-enforced borrow checker ensures safety	|Programmer must ensure safety|
|Mutability	|Immutable by default, must explicitly use &mut|	No mutability distinction|
|Lifetimes|	Lifetimes are explicitly or implicitly tracked	|No lifetime tracking|
|Aliasing Rules	|No simultaneous mutable + immutable references	|Allows multiple mutable references|
|Dangling References|	Not allowed; compile-time error	|Possible; leads to undefined behavior|
|Performance	|Same (both are zero-cost abstractions)|	Same (zero-cost abstractions)|

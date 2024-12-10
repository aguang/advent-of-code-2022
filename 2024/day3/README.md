## Things Learned

 * The `regex` package (and you install things thru `cargo add`)
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

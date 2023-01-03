# 2022

[Advent of Code 2022](https://adventofcode.com/2022)

Solutions are in Julia to brush up on and strengthen my Julia proficiency. Maybe I'll start using it for some real data science projects in the new year! Occasionally I've benchmarked multiple solutions to check my intuition on which one is faster. Those are shown in the table below.

| Day | Solution | Benchmark |
|---|---|---|
| 1 | If else statements to check top 3 and saving only those values | 132.917 μs |
| 1 | Array with all sums, sort, get top 3 | 149.583 μs |

## Things learned

I learned quite a bit about Julia doing these exercises.

 * What promotion is (converting arguments of mathematical operators to a common type), and how [Julia uses (polymorphic? I didn't learn what this was) multiple dispatch](https://docs.julialang.org/en/v1/manual/conversion-and-promotion/) rather than automatic promotion to do essentially the same thing
 * [The difference between parse and convert](https://docs.julialang.org/en/v1/manual/conversion-and-promotion/)
 * If I have a type `Substring{String}` but it's really just a character I should do `first(c)` as there is no method to convert `Substring{String}` to `Char`, presumably because `Substring{String}` could be multiple characters. So it's kind of more like an array.
 * Initializing an array with `undef` requires another initialization step later
 * How to write tests!

 Also learned quite a bit about VSCode shortcuts:

 * Press "Tab" to indent an entire section of highlighted code, and "Shift+Tab" to deindent
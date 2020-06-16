# Book Club Notes
[The Rust Programming Book](https://doc.rust-lang.org/book/title-page.html)

## Chap 9 Errors + Panic

- panic!
  - I thought you didn't want panic, but you can specify what things call into panic (expect and unwrap?)
    - 9.3 explains :)
- Result feels like Option T (kinda)
  - error -> result
  - none -> option
- ? is cool
  - from function is really cool (convert errors from one type to another)

## Chap 10 Generic Types, Traits, and Lifetimes

- good callbacks to past chapters
- largest example
  - aren't you mutating the given list? Is that considered bad style in rust?
10.1 Generics
- cool that you can implement functions for specific generic types
- I think monomorphization is how other languages do it too?
10.2 Traits
- I like that it calls out that it's like an interface
- coherence, and more specifically the orphan rule: can't implement a trait for non local code
  - others can't break your code
- Using a trait as  parameter is lit! with plus syntax!
- T has the trait bound Clone instead of Copy
  - how does clone work?
    - more extensive copy? clone can be implemented, but not copy
    - because copy is already nicely optimized
- Using the clone function means we’re potentially making more heap allocations

10.3 lifetimes

- notes are useful
- prevent dangling (reminds me of how registers work in mips )
- annotation of the lifetimes ('a) is kinda confusing
  - so you're telling the borrow checker that it'll last as long as the things you give it. why does that make it ok?
  - what if you dangle something still: answered it's the smallest one and won't compile
- 'static is the long lifetime

## Chap 11

- tests! who writes them :p
- in 11.2 it "showing function output" said it would panic, but doesn't? I don't think

## Chap 12 I/O

12.1 CLI

- collect things and notice module scoping
- pretty normal argument parsing

12.2 Reading a file

- read in file with lib

12.3 Refactor

- single responsibility, our error handling isn't great
  - move to lib.rs
  - main.rs needs to error handle
- clone: is expensive at runtime but can save you on ownership
- try not to panic! Use Result
- call back: `'static` is a special annotation for a lifetime of the duration of the program
  - Love that the book specifically calls this out
- Introduces `dyn` keyword for dynamic
  - you can box and dynamic error for extra flexiblity
- `if let` reminds me of ruby
  - though this type of assignment in an `if` is frowned upon there
- it mentions visibility `pub` are there other visibility?
  - default of package-public like java ?
- ? vscode not able to find local crate and complains?

12.4 TDD

- woot woot tdd!
- good re-explanation of lifetimes, good to be reminded when pertinant
- general make things testable, straightforwards

12.5 Env Vars

- why `&query`, because `to_lowercase` create a new String so query must be a String not a string slice

12.6 Stderr instead of stdout

- add an `e` to the prints

## Chap 13 Functional Language: Iterators + Closures

hol up, my entire college education has amalgamated to this point :0

13.1 Anonymous Functions

- closures have envs
- you don't need to annotate type in closure because complier can infer
- closures: take ownership, borrow mutably, and borrow immutable
  - FnOnce: closure takes ownership of env (called only once)
  - FnMut: borrows mutably (change env)
  - Fn: borrow immutably
- `move` forces ownership

13.2 Iterators

- iterators are lazy!
  - great error message that it tells you that! :D
- they get immutable references
- std lib contains the normal kind of functions

13.3 I/0 Project

- functional programming prefers minimize amount of mutable state!
- rust's closures and iterators are inspired by functional programming

## chap 14 cargo + crates.io

- 

Book Club Notes

Chap 9 Errors + Panic
- panic! 
    - I thought you didn't want panic, but you can specify what things call into panic (expect and unwrap?)
        - 9.3 explains :)
- Result feels like Option T (kinda)
    - error -> result
    - none -> option
- ? is cool
    - from function is really cool (convert errors from one type to another)

Chap 10 Generic Types, Traits, and Lifetimes
- good callbacks to past chapters
- largest example
    - aren't you mutating the given list? Is that considered bad style in rust?
10.1 Generics
- cool that you can implement functions for specific generic types
- I think monomorphization is how other languages do it too?
10.2 Traits
- I like that it calls out that it's like an interface
-  coherence, and more specifically the orphan rule: can't implement a trait for non local code
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
Chap 11
- tests! who writes them :p 
- in 11.2 it "showing function output" said it would panic, but doesn't? I don't think

Chap 12


chap 13

chap 14 

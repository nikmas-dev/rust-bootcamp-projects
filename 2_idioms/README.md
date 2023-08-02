Step 2: Idioms
==============

These steps describe common idioms required for writing well-designed and idiomatic [Rust] code.

> ❗️Before completing this step you should complete all its sub-steps.

After doing them you should be able to answer the following questions:
- Why should I care about types and expressing things in types? How do types help to increase guarantees of a program being correct?
    > By expressing more things in types we can move many correctness checks from runtime to compile time. This allows to catch 
      many errors earlier and thus increase the reliability of a running program. The more powerful type system is, the stronger 
      the guarantee becomes that if a program compiles, it is correct. 
- What is essential for writing well-designed and ergonomic APIs in [Rust] and why?
    > In Rust one of the key tips to well-designed and ergonomic APIs is to use types to express as much as possible,
      so that the compiler enforces the proper using of these APIs for achieving the desired result. For instance,
      for managing states there is a powerful thing called typestate pattern. With its help it's possible to move 
      properties of state (the dynamic information a program is processing) into the type level (the static world that the compiler 
      can check ahead-of-time). Ultimately, at compile time in some states we can access one set of methods, and in other states
      we can access other. If we try to access a method that is not available in the current state, the program will not even compile.
- Why `mem::replace` exists and what purpose does it sole? When and why is it really helpful?
    > Sometimes the strictness of the borrow checker may not allow you to do some operations like swapping and in-place mutations,
      and force you to use redundant cloning. In these cases mem::replace can help circumvent borrow checker limitations and avoid 
      unnecessary expensive clones. For instance, we have a &mut MyEnum which has two variants: A { name: String, x: u8 } and 
      B { name: String }. Say we want to change MyEnum::A to a B if x is zero, while keeping MyEnum::B intact. To achieve this without 
      cloning "name" we can use mem::replace (or mem::take) to move "name" from A to B while having mutable reference to the enum.
- How input type polymorphism is usually organized in [Rust] APIs? What cost does it have?
    > In Rust to make out API ergonomic we try to avoid forcing user to make explicit type conversions and hide them behind generic
      input parameters with certain bounds. For instance, if in our function we need to accept an owned String, we would rather
      define it as impl Into<String> than just String. Because with just String if the user initially has &str, they would have 
      to write the transform call from &str to String explicitly. But with impl Into<String> the use can just pass &str, and 
      we will perform the required conversions behind the scenes. 
   
    > The con of this idiom is that compiler generates more code due to monomorphization that potentially leads to code bloating. 
      But this can be optimized by the separation of generic conversions from other parts of a function.
- Which ways and tools do exist for future-proofing source code in [Rust]?
    > We can leverage the exhaustiveness checking of the pattern-matching in Rust to spot certain bugs at compile-time by checking 
      whether all the combinations of some values where covered and considered. With enums it's better to mention all the arms 
      without using the wildcards to make sure that if a new variant is added, the compiler will force us to consider it. When working
      with structs, we can use destructuring to make sure that all the fields are considered. If we need to preserve 
      backwards compatibility when adding new variants or fields to our data types, we may consider using the #[non_exhaustive] 
      attribute, which limits ways to construct and match types. Also, if we want to make adding new methods to our traits
      non-breaking change, we can harness the sealing technique.




## Task

__Estimated time__: 2 days




Design and implement a `VendingMachine` type, which behaves like a [vending machine][1]:
- `Product` should has a price and a name;
- `VendingMachine` should have a limited capacity of `Product`s;
- `VendingMachine` should be able to give a rest;
- `VendingMachine` should reject purchase if it cannot give a rest;
- `Coin`s could have only `1`, `2`, `5`, `10`, `20` and `50` nominals.

Make its usage API as convenient as you're capable to.




[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Vending_machine
[2]: https://doc.rust-lang.org/book/ch11-03-test-organization.html
[3]: https://youtu.be/Vw8BFScm0K0

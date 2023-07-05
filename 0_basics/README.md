Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 4 days

Read through [Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing [Rust By Example] and [rustlings].

Read through [Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, you should be able to answer (and understand why) the following questions:
- What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
    > Rust uses a system of ownership, which is a set of rules the compiler checks at compile time
      to ensure memory safety of a program. There are 3 main rules: each value has its owner;
      there can only be one owner at a time; when the owner goes out of scope, the value will be dropped.

    > Rust uses the stack and the heap, which are parts of memory available to use at runtime.
      Data with known at compile time and fixed size is stored on the stack and data with unknown
      at compile time or variable size is stored on the heap.

    > Rust is both single-threaded and multiple-threaded. One can write single-threaded code,
      but it's also possible to use standard library primitives to spawn system threads.
      Rust prevents data races at compile time and minimizes the risk of deadlocks and race conditions.
  
    > Rust is both synchronous and asynchronous. It's possible to write synchronous code, but at the same time
      Rust also has support for asynchronous programming with its async/await syntax. Rust had a built-in async
      runtime before 1.0, but then it was pulled out. Rust sticks with a philosophy to provide only the crucial 
      functionality with its standard library and to move everything else to external libraries. That's why the
      async runtime was removed from the std, and now it's available as a number of libraries on crates.io
      such as "tokio" and "async-std".
- What runtime [Rust] has? Does it use a GC (garbage collector)?
    > Rust doesn't have a runtime in the traditional sense. It doesn't require a separate runtime environment
      or virtual machine to execute programs. Rust programs are compiled to machine code and can be run directly
      by the operating system. But at the same time there are pieces of functionality that can be called a runtime,
      such as the allocator or the async runtime provided by the external crates.

    > Rust doesn't use a GC. Instead, memory in Rust is managed through a system of ownership, borrowing, and lifetimes,
      which are checked at compile time.
- What statically typing means? What is a benefit of using it? Weak typing vs strong typing? Implicit / explicit?
    > Statically typing means that the type of variable is known at compile time. One of the main benefits of it is catching
      type errors early at compile time to prevent a decent amount of bugs before a program even runs. Also, because the type
      of each value is known ahead of time, the compiler can optimize the code more effectively. It's also easier to understand 
      a program when you know all of its types. And for the IDE writers it's simpler to implement language servers, because
      the types are statically known. In the context of Rust it's the statically typed language.

    > With weak typing variables can be implicitly coerced to unrelated types, while with strong typing 
      they cannot. The benefit of strong typing over weak typing is that it can prevent unexpected bugs due to implicit conversion 
      between unrelated types making code more predictable. But at the same time the con of strong typing is that it requires
      more explicit type conversions and might feel less flexible. In the context of Rust it's strongly typed language.

    > With explicit typing a programmer should specify all types of variables explicitly, but with implicit typing types
      can be inferred by the compiler. Rust has both implicit and explicit typing. The compiler will try to infer as much
      as possible based on the program's context, but sometimes it can't do it, and thus it's required to specify types explicitly. 
- What are generics and parametric polymorphism? Which problems do they solve?
    > Parametric polymorphism is a more formal term for what most people think of as "generics". It's a feature that allows 
      code to operate on multiple types. Instead of writing code for each specific type, you can write code that works on a general
      type, and then provide the specific types when you use it. The main problem generics solve is code duplication.
      Without generics we would have to write the same logic multiple times for different types, but with generics we can write
      the logic once and then apply it to multiple types.
- What is nominative typing and structural typing? What is difference?
    > In nominative typing 2 instances are of the same type if they're declared of the same type name.
      An example of nominative typing is Java. In structural typing 2 objects are considered to be of the same type
      if they have the same properties and methods. An example of a language that uses structural typing is TypeScript.
      If to talk about Rust, it leverages a mix of nominative and structural typing. For instance, Rust uses nominative typing 
      for its data types, but traits use a form of structural typing.
- What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? Uncovered type? What is a marker trait?
    > Traits in Rust are a way to define shared behaviour across types. For example, we can define "Swim" trait with method "swim"
      and implement this trait for all types we're considered to be able to swim. Then we can write functions that operate
      on "Swim" and pass in any type that implements "Swim".

    > One of the main differences between traits and interfaces is that traits can be implemented on types you didn't define.
      For instance, it's possible to define custom traits on built-in primitives like i32. As a result, all i32 variables
      in the context where the trait is imported will get methods of this trait. 

    > The auto trait is the trait that is automatically implemented for a type if all of the type's constituent parts implement
      this trait. For example, Sync trait is the auto trait. If every type inside a struct implements Sync, then the struct 
      itself is also Sync.

    > The blanket impl is the implementation of a trait for a generic type with certain bounds. For instance, the standard library 
      includes a blanket implementation of the AsRef trait for any type that implements Borrow. It means that if you have 
      a type that implements Borrow, you get the AsRef methods for free.

    > The uncovered type is a type that doesn't appear as an argument to another type. For instance, T on its own is uncovered,
      but the T in Vec<T> is covered.

    > The marker trait is a trait that doesn't have any methods. It's used to mark types as having certain properties. 
      For example, Copy trait is the marker trait that indicates types that are "plain old data". Such types can be duplicated
      just by copying bits on the stack.
- What are static and dynamic dispatches? Which should I use, and when? What is monomorphisation?
    > Static dispatch is a type of function call where the exact function to be called is known at compile time. 
      It's possible when the function's concrete type is known ahead of time and this function is directly called.
      In Rust the static dispatch is implemented through monomorphisation. Monomorhisation is the process during which 
      the compiler creates copies of the generic function, but with the concrete type parameters this function was called
      with in the context of the program. This can result in faster code, because the exact function call can be inlined 
      by the compiler. But it may also increase the binary size due to duplications of the same function.
  
    > Dynamic dispatch determines which function to call at runtime. In Rust, it's achieved by using the trait objects
      (Box<dyn Trait> or &dyn Trait). The trait object is defined as the fat pointer. This fat pointer contains of 2 pointers:
      one points to the data of the object and second points to the vtable that comprises all implementations of the methods 
      of the Trait for the particular type. With such approach the size of the trait object is known at compile time: it's
      exactly 2 * usize. Also, the compiler treats the trait objects for the particular trait as having 
      the same type. For example, because of all of that it's possible to put trait objects built from different types into 
      the vector. Dynamic dispatch can result in slower code due to the indirection (vtable) and inability to inline 
      the function call, but it allows for more flexibility and can decrease binary size because there is only one version 
      of the function.

    > If you need the highest possible performance and don't mind potentially increasing the binary size, then static dispatch
      is the way to go. If you need to store different types in the same data structure, the exact types are not known until 
      runtime, or you need the smallest possible binary, then dynamic dispatch is needed.
- What is a crate, module and package in Rust? How do they differ? How are the used? What is workspace?
    > The crate is the basic compilation unit in Rust. It can be either a binary or a library. Binary crate is an executable 
      program, while a library crate is a set of functionalities other programs can use. The crate consists of the tree of modules. 
      Modules are a way to organize code within the crate. These modules can be defined in one or more files. 
      The package is a collection of zero or one library crates and zero or more binary crates. It contains a Cargo.toml file, 
      which is a manifest that describes how to build the crates. A workspace is a set of packages that share the same Cargo.lock 
      and output directory. This allows you to group related packages together in a way that is easier to manage. For instance, 
      you might have a project that includes a library, a command-line tool that uses the library, and an example program that 
      demonstrates how to use the library. You could put all these in one workspace, which would make it easier to build and test 
      them together.
- What is cloning? What is copying? How do they compare? What is for trait drop? What is special about the trait?
    > Cloning is a way to duplicate complex data types that involve additional resources, such as heap memory.
      Types that implement the Clone trait provide a method to create a complete copy of their values, including any heap data.
  
    > Copying is a way to duplicate "plain old data" types where no additional resources are involved. In Rust types
      that implement the Copy trait can be duplicated by calling memcpy, which just copies bits on the stack.  

    > Drop is special trait in Rust that allows you to specify what happens when a value goes out of scope. When a value is about 
      to be destroyed, Rust will call the drop method on it, allowing for any necessary cleanup. For instance, it can be useful
      if a type owns data on the heap which has to be cleaned up when the value is no longer in use. This is a way of how Vec 
      handles a clean-up of the heap buffer it owns.

    > The Drop trait is special, because Rust calls its method automatically when a value goes out of scope, 
      and it can't be called manually.
- What is immutability? What is the benefit of using it? What is the difference between immutability and const?
    > Immutability is a programming property where a variable, once created, cannot be changed. In Rust all variables
      are immutable by default. The benefit of immutability by default is that you can be sure that the value won't 
      be changed unexpectedly. But if you see the "mut" keyword it is special indicator of a fact that the value may be changed
      somewhere and that you should pay attention to it.

    > In Rust const represents a value that is determined at compile time and that can't be changed throughout the program.
      Instead of being stored in memory as with regular immutable variables, the const value is embedded directly into 
      the program's binary. Essentially, const variables are just inlined to each usage location, whereas "let" creates
      an immutable variable in memory. "Const" is good for defining truly constant values like mathematical constants or 
      config parameters known at compile time. If you just need a variable that won't change its value after assignment,
      "let" is the way to go.
- What are move semantics? What are borrowing rules? What is the benefit of using them?
    > In Rust assignment of a variable to another or passing a variable as an argument to a function is "move",
      which means the ownership of the value is transferred and the original variable can no longer be used.
      Essentially, with "move" bits of a value are copied on the stack from one place to another, and the compiler just
      marks the original value as no longer valid to be used. This prevents double-free since the responsibility 
      of freeing the memory always stays only with one variable.

    > The exception to "move" behaviour is types that are Copy. With such types after memcpy of bits on the stack, 
      the original value stays valid to be used. This is because such values comprise just these bits on the stack
      and don't own resources like heap memory. So after assignment or argument passing the new value is the complete
      clone of the original value, and the original values remains absolutely untouched. That's why the original value
      continues being perfectly valid for use. 

    > Rust allows you to 'borrow' references to a value, which means accessing the value without taking ownership. 
      Borrowing comes in two flavors: mutable and immutable. Immutable borrows allow read-only access to a value,
      and mutable borrows allow modifying a value. You can have multiple immutable borrows at the same time, 
      but you cannot have an immutable borrow at the same time with a mutable borrow.

    > All these rules are established to provide memory safety to Rust programs. For instance, as we've already seen, 
      "move" prevents double-free. Borrowing rules can help preventing data races at compile time, since they ensure 
      that you can't have multiple threads simultaneously reading and writing to a variable. Also, borrowing rules save
      us from having dangling reference: for instance, if we were allowed to have mutable and immutable references 
      at the same time, we would be able to trigger the reallocation of the heap buffer by mutating the value of the Vec through 
      the mutable reference, and our immutable reference would contain the pointer to freed memory, and accessing such memory 
      is the classic case of UB.
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
    > RAII, or Resource Acquisition Is Initialization is a programming idiom that states that resource acquisition like memory 
      allocation, file opening, etc. should occur when an object is initialized, and resource deallocation like memory freeing, 
      file closing, etc. should occur when the object is destroyed.

    > In Rust, RAII is implemented through a combination of the ownership system and the Drop trait. The Drop trait, as we've 
      figured out earlier, provides a drop method that is called automatically when an object goes out of scope. You can implement 
      drop for your own types to specify what should happen when they're no longer needed, for instance, to close a file handle 
      or release network resources.

    > The main benefits of RAII is automatic resource management and exception safety. The programmer doesn't have to remember 
      to manually release resources, which reduces the risk of resource leaks. And also, if the error occurs, during the stack
      unwinding all drops are called anyways, so the resources are still properly cleaned up.
- What are lifetimes? Which problems do they solve? Which benefits do they give?
    > In Rust, lifetimes are a compile-time feature that the language uses to ensure references to data are always valid. 
      Lifetimes are a way of representing the span of time during which a piece of data is valid and can be safely used.

    > The Rust compiler uses lifetimes to prevent "dangling references". These bugs occur when 
      a reference to a value outlives the value it points to. Without lifetimes, you could potentially access data that 
      has been deallocated, which can lead to UB.

    > The primary benefit of lifetimes is they enable Rust to guarantee memory safety without needing a garbage collector. 
      The lifetimes ensure that you cannot have a reference to data that no longer exists.
- What is an iterator? What is a collection? How do they differ? How are they used?
    > An iterator is a programming concept that provides a way to access elements in a collection of items sequentially, 
      without exposing the underlying details of the collection's implementation. In Rust, the Iterator trait defines 
      a shared set of methods for all iterators. 

    > A collection is an object that groups multiple elements into a single unit. Collections are used to store, retrieve, 
      and manipulate aggregate data. Typical examples of the collection are vectors, lists, linked lists, hash maps, etc.

    > The primary role of a collection is to store and manage a group of elements. This is a data structure that offers 
      methods to add, remove, or change elements in various ways. It also provides ways to access elements, whether that 
      be by an index, a key, or some other manner depending on the type of the collection. On the other hand, an iterator's 
      primary role is not to store data but to provide a mechanism to iterate over a sequence of elements, regardless of how 
      they are stored. An iterator abstracts the process of a loop, where you can perform some operation on each item in 
      a collection.
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
    > Macros in Rust are a way of defining reusable chunks of code. They use a form of metaprogramming to generate code 
      at compile time, based on the input provided.

    > Macros are a powerful feature and solve a few important problems. Macros can help avoiding code repetition. 
      If you find yourself writing the same or very similar code in multiple places, you might consider using a macro 
      to abstract it away. Macros can be used to generate code, which is especially useful when you want to automatically 
      implement traits or functions based on some input. Also, macros help defining variadic functions. Rust does not have 
      built-in support for functions with a variable number of arguments, but it's possible can achieve similar results 
      with a macro. A good example of such variadic function implemented using macro is vec![].

    > In Rust, there are two kinds of macros: declarative macros and procedural macros. Declarative macros use 
      the macro_rules! construct. They match against the specific pattern you provide and generate code based on that pattern. 
      Declarative macros are somewhat simpler and more limited than procedural macros. Procedural macros are more complex 
      and more powerful. They're defined in their own crates, and they operate on the abstract syntax tree (AST) that Rust 
      generates when it parses your code.

    > Procedural macros come in three flavors: custom #[derive] macros that allow you to specify code to be added when 
      a trait is derived, attribute-like macros that define custom attributes usable on any item and function-like macros 
      that define macros which look like function calls, similar to macro_rules! macros, but with more flexibility.

    > The key difference between declarative and procedural macros is in their flexibility and complexity. 
      Declarative macros are simpler and have a straightforward syntax, but they can only perform text substitution. 
      Procedural macros are more complex and require separate crate types, but they can inspect and manipulate Rust's syntax tree, 
      which makes them more powerful.
- How code is tested in [Rust]? Where should you put tests and why?
    > The Rust standard library provides a basic test framework via the #[test] attribute, which you can attach to any function 
      to make it a test function. The assert_eq! macro is provided by the standard library for asserting that two values are equal. 
      If they're not, the test will fail.
  
    > There are 2 types of tests in Rust: unit tests and integration tests. Unit tests are small, focused tests that exercise 
      a specific piece of functionality in isolation. By convention, they are typically placed in the same file as the code 
      they're testing, inside a module named tests that is gated by #[cfg(test)]. This attribute tells Rust to only compile 
      and run the code under this attribute when running cargo test, not when running cargo build. Integration tests are larger 
      tests that exercise your code's public interface and may involve multiple parts of your library working together. 
      By convention, integration tests are placed in a separate directory at the root of your crate named tests. 
      Each file in this directory is compiled as a separate crate, so it can only use your library's public interface.

    > The reason for conventions of placing tests in Rust is primarily to keep tests organized and separate from your library's 
      code while ensuring they have appropriate access to the code they're testing. Unit tests need access to the internals 
      of your library's code, so they're kept in the same files but in separate #[cfg(test)] modules. Integration tests only need 
      access to your library's public interface, so they're kept in a separate tests directory.

- What is special about slice? What is layout of Rust standard data types? Difference between fat and thin pointers?
    > A slice in Rust is a dynamically-sized view into a sequence of elements in an array. It is a type that doesn't have 
      a known size at compile time, so it can only be used through a pointer of some kind. 

    > Data types in Rust can be grouped into 2 categories based on their memory layout: statically-sized types 
      and dynamically-sized types. Statically-sized types have a size that is known at compile time. These include primitive 
      types such as integers (i32, u64, etc.), booleans (bool), characters (char), and also structs, enums, and arrays of known size.
      Dynamically-sized types (DSTs), on the other hand, have a size that is not known until runtime. Examples of DSTs include 
      slices ([T]), trait objects (dyn Trait), and the special type str for string slices.

    > There are 2 groups of pointers in Rust: thin and fat pointers. Thin pointers are simple pointers that contain the memory 
      address of their data. Examples of thin pointers include references to sized types (&T, &mut T), raw pointers (*const T, 
      *mut T), and Box<T> where T is a sized type. Fat pointers are used for dynamically-sized types and consist of a thin pointer
      plus additional metadata. For slices (&[T], &mut [T]), the fat pointer consists of the thin pointer to the first element
      of the slice and the length of the slice as the metadata. For trait objects (&dyn Trait, &mut dyn Trait, Box<dyn Trait>), 
      the fat pointer consists of the thin pointer that points to the data of the object that the trait object represents
      and as the metadata it includes a pointer to a vtable.

    > The key difference between thin and fat pointers is this additional metadata. It makes fat pointers larger (typically twice 
      the size of thin pointers), but it also allows them to point to dynamically-sized types.
- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them? Why str slice coexist with slice?
    > In Rust, String and &str are two distinct types that both represent sequences of characters. The String type is a growable,
      mutable, owned, heap-allocated data structure. String is able to take ownership of its content, and it has an invariant 
      of holding only valid UTF-8 strings. The &str type is a reference to a sequence of UTF-8 characters. &str is typically used 
      in function arguments to take an arbitrary string input. &str is a view into a String (or into another &str). It does not own 
      the data it refers to, and it's immutably borrowed. str is DST, that's why it's only possible to work with it through 
      the pointer, particularly, the fat pointer. 

    > The &str and String types coexist because they serve different purposes. You would use String when you need a mutable string 
      or when you need to own the string data. &str, on the other hand, is used when you just need to read from a string, 
      not change it or own it. This can be more efficient, as it often avoids copying the string data.

    > &str and [T] are both DSTs. Slices, such as &[T], are just references to a contiguous sequence of elements (T), 
      while &str is a contiguous sequence of UTF-8 bytes. So the main difference is this invariant of &str 
      to point to valid UTF-8 bytes.
- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance? Is Rust functional language?
    > Rust has some features associated with object-oriented languages. For instance, its struct and enum types can be used 
      to represent objects, and its impl blocks can be used to define methods on those objects. It also supports interfaces 
      through its trait system. However, Rust does not support inheritance, which is a fundamental aspect of OOP in languages 
      like Java or C++. Instead of inheritance, Rust encourages using composition and trait objects.

    > SOLID principles can be applied in Rust to a certain extent. Single Responsibility Principle (SRP): functions and methods 
      in Rust can be designed to serve a single responsibility. Open-Closed Principle (OCP): through the use of traits, one can 
      extend the functionality of types without modifying their implementation. Liskov Substitution Principle (LSP): LSP can be 
      achieved by using trait bounds to enforce that a type behaves in a certain way. Interface Segregation Principle (ISP): 
      Rust's trait system allows one to define small, specific interfaces. Dependency Inversion Principle (DIP): by using traits 
      as interfaces, one can invert dependencies effectively in Rust. Similarly, many GRASP concepts can also be applied in Rust.

    > Rust also incorporates many features from functional programming languages, such as first-class and higher-order functions,
      powerful pattern matching, and a strong type inference system. It also has a number of types and methods in its standard 
      library to facilitate functional programming, like Option and Result for handling potential errors in a type-safe way, 
      and Iterator for working with sequences of values.

    > Thus, Rust provides features from both paradigms, but it's not considered a pure object-oriented or functional programming 
      language. It encourages you to use the best tool for your specific need, which may involve mixing and matching 
      different paradigms.


After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.

_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Adolfo Ochagavía: You are holding it wrong][23]
- [HashRust: A guide to closures in Rust][24]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Bradford Hovinen: Demystifying trait generics in Rust][25]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Trait Drop][10]
- [Common Lifetime Misconception][11]
- [Visualizing Memory Layout][12]
- [Package vs. Crate terminology (r/rust)][13]
- [Packages and crates (Rust wiki)][14]
- [Full list of available crates on Rust Playground][16]
- [Blanket impl definition][17]
- [Auto-trait definition][18]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][21]
- [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust][22]
- [Asynchronous vs Multithreading][29]

Additional:
- [Rust API guidline checklist][19]
- [Interview Questions on Rust Programming][20]
- [Step-by-step instruction to start development in Rust][26]
- [Awesome collection of crates for productive development in Rust][27]
- [Awesome Collection of Materials to Learn Rust][28]

[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[rustlings]: https://rustlings.cool

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://vojtechkral.github.io/blag/rust-drop-order/
[11]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
[12]: https://www.youtube.com/watch?v=rDoqT-a6UFg
[13]: https://www.reddit.com/r/rust/comments/lvtzri/confused_about_package_vs_crate_terminology/
[14]: https://rustwiki.org/en/book/ch07-01-packages-and-crates.html
[16]: https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml
[17]: https://doc.rust-lang.org/reference/glossary.html#blanket-implementation
[18]: https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits
[19]: https://rust-lang.github.io/api-guidelines/checklist.html
[20]: https://iq.opengenus.org/questions-on-rust/
[21]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[22]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[23]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[24]: https://hashrust.com/blog/a-guide-to-closures-in-rust
[25]: https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust
[26]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/introduction.md
[27]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/toolbox_general.md
[28]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/learn.md
[29]: https://github.com/Learn-Together-Pro/ComputerScience/blob/master/cheatsheets.md#asynchronous-vs-multithreading

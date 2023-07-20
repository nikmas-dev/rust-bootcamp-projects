Step 1: Concepts
================

These steps describe common and necessary-to-know concepts for everyday programming in [Rust].

> ❗️Before completing this step you should complete all its sub-steps.

After doing them you should be able to answer the following questions:
- How do I recognize that data is allocated at the heap rather than at the stack? When data should be allocated at the heap?
    > In Rust, you can tell that data is allocated on the heap if it's wrapped in a Box, a reference counting smart pointer 
      (Rc<T>, Arc<T>), or one of the standard library types that use heap allocation like Vec, String, HashMap.

    > You usually allocate data on the heap in 3 cases: you have a large amount of data that could overflow the stack,
      or it's large enough that copying it here and there through memcpy can be expensive bottleneck operation;
      you need to keep data around across function calls, or share data between different parts of a program;
      you don't know the exact amount of data you're working with at compile time.
- What is copying and cloning data in [Rust]? What's the difference? When and why should I use them?
    > In Rust, copying and cloning are mechanisms that allow you to duplicate data. Copying happens simply by copying bits on stack.
      If a value consists only of data that lives on stack, it's possible to implement the Copy trait for it to turn on the implicit 
      copy semantic for this type. Cloning is a more explicit and flexible way to duplicate values. It's done by calling the clone 
      method on a value. If a type implements the Clone trait, it means that its values can be duplicated, but the process might 
      involve more than just copying bits. For example, if a type represents a string or a vector, cloning that type's value means 
      allocating new memory on the heap and copying the contents into that memory.

    > If you need to be able to clone your data type and this data type owns some resources beyond the stack, you need to implement
      the Clone trait and handle the deep copy process. If your data type is just a bunch of bits on the stack, you can implement 
      the Copy trait for it. But it's important to pay attention to the fact that the implementation of the Copy trait is the part 
      of a data type's public API. Thus, if you add some value inside the data type that isn't Copy, you will need to remove 
      the Copy implementation from this data type. And doing this is a breaking change that needs the increment of the major version
      of your library where this data type lives.
- How can a single piece of data be owned by multiple parts of program? When and why is this commonly required?
    > In Rust, by default, each value has a single owner, and the lifetime of the value is tied to the scope of its owner. 
      However, there are situations where you may want a single piece of data to be owned by multiple parts of the program. 
      This is particularly common when you're dealing with shared state across threads, or you're implementing data structures 
      that need shared ownership semantics (e.g. graph data structures). The primary mechanism in Rust for allowing multiple 
      owners is reference counting. The Rc and Arc types provide shared ownership for single-threaded and multi-threaded 
      scenarios, respectively.
- How borrowing rules may be violated? In what price? When and why is this commonly required?
    > Rust's borrowing rules are part of its core memory safety guarantees, so they can't be violated without using unsafe code.  
      There may be situations where you're sure that what you're doing is safe, but the compiler can't verify this. That's when 
      you'd need to use unsafe code to bypass the borrowing checks. The cost of using unsafe is that you're now responsible 
      for upholding Rust's safety guarantees. If you use unsafe incorrectly, your program could have undefined behavior, 
      which can lead to bugs, security vulnerabilities, crashes, etc.

    > Also, there is a way to move borrowing rules checks from compile time to runtime by using data structures with interior
      mutability. Such structures allow you to mutate data through the shared reference. But if you try, for example, to borrow
      mutably more than one time, it'll lead to panic at runtime instead of compile time. Such behaviour may be useful in a case 
      of mock objects. You may need to mutate them for tracking purposes while they stand behind the shared reference.
- How to deal with owned and borrowed data simultaneously? When and why is this commonly required?
    > You can deal with owned and borrowed data simultaneously by using the Cow data structure. It lets you store either borrowed
      or owned data in the same type. This data structure can help reduce allocations when they're unnecessary. For instance,
      a function may return the String in some cases and &'static str in other cases. Without the Cow we would need to define
      the return type of the function as being the String and always allocate the new String by converting &'static str to String. 
      But with the Cow we can return both String and &'static str types as a one time, thus eliminating extra allocations and 
      improving the memory efficiency of our program. 
- How to share values between threads? What is `Send` and `Sync` markers? Why are they required, when should be used?
    > In Rust, values can be shared between threads through using the message passing or data structures that allow multiple ownership
      with synchronization primitives (Arc<Mutex<T>>). 

    > Send is a marker trait that indicates that an object's ownership can be transferred safely between threads. Sync is another 
      marker trait that indicates that a type is safe to share between threads by reference. These traits are auto traits meaning 
      that they're implemented automatically if all the nested types implement these traits. Sync and Send form the basis of Rust's
      fearless concurrency and allow Rust to prevent data races at compile time.
- How do static and dynamic dispatches differ? Why do they exist? When and why should I choose between them?
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
- Why `?Sized` types exist? How are they used? Why should I care about them?
    > In Rust, all types are assumed to have a known, fixed size at compile time by default. This allows Rust to make a number 
      of optimizations, like placing data on the stack, which can result in more efficient code. Types that have a fixed size 
      at compile time are known as Sized types. However, not all types can have a known size at compile time. For instance, 
      slices, trait objects, and types that include those non-sized types cannot have a known size at compile time. 
      These types are known as dynamically sized types, or DSTs. 

    > ?Sized is a marker that can be used on trait bounds to indicate that the trait can be implemented for types that may 
      or may not have a known size at compile time. By default, when you write a generic function or implement a trait for 
      a generic type, Rust assumes that the type is Sized. If you want to be able to use a generic function for types that 
      might not be Sized, you can use the ?Sized bound. Let's consider this function definition: 
      fn perform_operation<T: ?Sized>(item: &T). Because there is the & before T we can extend the spectrum of the appropriate T types
      to include DSTs. It's possible because we can store DSTs behind the reference which in this case turns into the fat pointer. 
      And this fat pointer has a known size at compile time. By marking T as ?Sized we extend the spectrum of the appropriate 
      T types making our implementation more general and flexible.
- Why phantom types exist? What problems do they solve?
    > There are situations when you need to express some type relations without having any values of those types. This is the case
      when phantom types enter the game. For instance, you may need to have some generic T parameter with a bound of implementing
      some trait with an associated function. This means that we only want to call static functions on T like this: T::function().
      In this situation we don't need to store any data inside our data structure that is of T. But Rust won't allow us to define 
      T without bounding it to some fields in our struct. In this case we can imitate it by using PhantomData<T>, which lives only
      at compile time and is totally gone at runtime.

The following articles may help you to sum up your experience:
- [Wrapper Types in Rust: Choosing Your Guarantees][1]
- [Rust, Builder Pattern, Trait Objects, `Box<T>` and `Rc<T>`][2]
- [Rust's Built-in Traits, the When, How & Why][3]
- [Learn Rust With Entirely Too Many Linked Lists][4]
- [Rustonomicon: Subtyping and Variance][13]
- [Crust of Rust: Subtyping and Variance][14]



## Task

__Estimated time__: 2 days




Provide your own implementation of [doubly linked list][11] data structure. It should be [thread safe][12] without a necessity to use explicit synchronization primitives (like `Arc<Mutex<T>>`) on top of it.

Prove your implementation correctness with tests. Provide both single-threaded and multi-threaded examples of usage.  




[Rust]: https://www.rust-lang.org

[1]: https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees
[2]: https://abronan.com/rust-trait-objects-box-and-rc
[3]: https://llogiq.github.io/2015/07/30/traits.html
[4]: https://rust-unofficial.github.io/too-many-lists/
[11]: https://en.wikipedia.org/wiki/Doubly_linked_list
[12]: https://en.wikipedia.org/wiki/Thread_safety
[13]: https://doc.rust-lang.org/nomicon/subtyping.html
[14]: https://www.youtube.com/watch?v=iVYWDIW71jk

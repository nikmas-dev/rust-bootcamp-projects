Step 3: Common ecosystem
========================

These steps describe common crates and tools in [Rust] ecosystem required for application and library development.

> ❗️Before completing this step you should complete all its sub-steps.

After doing them you should be able to answer the following questions:
- What testing capabilities does [Rust] offer and when should I use them? Why should I follow [BDD] style?
    > Rust offers 3 types of tests: unit tests, integration tests and doc-tests. Unit tests are uses to test 
      individual components in isolation. They're best for small, isolated functionalities. Integration tests 
      test interactions of modules or external services. They're useful to ensure components work together as expected.
      Doc-tests are tests written in the documentation. They ensure that examples in documentation are correct and up-to-date.
      BDD style is useful for representing a clear and readable specification of a system.
- What are macros? How do they differ? What benefits does their usage give? When should I write one?
    > Macros are a powerful metaprogramming tool that allows code generation at compile-time. There are 2 types of macros
      in Rust: declarative and procedural. Declarative macros are defined using macro_rules! syntax. Also, they're pattern-based: 
      you define patterns to match against and then specify the code to generate based on these patterns. Procedural macros
      are more powerful and complex than declarative ones. They work as functions that accept source code as input and produce 
      source code as output.

    > Macros allows to reduce code duplication and boilerplate code. If you want to create an abstraction over repetitive code 
      patterns, and it cannot be achieved properly just with function, macros are a way to go. Also, macros allow to create DSLs. 
- How to work with date and time in [Rust]? How should I store time? How should I return it to other applications?
    > Working with date and time in Rust is commonly done using the chrono and time crates. Chrono seems to be more feature rich
      ,and it also parametrizes time zone in types, while time crate handles it in runtime. It's recommended to store time in UTC
      as it's unambiguous. Also, it's common to return time as an ISO 8601 formatted string in UTC, e.g., 2023-04-01T12:00:00Z.
- How are regular expressions used in [Rust]? When are they not enough? How can I write a custom parser in [Rust]?
    > In Rust the regex crate provides the ability to use regular expressions. Regular expressions are powerful, but they have 
      limitations. For instance, they can't parse nested structures or recursive patterns (like matching pairs of opening 
      and closing parentheses). Also, they might be inefficient or unreadable for complex patterns, and they can't handle 
      context-sensitive grammar. When regular expressions aren't enough, you might need to create a custom parser in one of these
      ways: 1. Manual Parsing. You can manually iterate through the input string and parse it using Rust's powerful string 
      manipulation and pattern matching capabilities. This can be efficient but might become cumbersome for more complex grammars.
      2. Parser Combinators. Libraries like nom allow you to build parsers by combining smaller parsers. They provide a more 
      declarative way to specify the parsing logic and are more readable than manual parsing for complex grammars. 
      3. Parser Generators: Tools like lalrpop or pest can be used to generate parsers from a grammar specification. 
      You need to specify the grammar in a separate file, and the tool would generate Rust code for the parser.
- How do iterator and collection compare and differ in [Rust]? What is the purpose of immutable collections? Why should I care about using concurrent collections?
    > An iterator represents a sequence of items and provides a way to access them one by one. You can think of it as a conveyor
      belt of items where you get one item at a time, and once retrieved, it's gone from the conveyor. Iterators in Rust are lazy,
      meaning they don't do any actual work until you ask them to. Due to their lazy nature, iterators can be more memory efficient
      for certain operations, as they generate items on-the-fly rather than storing a large number of items in memory. A collection 
      is a data structure that holds multiple items. While you can obtain an iterator from a collection, the collection itself 
      is more like a container that holds items, rather than a conveyor belt. Operations on collections are eager, meaning 
      they perform the action immediately. If you filter a vector, for instance, you immediately get a new vector with 
      the filtered items. Collections occupy memory based on the number of items they hold and the type of those items.

    > Immutable collections guarantee that no modifications can happen once they are created. This ensures that data won't be 
      accidentally mutated, which is particularly beneficial in multi-threaded contexts where unexpected mutations 
      can lead to bugs.

    > Concurrent collections are designed for safe concurrent access and modification. Regular collections aren't safe 
      to be accessed or modified by multiple threads simultaneously. The benefits of using concurrent collections are
      that you can safely use them in multi-threaded applications without manual synchronization and also that they're 
      often optimized for concurrent access, reducing contention and bottlenecks.
- What should I use for serialization in [Rust]? Why this is good or bad?
    > In Rust, the go-to library for serialization is serde due to its flexibility in supporting numerous formats like 
      JSON, TOML, and YAML. It stands out because of its high performance, customization options via attributes, type safety 
      assurances, and its widespread adoption in the Rust community. However, using serde does come with a slight learning curve,
      especially for intricate customizations. The library's heavy reliance on macros can also lead to longer compilation times 
      and occasionally verbose compiler errors. It's also crucial to ensure version compatibility between serde and its various
      adapters. Despite these considerations, serde remains a top choice in the Rust ecosystem because its benefits outweigh 
      potential challenges.
- How can I generate randomness in [Rust]? Which guarantees of random generator should I choose and when?
    > In Rust, randomness is typically achieved using the rand crate. This crate provides a suite of utilities to produce 
      random numbers, samples, and distributions. For most common use-cases, the thread_rng() function provides a good balance 
      of performance and unpredictability. However, if you require cryptographic security, you should lean towards 
      the rand::rngs::OsRng generator, as it's designed to produce random numbers suitable for cryptographic operations. 
      Remember, the specific guarantees of a random number generator, such as its unpredictability and performance, 
      should match the requirements of your application. For example, while generating a game dice roll, high-speed 
      non-cryptographic randomness is adequate; but for cryptographic keys, you need strong unpredictability provided 
      by cryptographic RNGs. 
- What should I use for password hashing in [Rust]? How can I encrypt a message with [Rust]? How should I compare secret values and why?
    > In Rust, for password hashing, the argon2 crate is a solid choice, being a winner of the Password Hashing Competition. 
      It provides strong, tunable security against various attacks. When it comes to encrypting messages, the rust_crypto 
      library offers a variety of cryptographic primitives, but the ring crate is a more modern and commonly recommended option.
      When comparing secret values, such as cryptographic hashes, always use a constant-time comparison method to prevent timing 
      attacks. This means that, regardless of the data being compared, the time it takes to compare two values is always the same.
      Libraries like ring usually offer these utilities out-of-the-box to ensure that your cryptographic operations are both safe
      and performant.
- How logging is organized in [Rust] ecosystem? Why should I care about structured logging?
    > In the Rust ecosystem, logging is primarily facilitated by the log crate, which defines logging macros and a trait for 
      creating custom loggers. Implementations like slog and tracing extend this base. The tracing crate, in particular, 
      emphasizes structured logging. Structured logging allows you to capture logs as more than just strings—instead, 
      they're structured data like key-value pairs. This is valuable because it makes logs more meaningful and easily 
      queryable. For instance, instead of manually parsing logs to extract specific details, with structured logs, you can 
      directly filter or search for logs based on specific attributes. It enhances debugging, observability, and analytics, 
      making it easier to derive insights and diagnose issues in complex systems.
- What should I use for building [CLI] interface in [Rust]? How can I organize a configuration for my application and why?
    > In Rust, the go-to library for building CLI interfaces is clap, with its derive cargo feature offering a convenient way 
      to define CLI arguments using struct definitions. For application configuration, the config crate is popular as it 
      consolidates configurations from various sources: files, environment variables, and more. Organizing configuration 
      is crucial as it makes your application adaptable to different environments and use cases without code changes. 
      By separating the configuration from the code, you enhance maintainability and flexibility, allowing settings to be 
      adjusted easily without recompilation. Combining clap and config gives you a powerful toolkit for crafting adaptable 
      and user-friendly applications.
- Why multithreading is required for [Rust] programs and what problems does it solve? How threads concurrency differs with parallelism? How can I parallelize code in [Rust]?
    > Multithreading in Rust empowers programs to efficiently utilize multi-core processors, enhancing performance, especially 
      for cpu-bound tasks. It helps in achieving both concurrency (multiple tasks making progress without necessarily 
      simultaneously executing) and parallelism (tasks running simultaneously on different cores). While concurrency deals 
      with managing multiple tasks efficiently, parallelism focuses on simultaneous execution for speed-up. Rust's standard 
      library provides primitives for threading, but for easier parallelization, the rayon crate is commonly used. It extends 
      iterators, allowing data to be processed in parallel with minimal code changes. Leveraging multithreading and parallelism, 
      Rust programs can scale gracefully with available hardware resources.
- What is asynchronicity and what problems does it solve? How is it compared to threads concurrency? What is [Rust] solution for asynchronicity and why it has such design?
    > Asynchronicity is a programming paradigm that enables non-blocking operations, allowing systems to perform other tasks 
      instead of waiting, thus optimizing resource usage. It's particularly beneficial for IO-bound tasks like network or 
      database operations. Compared to thread concurrency, asynchronicity avoids the overhead of thread creation and context 
      switching, often leading to more scalable and efficient systems for IO-heavy workloads. Rust's solution is its async/await 
      syntax combined with the tokio runtime. This design marries Rust's safety principles with performance, ensuring safe 
      concurrent execution without data races. The choice of an explicit async model prevents inadvertent performance pitfalls, 
      guiding developers towards more predictable and efficient asynchronous code.
- What are actors? When are they useful?
    > Actors are a computational model for concurrent computation where entities, called "actors," communicate solely through 
      message-passing. Each actor can process messages, send messages to other actors, and spawn new actors. They're isolated,
      meaning they don't share state with other actors, which naturally avoids many concurrency issues. Actors are useful 
      in systems where components need to operate concurrently but without the complexities of shared state. They shine 
      in scenarios with high concurrency, like web servers, distributed systems, or real-time applications. The actor model 
      provides a higher-level abstraction over threads, making concurrency easier to reason about and often leading to more 
      scalable and maintainable systems.


## Some usefull tools

- [cross-rs/cross](https://github.com/cross-rs/cross)
- [cargo-hack](https://github.com/taiki-e/cargo-hack)
- [Miri: unsafe core interpreter](https://github.com/rust-lang/miri)
- [cargo-outdated](https://crates.io/crates/cargo-outdated)
- [cargo-modules](https://github.com/regexident/cargo-modules)
- [cargo-make](https://github.com/sagiegurari/cargo-make)
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [Rust Developer's Toolbox](https://github.com/rust-lang-ua/learn_rust_together/blob/master/toolbox_general.md)

## Task

__Estimated time__: 2 days




Write a [CLI] tool for stripping [JPEG] images [metadata][21] and minimizing their size (a simplified analogue of [tinyjpg.com]).

Requirements:
- Accept input list of files and remote [URL]s via: either [CLI] arguments, [STDIN], or read it from a specified file ([EOL]-separated).
- Allow configuring how much images are processed at the same time.
- Allow configuring the output directory to store processed images in.
- Allow configuring the output [JPEG] quality of processed images.
- Read configuration with ascending priority from: a file (format is on your choice), [environment variables][22], [CLI] arguments. All are optional for specifying.
- Support `RUST_LOG` environment variable, allowing granular tuning of log levels per module.
- Print execution time in logs, so it's easy to see how much which operation takes during the execution.

If you have enough time after implementing base requirements, consider to add the following to your solution:
- Allow configuring download speed limit for images from remote [URL]s.
- Cover your implementation with unit and E2E tests.
- Support [PNG] images as well.
- Add comprehensive documentation to your code.




[BDD]: https://en.wikipedia.org/wiki/Behavior-driven_development
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[EOL]: https://en.wikipedia.org/wiki/Newline
[JPEG]: https://en.wikipedia.org/wiki/JPEG
[PNG]: https://en.wikipedia.org/wiki/PNG
[Rust]: https://www.rust-lang.org
[STDIN]: https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)
[tinyjpg.com]: https://tinyjpg.com
[URL]: https://en.wikipedia.org/wiki/URL

[21]: https://picvario.com/what-is-image-metadata-role-and-benefits
[22]: https://en.wikipedia.org/wiki/Environment_variable

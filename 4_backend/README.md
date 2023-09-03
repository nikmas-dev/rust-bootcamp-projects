Step 4: Backend ecosystem
=========================

These steps describe common crates and tools in [Rust] ecosystem required for web backend development.

> ❗️Before completing this step you should complete all its sub-steps.

After doing them you should be able to answer the following questions:
- How should I interact with databases in [Rust] application and why? How can I organize migrations for my project?
    > One can interact with databases in Rust with no-dsl toolkit such as sqlx, query builders such as sea-query 
      and full-fledged ORMs such as diesel and sea-orm. If you want to have a full control over your queries
      and be able to use the latest features of the particular DBMS, then the raw sql with sqlx is the way to go.
      If you want to interact with databases with using the Rust language, then query builders and ORMs are the way to go.

    > Migrations are a way to evolve your database schema over time without manually managing the SQL schema. For managing 
      migrations, tools like diesel_cli for Diesel or sqlx-cli for SQLx can be employed. They allow you to create, run, 
      and manage migrations, ensuring your database schema is up-to-date and consistent across environments.
- What should I use for [HTTP] server implementation in [Rust], when and why? What about [WebSocket] connections?
    > For HTTP server implementation in Rust, Axum stands out as great choice nowadays due to its modularity, ergonomics, 
      and being built on the robust hyper foundation.

    > Numerous HTTP solutions in Rust don't come with native WebSocket support. To fill this gap, the tungstenite crate was 
      developed, offering a foundational and neutral WebSocket mechanism. To cater to different ecosystems and async runtimes, 
      crates such as async-tungstenite and tokio-tungstenite are available, furnishing out-of-the-box client/server integrations. 
      In the realm of the actix-web ecosystem, the preferred approach lies with the actix-web-actors::ws module, which furnishes 
      WebSocket functionalities through the actor model of actix. Notably, for those embracing the axum framework, its integration 
      with the hyper foundation simplifies WebSocket interactions, bridging the gap between HTTP and WebSocket communication 
      seamlessly.
- What are options for making [HTTP] request (including [WebSocket] ones)?
    > To make HTTP requests in Rust, you can leverage the reqwest crate, known for its simplicity and asynchronous capabilities. 
      For WebSocket interactions, tungstenite and its async variants like async-tungstenite or tokio-tungstenite are popular choices.
      Additionally, frameworks such as actix-web and axum provide integrated tools for both HTTP and WebSocket communication, 
      streamlining the process in a unified environment.
- What is [RPC]? Name several the most adopted technologies, their advantages and disadvantages, explain which one could be used under which circumstances, and what and where is their best fit? 
    > RPC, or Remote Procedure Call, is a protocol that one program can use to request a service from a program located on another 
      computer in a network. Some of the most adopted RPC technologies include:

    > gRPC: Developed by Google, it uses Protocol Buffers as its interface definition language. It's highly efficient, supports 
      multiple programming languages, and is suitable for microservices due to its performance and bi-directional streaming. 
      However, it requires learning Protocol Buffers and might be overkill for simple applications.

    > JSON-RPC and XML-RPC: Both are lightweight and language-agnostic, allowing for data to be processed as either JSON 
      (in the case of JSON-RPC) or XML (for XML-RPC). Their simplicity is beneficial for straightforward implementations, 
      but they lack features compared to more modern RPC solutions.

    > SOAP: It's a protocol, primarily using XML, known for its robustness and features like WS-ReliableMessaging. However, it's 
      more heavyweight than alternatives and has fallen out of favor due to its complexity.

    > The best fit for each depends on the specific use-case. For high-performance microservices or when working within Google's
      ecosystem, gRPC shines. For simpler implementations where language-agnosticism is paramount, JSON-RPC or XML-RPC might be 
      ideal. SOAP is best suited for legacy systems or applications that specifically demand its unique features.




## Task

__Estimated time__: 3 days




Write a simple [GraphQL] API server with the following data model:
- `User` has `id` (unique), `name` (unique) and `friends` (list of other `User`s) fields.
- `User` is able to authenticate with its `password`.

API requirements:
- Ability to register users.
- Ability to authenticate users.
- Ability to retrieve a single user and all its friends (with their friends) (should require authorization).
- Ability to add some user to friends list and remove from there (should require authorization).

Web frameworks, tools and database choices are up to you. Keep things simple to fit into the dedicated time.

If you have enough time after implementing base requirements, consider to add the following to your solution:
- Provide migrations for database schema (if possible).
- Add comprehensive documentation to your code and [API], and generate it in [HTML] form.
- Cover your implementation with unit and E2E tests.
- Implement [GraphQL] query [depth limiting][21].
- Use [dataloading][22] to optimize interaction with database in [GraphQL] resolvers. 




[API]: https://en.wikipedia.org/wiki/API
[GraphQL]: https://graphql.org/learn
[HTML]: https://en.wikipedia.org/wiki/HTML
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[RPC]: https://en.wikipedia.org/wiki/Remote_procedure_call
[Rust]: https://www.rust-lang.org
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

[21]: https://escape.tech/blog/cyclic-queries-and-depth-limit
[22]: https://medium.com/the-marcy-lab-school/how-to-use-dataloader-js-9727c527efd0

# actix_data
Example of how to use take_payload and set_payload in acti-web middleware.

This demo follows the Middleware demo at [Actix doc](https://actix.rs/docs/middleware/).

Most of the code is boiler plate to implement the Service and Transform trates for the middleware. There are several examples available that show how to read the request body also.  This crate shows how to return it to the request, so that the downstream route handlers can process the data normally:

```rust
            let (_, mut payload) = Payload::create(true);
            payload.unread_data(body.into());
            req.set_payload(payload.into());
```

Of course, this has the obvious negative impact of streaming the request body twice for every call.  A better approach might be to add state to the request, and let the route handlers determine whether to use the new state or process the request body.
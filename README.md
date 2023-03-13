Using ext trait:

```
error[E0599]: the method `ready` exists for struct `Retry<RetryPolicy, RateLimit<Client<HttpConnector>>>`, but its trait bounds were not satisfied
  --> src/main.rs:15:13
   |
15 |       service.ready();
   |               ^^^^^ method cannot be called on `Retry<RetryPolicy, RateLimit<Client<HttpConnector>>>` due to unsatisfied trait bounds
   |
  ::: /Users/luciofra/.cargo/registry/src/github.com-1ecc6299db9ec823/tower-0.4.13/src/retry/mod.rs:16:1
   |
16 | / pin_project! {
17 | |     /// Configure retrying requests of "failed" responses.
18 | |     ///
19 | |     /// A [`Policy`] classifies what is a "failed" response.
...  |
25 | |     }
26 | | }
   | | -
   | | |
   | |_doesn't satisfy `_: Service<_>`
   |   doesn't satisfy `_: ServiceExt<_>`
   |
   = note: the following trait bounds were not satisfied:
           `Retry<RetryPolicy, RateLimit<Client<HttpConnector>>>: Service<_>`
           which is required by `Retry<RetryPolicy, RateLimit<Client<HttpConnector>>>: ServiceExt<_>`

warning: unused import: `ServiceExt`
 --> src/main.rs:4:53
  |
4 | use tower::{retry::Policy, Service, ServiceBuilder, ServiceExt};
  |                                                     ^^^^^^^^^^
  ```

When using call directly instead of `ServiceExt::ready` we get the compiler to
tell us that there is a missing clone impl required by Retry, with just the ext
trait we get a cryptic error.

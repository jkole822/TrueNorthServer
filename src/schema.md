# Lessons Learned from `schema.rs` (Rust + async-graphql)

---

## 📦 Schema Structure

* `QueryRoot` and `MutationRoot` are the core GraphQL resolver entry points.
* Implemented using `#[Object] impl` blocks from `async-graphql`.
* The schema is constructed using:

  ```rust
  Schema::build(QueryRoot, MutationRoot, EmptySubscription)
      .data(pool.clone())
      .finish()
  ```

---

## 🔁 Lifetimes and References

* `&self` in resolvers: allows resolvers to use instance data without ownership.
* `ctx: &Context<'_>`: The underscore (`'_`) is an *elided* lifetime, allowing Rust to infer it. It's needed because `Context` holds references that must live at least as long as the function.
* You only specify lifetimes in return types if those references are part of the return value (e.g. returning `&str`).

---

## 🎯 `.map`, `.into_iter`, `.collect`

* `.map(...)` is used to transform each element in a container (`Vec`, `Option`, etc).
* `.into_iter()` consumes a `Vec` and produces an iterator over its elements.
* `.collect()` transforms an iterator into a concrete collection like `Vec<_>`.
* For `Option<T>`, `.map(...)` applies a function if the value is `Some`, or does nothing if `None`.

---

## ❓ `?` vs `unwrap()`

* `?` is used to propagate errors upward safely and idiomatically.

  ```rust
  let created_at = some_string.parse::<DateTime<Utc>>()?;
  ```
* `unwrap()` panics on error, and should be avoided in production code.
* Inside closures like `.map(...)`, you **cannot** use `?` unless the closure returns a `Result` or `Option`.

---

## 🧪 SQL Result Mapping

* `DecisionRow` is a `#[derive(FromRow)]` struct used for DB result mapping.
* `Decision` is a `#[derive(SimpleObject)]` used for GraphQL responses.
* You often transform `DecisionRow -> Decision` manually, parsing `created_at` into a `DateTime<Utc>`.

---

## 🔐 Error Handling with `.map_err` and `.extend_with`

* Instead of crashing on parse errors, we do:

  ```rust
  .map_err(|e| Error::new("Invalid datetime").extend_with(|_, ext| ext.set("source", e.to_string())))
  ```
* This gives clients structured, human-readable errors in the GraphQL response.

---

## ✨ Optional Values and `.map` with `Option`

* In `decision_by_id`, we use `.map(...)` on `Option<DecisionRow>` to transform to `Option<Decision>`.
* You don’t need `.into_iter()` or `.collect()` for single values.

---

## ✅ Pattern Matching Instead of `.map` (for `?` use)

When needing to use `?` inside `.map(...)`, use a `match` instead:

```rust
match decision_row {
    Some(row) => {
        let created_at = row.created_at.parse()?;
        Some(Decision { ... })
    },
    None => None,
}
```

---

## 🌱 Future Refactors

* Implement `TryFrom<DecisionRow> for Decision` to centralize transformation and error handling.
* Abstract away `parse()` + `map_err(...)` logic for reuse.

---

This session showed the deep ergonomics of Rust with GraphQL, highlighting type safety, composability, and expressive error handling. Excellent work digging into the internals!

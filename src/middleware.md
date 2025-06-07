# 🛠️ Rust Power Tool Cheat Sheet for Kole

This cheat sheet is a quick-reference guide to the most common, powerful, and confusing Rust expressions you're encountering in real-world development. Use this when your brain starts melting. You're not alone. You're just learning high-octane programming.

---

## ✅ `.into()`

**What it does:** Converts a value into another type, if that conversion is defined.

```rust
let s: String = "hello".into(); // same as String::from("hello")
```

Use when:

* You want to convert `&str` into `String`
* You want cleaner code than `String::from(...)`
* A function expects a different type, but `into()` is implemented for your value

---

## ✅ `.ok_or(...)`

**What it does:** Turns an `Option<T>` into a `Result<T, E>`.

```rust
let val = maybe_thing.ok_or("Missing thing")?;
```

Use when:

* You have an `Option` but your function needs to return `Result`
* You want to use `?` to bail early on `None`

---

## ✅ `.and_then(...)`

**What it does:** Chain computations that return `Option<T>` or `Result<T, E>`.

```rust
some_option.and_then(|v| Some(v + 1));
```

Use when:

* You want to perform more logic if the previous result was `Some` or `Ok`
* You want to avoid a deep nesting of `match` or `if let`

---

## ✅ `Result<Self, Error>`

**What it means:** The function returns either:

* `Ok(Self)` — the type being implemented (e.g. in `impl FromRequestParts for AuthUser`)
* `Err(Error)` — usually something like `async_graphql::Error`, `axum::Error`, or custom

Use when:

* You're implementing a trait or factory method like `.new()` or `FromRequest`

---

## ✅ `Self`

**What it means:** The type being implemented.

```rust
impl MyType {
    fn new(...) -> Self {
        ...
    }
}
```

---

## ✅ `&S` or `&State`

This is a generic borrowed reference to some shared state (like a database pool or app settings). Used in things like:

```rust
async fn from_request_parts(..., state: &S) -> Result<Self, ...>
```

You're not expected to use `state` in all cases — but the function signature still requires it.

---

## ✨ The Pattern: `if let Some(val) = thing { ... } else { return Err(...) }`

Shorthand for matching against an `Option` without full pattern matching:

```rust
let Some(user) = user else {
    return Err(Error::new("Not found"));
};
```

This avoids verbose matching and is very readable.

---

## ✅ `unwrap()` vs `expect(...)` vs `?`

* `unwrap()` panics with no context on failure
* `expect("msg")` panics **with a helpful message**
* `?` returns early with `Err(...)` in a `Result`-returning function

Prefer `?` in production. Use `unwrap`/`expect` only when you're *100%* sure it won't fail.


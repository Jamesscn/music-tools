# Contributing

Contributions are allowed however they should adhere to the guidelines stated below.

## Linting and format

Any code must be formatted with the rustfmt configuration included in this repository. All clippy lints must also pass, and exceptions should be avoided if possible.

## Structures

Structures should try to implement the following traits if it makes sense:

- PartialEq
- Eq
- PartialOrd
- Ord
- Copy
- Clone
- Debug
- Default
- Display
- Hash
- Iterator
- IntoIterator
- FromStr

They should also implement the From, TryFrom and FromIterator traits for other types wherever possible.

It is recommended that structures implement a function like the following (assuming the structure is called S):

```rust
S::from_string(string: &str) -> Result<S, InputError>;
```

and also the TryFrom and FromStr traits as follows:

```rust
impl TryFrom<&str> for S {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

impl TryFrom<String> for S {
    type Error = InputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_string(&value)
    }
}

impl FromStr for S {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}
```

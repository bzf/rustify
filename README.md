rustify
-------

*A library for using the `libspotify` API in Rust.*

[![CircleCI](https://circleci.com/gh/bzf/rustify.svg?style=svg)](https://circleci.com/gh/bzf/rustify)

## Building
```sh
$ cargo build
```

## Examples
All the examples required you to have your own application key to use the
`libspotify` API, which you can get [here](https://devaccount.spotify.com/my-account/keys/).

```rust
// examples/application_key.rs
pub fn get() -> Vec<u8> {
  return vec![
    // Your C-Code key goes here
  ];
}
```

### `tekno`
This example loads the track ['Den modernitet som aldrig kom' by
1900](https://open.spotify.com/track/79ORARO8rXmk1ap0sfMPyC) and plays it using
`OpenAL`.

```
$ cargo run --example tekno USERNAME PASSWORD
```

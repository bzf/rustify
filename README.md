rustify
=======
A library for using the `libspotify` API in Rust.

## Building
```sh
$ cargo build
```

## Examples
All the examples required you to have your own application key to use the
`libspotify` API, which you can get [here](https://devaccount.spotify.com/my-account/keys/).

The examples fetches the application key from the `get()` function in the
`examples/application_key.rs` file. Remove the `panic!` call and return your
own application key from that function.


### `tekno`
This example loads the track ['Den modernitet som aldrig kom' by
1900](https://open.spotify.com/track/79ORARO8rXmk1ap0sfMPyC) and plays it using
`OpenAL`.

```
$ cargo run --example tekno USERNAME PASSWORD
```

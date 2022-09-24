# RustlePoc

**A POC using Elixir -> Rustler -> Rust**

### NOTES
  There seem to be a couple of different approaches to error handling with a Rust NIF.
  The more Rust way seems to be let it blow up, which on the Elixir side becomes an `ElrangError`:

  ```
  thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: IoError("No such file or directory (os error 2)")', src/lib.rs:16:60
  ```

  ```
  ** (ErlangError) Erlang error: :nif_panicked
  code: assert RustlePoc.Mmdb.country_code_for_ip_address(ip_address, db_path) == {:ok, "SE"}
  stacktrace:
   (rustle_poc 0.1.0) RustlePoc.Native.country_code_for_ip_address("89.160.20.112", "GeoIP2-Country.mmdb")
   test/rustle_poc_test.exs:9: (test)
   ```

  I did not find this helpful at all to the Elixir calling code

  A more Elixir way is to match all error cases in Rust code and return `{:ok, _} | {:error, _}`
  This gives the calling code a nice expectation of what can happen and `Ruslter` makes it simple to return
  tuples like this.

  I think another ELixir way would be to return Elixir Error structs instead of and error tuple

  I also think there may be a more in between way that handles the errors at this level
  and can return `{:ok, _} | {:error, _}`
  Either way some where has to handle the errors and return something usable back.

  I would like find a better way in Rust, I am not fond of the Javascript style Flying V formation
 ![](https://s3-us-west-2.amazonaws.com/courses-images/wp-content/uploads/sites/1865/2017/05/04203601/Screen-Shot-2016-06-21-at-10.52.04-AM-300x186.png)

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `rustle_poc` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:rustle_poc, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/rustle_poc>.


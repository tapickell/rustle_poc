defmodule RustlePoc.Mmdb do
  # NOTE - There are a couple of different approaches to error handling
  # with a Rust NIF. The more Rust way seems to be let it blow up, which on the Elixir side becomes an ElrangError:
  #
  # thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: IoError("No such file or directory (os error 2)")', src/lib.rs:16:60
  #
  # ** (ErlangError) Erlang error: :nif_panicked
  # code: assert RustlePoc.Mmdb.country_code_for_ip_address(ip_address, db_path) == {:ok, "SE"}
  # stacktrace:
  #  (rustle_poc 0.1.0) RustlePoc.Native.country_code_for_ip_address("89.160.20.112", "GeoIP2-Country.mmdb")
  #  test/rustle_poc_test.exs:9: (test)
  #
  # I did not find this helpful at all to the Elixir calling code, nor the developer perspective to debug the error
  #
  # A more Elixir way is to match all error cases in Rust code and return {:ok, _} | {:error, _}
  # I think another ELixir way would be to return Elixir Error structs instead of and error tuple
  #
  # I also think there may be a more in between way that handles the errors at this level
  # and can return {:ok, _} | {:error, _}
  # Either way somehwere has to handle the errors and return something usable back.

  @spec country_code_for_ip_address(String.t(), String.t()) ::
          {:ok, String.t()} | {:error, atom()}
  def country_code_for_ip_address(ip_address, db_path) do
    RustlePoc.Native.country_code_for_ip_address(ip_address, db_path)
  end
end

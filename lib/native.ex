defmodule RustlePoc.Native do
  use Rustler, otp_app: :rustle_poc, crate: "mmdb_client"

  def country_code_for_ip_address(_ip_address, _db_path), do: :erlang.nif_error(:nif_not_loaded)
end

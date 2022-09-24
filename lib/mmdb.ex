defmodule RustlePoc.Mmdb do
  @spec country_code_for_ip_address(String.t(), String.t()) ::
          {:ok, String.t()} | {:error, atom()}
  def country_code_for_ip_address(ip_address, db_path) do
    RustlePoc.Native.country_code_for_ip_address(ip_address, db_path)
  end
end

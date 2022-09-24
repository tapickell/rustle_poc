defmodule RustlePocTest do
  use ExUnit.Case
  doctest RustlePoc

  @file_path "GeoIP2-Country-Test.mmdb"

  describe "country_code_for_ip_address/2" do
    test "where is file" do
      assert File.exists?(@file_path)
    end

    test "happy path" do
      db_path = @file_path
      ip_address = "89.160.20.112"
      assert RustlePoc.Mmdb.country_code_for_ip_address(ip_address, db_path) == {:ok, "SE"}
    end

    test "db file not found" do
      db_path = "Nope.mmdb"
      ip_address = "89.160.20.112"

      assert RustlePoc.Mmdb.country_code_for_ip_address(ip_address, db_path) ==
               {:error, :database_not_found}
    end

    test "ip address parse error" do
      db_path = @file_path
      ip_address = "89.160.20"

      assert RustlePoc.Mmdb.country_code_for_ip_address(ip_address, db_path) ==
               {:error, :ip_address_invalid}
    end

    test "address not in db" do
      db_path = @file_path
      ip_address = "64.251.151.143"

      assert RustlePoc.Mmdb.country_code_for_ip_address(ip_address, db_path) ==
               {:error, :address_not_found_in_database}
    end
  end
end

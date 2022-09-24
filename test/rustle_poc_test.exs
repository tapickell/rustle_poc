defmodule RustlePocTest do
  use ExUnit.Case
  doctest RustlePoc

  test "greets the world" do
    assert RustlePoc.hello() == :world
  end
end

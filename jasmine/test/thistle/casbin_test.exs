defmodule Thistle.CasbinTest do
  use ExUnit.Case

  require Logger

  test "casbin" do
    assert Thistle.NIF.add_roles_for_user(1, 2) == 3
  end
end

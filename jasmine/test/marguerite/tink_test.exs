defmodule Marguerite.TinkTest do
  use ExUnit.Case

  require Logger

  test "jwt" do
    assert 1+1 == 2
  end

  test "hmac" do
    hi = "Hello, jasmine!"
    code = Marguerite.NIF.hmac_sign(hi)
    Logger.info("hmac(#{hi}): #{code |> Base.encode64(padding: false)}")
    assert Marguerite.NIF.hmac_verify(code, hi)
    assert !Marguerite.NIF.hmac_verify(code, "Hi")
  end

  test "aes" do
    assert 1+1 == 2
  end
end

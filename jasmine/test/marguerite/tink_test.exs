defmodule Marguerite.TinkTest do
  use ExUnit.Case

  require Logger

  test "jwt" do
    issuer = "iii"
    subject = "sss"
    audience = "aaa"
    payload = "{\"id\":1}"
    not_before = DateTime.utc_now() |> DateTime.to_unix()
    expires_at = not_before + 60*5

    token = Marguerite.NIF.jwt_sign(issuer, subject, audience, not_before, expires_at, payload)
    Logger.info("jwt token: #{token}")

    {subject1, payload1} = Marguerite.NIF.jwt_verify(token, issuer, audience)
    assert subject1 == subject
    assert payload1 == payload
  end

  test "hmac" do
    hi = "Hello, jasmine!"
    code = Marguerite.NIF.hmac_sign(hi)
    Logger.info("hmac(#{hi}): #{code |> Base.encode64(padding: false)}")
    assert Marguerite.NIF.hmac_verify(code, hi)
    assert !Marguerite.NIF.hmac_verify(code, "Hi")
  end

  test "aes" do
    hi = "Hello, jasmine!"
    code = Marguerite.NIF.aes_encrypt(hi)
    Logger.info("aes(#{hi}): #{code |> Base.encode64(padding: false)}")

    tmp = Marguerite.NIF.aes_decrypt(code)
    assert tmp == hi
  end
end

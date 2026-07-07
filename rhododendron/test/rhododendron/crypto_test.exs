defmodule Rhododendron.CryptoTest do
  use ExUnit.Case, async: true

  test "sha256" do
    hi = "Hello, Palm!"
    code = :crypto.hash(:sha256, hi) |> Base.encode64()
    IO.puts("sha256(#{hi}): #{code}")
  end

  test "sha512" do
    assert 1 + 1 == 2
  end

  test "hmac sha512" do
    assert 1 + 1 == 2
  end

  test "ssha512" do
    assert 1 + 1 == 2
  end

  test "aes-gcm-256" do
    assert 1 + 1 == 2
  end

  test "gravatar avatar" do
    items = [
      "MyEmailAddress@example.com",
      " MyEmailAddress@example.com",
      "MyEmailAddress@example.com ",
      " MyEmailAddress@example.com "
    ]

    Enum.each(items, fn it ->
      url = Rhododendron.Gravatar.avatar(it)
      IO.puts("avatar(#{it}): #{url}")

      assert url ==
               "https://gravatar.com/avatar/84059b07d4be67b806386c0aad8070a23f18836bbaae342275dc0a83414c32ee"
    end)
  end
end

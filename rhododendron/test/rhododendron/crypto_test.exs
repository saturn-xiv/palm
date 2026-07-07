defmodule Rhododendron.CryptoTest do
  use ExUnit.Case, async: true

  test "sha256" do
    code = :crypto.hash(:sha256, hi()) |> Base.encode64()
    IO.puts("sha256(#{hi()}): #{code}")
  end

  test "sha512" do
    assert 1 + 1 == 2
  end

  test "password hashing" do
    Enum.each(1..3, fn _ ->
      {:ok, hashed} = Rhododendron.PasswordHashing.sign(hi(), 16)
      IO.puts("Argon2id('#{hi()}'): #{hashed}")
      {:ok} = Rhododendron.PasswordHashing.verify(hashed, hi())
    end)

    assert 1 + 1 == 2
  end

  test "ssha512" do
    Enum.each(1..3, fn _ ->
      hashed = Rhododendron.Ssha512.sign(hi(), 16)
      IO.puts("doveadm pw -t '#{hashed}' -p '#{hi()}'")
      {:ok} = Rhododendron.Ssha512.verify(hashed, hi())
    end)
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

  defp hi do
    "Hello, Palm!"
  end
end

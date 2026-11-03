defmodule BougainvilleaTest.Hibiscus do
  use ExUnit.Case

  test "Hibiscus version" do
    {:ok, v} = Hibiscus.version()
    IO.puts("Hibiscus version: #{v}")
  end

  test "Hibiscus argon2" do
    password = "Hello, Bougainvillea!"
    {:ok, hashed} = Hibiscus.Argon2.encode(password)
    IO.puts("Hibiscus argon2 encode: #{hashed}")
    {:ok, v} = Hibiscus.Argon2.verify(password, hashed)
    IO.puts("Hibiscus argon2 verify: #{v}")
  end
end

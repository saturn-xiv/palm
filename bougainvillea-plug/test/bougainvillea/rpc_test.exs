defmodule BougainvilleaTest.Hibiscus do
  use ExUnit.Case
  @hi "Hello, Bougainvillea!"

  test "loquat argon2" do
    Enum.each(1..3, fn i ->
      {:ok, hashed} = Loquat.Argon2.sign(@hi)
      IO.puts("argon2 sign(#{i}): #{hashed}")
      assert Loquat.Argon2.verify?(hashed, @hi)
    end)
  end
end

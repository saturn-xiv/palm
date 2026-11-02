defmodule BougainvilleaTest.Hibiscus do
  use ExUnit.Case

  test "Hibiscus version" do
    {:ok, v} = Hibiscus.version()
    IO.puts("Hibiscus #{v}")
  end
end

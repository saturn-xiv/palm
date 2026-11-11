defmodule Hibiscus.Random do
  @alphanumeric "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
  @alphanumeric_size String.length(@alphanumeric)

  def alphanumeric(len) do
    for _ <- 1..len, into: "" do
      String.at(@alphanumeric, :crypto.strong_rand_range(@alphanumeric_size))
    end
  end

  def bytes(len) do
    :crypto.strong_rand_bytes(len)
  end
end

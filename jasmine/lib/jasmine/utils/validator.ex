defmodule Jasmine.Utils.Validator do
  require Logger

  def email(s) do
    v = s |> String.downcase() |> String.trim()
    # TODO
    {:ok, v}
  end

  def name(s) do
    v = s |> String.trim()
    # TODO
    {:ok, v}
  end

  def code(s) do
    v = s |> String.downcase() |> String.trim()
    # TODO
    {:ok, v}
  end

  def password(s) when is_bitstring(s) do
    l = String.length(s)

    if l >= 6 and l <= 32 do
      {:ok, s}
    else
      {:error, :invalid_length}
    end
  end
end

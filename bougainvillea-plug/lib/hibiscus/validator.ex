defmodule Hibiscus.Validator do
  def email(s, max \\ 63) do
    s = s |> String.trim() |> String.downcase()
    l = String.length(s)

    if l <= max && Regex.match?(~r/^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$/, s) do
      {:ok, s}
    else
      {:error, "#{s} isn't a valid email address"}
    end
  end

  def password(s, min \\ 6, max \\ 31) do
    s = s |> String.trim()
    l = String.length(s)

    if l >= min and l <= max do
      {:ok, s}
    else
      {:error, "Not a valid password"}
    end
  end

  def name(s, min \\ 2, max \\ 31) do
    l = String.length(s)

    if l >= min and l <= max do
      {:ok, s}
    else
      {:error, "Not a valid name"}
    end
  end

  def code(s, min \\ 2, max \\ 31) do
    l = String.length(s)

    if (l >= min and l <= max) && Regex.match?(~r/^[a-zA-Z0-9.-_]+$/, s) do
      {:ok, s}
    else
      {:error, "#{s} isn't a valid code"}
    end
  end
end

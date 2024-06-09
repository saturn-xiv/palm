defmodule Tuberose.Validation do
  def email!(s) when is_binary(s) do
    s = String.trim(s) |> String.downcase()
    l = String.length(s)

    unless l >= 5 and l <= 127 and
             String.match?(s, ~r/^[a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$/) do
      raise ArgumentError, "not a valid email address"
    end

    s
  end

  def code!(s, min_len \\ 1, max_len \\ 63) when is_binary(s) and min_len > 0 and max_len > 0 do
    s = String.trim(s) |> String.downcase()
    l = String.length(s)

    unless l >= min_len and l <= max_len and String.match?(s, ~r/^[a-z][-._a-z0-9]+[a-z0-9]$/) do
      raise ArgumentError, "not a valid code"
    end

    s
  end

  def language_code!(s) when is_binary(s) do
    s = String.trim(s)
    l = String.length(s)

    unless l >= 2 and l <= 15 and String.match?(s, ~r/^[a-z][-._a-zA-Z]+[a-zA-Z]$/) do
      raise ArgumentError, "not a valid language code"
    end

    s
  end

  def timezone!(s) when is_binary(s) do
    s = String.trim(s)
    l = String.length(s)

    unless l >= 2 and l <= 31 and String.match?(s, ~r/^[A-Z][\/a-zA-Z]+[a-zA-Z]$/) do
      raise ArgumentError, "not a valid time zone"
    end

    s
  end

  def label!(s, min_len \\ 1, max_len \\ 63) when is_binary(s) and min_len > 0 and max_len > 0 do
    s = String.trim(s)
    l = String.length(s)

    unless l >= min_len and l <= max_len do
      raise ArgumentError, "not a valid label"
    end

    s
  end

  def url!(s) when is_binary(s) do
    s = String.trim(s)
    l = String.length(s)

    unless l >= 5 and l <= 127 and String.contains?(s, "://") do
      raise ArgumentError, "not a valid url"
    end

    s
  end

  def password!(s) when is_binary(s) do
    l = String.length(s)

    unless l >= 6 and l <= 32 do
      raise ArgumentError, "not a valid password"
    end

    s
  end

  def domain!(s) when is_binary(s) do
    s = String.trim(s)
    l = String.length(s)

    unless l >= 5 and l <= 63 and
             String.match?(
               s,
               ~r/^((http:\/\/)|(https:\/\/))?([a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,6}(\/)/
             ) do
      raise ArgumentError, "not a valid domain"
    end

    s
  end
end

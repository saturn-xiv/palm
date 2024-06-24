defmodule Tuberose.Validation do
  def email(s) when is_binary(s) do
    s = String.trim(s) |> String.downcase()
    l = String.length(s)

    if l >= 5 and l <= 127 and
         String.match?(s, ~r/^[a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$/) do
      {:ok, s}
    else
      {:error, :not_a_valid_email_address}
    end
  end

  def code(s, min_len \\ 1, max_len \\ 63) when is_binary(s) and min_len > 0 and max_len > 0 do
    s = String.trim(s) |> String.downcase()
    l = String.length(s)

    if l >= min_len and l <= max_len and String.match?(s, ~r/^[a-z][-._a-z0-9]+[a-z0-9]$/) do
      {:ok, s}
    else
      {:error, :not_a_valid_code}
    end
  end

  def language_code(s) when is_binary(s) do
    s = String.trim(s)
    l = String.length(s)

    if l >= 2 and l <= 15 and String.match?(s, ~r/^[a-z][-._a-zA-Z]+[a-zA-Z]$/) do
      {:ok, s}
    else
      {:error, :not_a_valid_language_code}
    end
  end

  def timezone(s) when is_binary(s) do
    s = String.trim(s)
    l = String.length(s)

    if l >= 2 and l <= 31 and String.match?(s, ~r/^[A-Z][\/a-zA-Z]+[a-zA-Z]$/) do
      {:ok, s}
    else
      {:error, :not_a_valid_timezone}
    end
  end

  def label(s, min_len \\ 1, max_len \\ 63) when is_binary(s) and min_len > 0 and max_len > 0 do
    s = String.trim(s)
    l = String.length(s)

    if l >= min_len and l <= max_len do
      {:ok, s}
    else
      {:error, :not_a_valid_label}
    end
  end

  def url(s) when is_binary(s) do
    s = String.trim(s)
    l = String.length(s)

    if l >= 5 and l <= 127 and String.contains?(s, "://") do
      {:ok, s}
    else
      {:error, :not_a_valid_url}
    end
  end

  def password(s) when is_binary(s) do
    l = String.length(s)

    if l >= 6 and l <= 32 do
      {:ok, s}
    else
      {:error, :not_a_valid_password}
    end
  end

  def domain_name(s) when is_binary(s) do
    s = String.trim(s) |> String.downcase()
    l = String.length(s)

    # https://www.oreilly.com/library/view/regular-expressions-cookbook/9781449327453/ch08s15.html
    if l >= 5 and l <= 63 and
         String.match?(s, ~r/^([a-z0-9]+(-[a-z0-9]+)*\.)+[a-z]{2,}$/) do
      {:ok, s}
    else
      {:error, :not_a_valid_domain_name}
    end
  end

  # ---------------------------------------------------------------------------

  def pagination(%{page: page, size: size}, total) when total >= 0 do
    size = if size >= 5 and size <= 120, do: size, else: 60

    page =
      cond do
        page <= 1 -> 1
        page * size <= total -> page
        rem(total, size) == 0 -> div(total, size)
        true -> div(total, size) + 1
      end

    %{page: page, size: size, total: total, has_previous: page > 1, has_next: total > page * size}
  end
end

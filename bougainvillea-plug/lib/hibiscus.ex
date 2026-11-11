defmodule Hibiscus do
  def uuid() do
    UUID.uuid4()
  end

  def ipv4 do
    {:ok, interfaces} = :inet.getifaddrs()

    interfaces
    |> Enum.flat_map(fn {_name, opts} -> Keyword.get_values(opts, :addr) end)
    |> Enum.find(fn
      {127, 0, 0, 1} -> false
      {_, _, _, _} = _ipv4 -> true
      _ -> false
    end)
    |> :inet.ntoa()
  end

  def public_ip do
    case Req.get("https://ipify.org") do
      {:ok, %{status: 200, body: ip}} -> {:ok, ip}
      _ -> {:error, "Unknown"}
    end
  end

  def page(index, size) do
    {(index - 1) * size, size}
  end

  def pagination(index, size, total) do
    total =
      case total do
        x when x <= 0 -> 0
        x -> x
      end

    size =
      case size do
        x when x <= 10 -> 10
        x when x >= 5000 -> 5000
        x -> x
      end

    pages =
      case rem(total, size) do
        0 -> div(total, size)
        _ -> div(total, size) + 1
      end

    index =
      case index do
        x when x <= 1 -> 1
        x when x >= pages -> pages
        x -> x
      end

    %{
      index: index,
      size: size,
      total: total,
      pages: pages,
      has_previous: index > 1,
      has_next: index < pages
    }
  end
end

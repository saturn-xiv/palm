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
end

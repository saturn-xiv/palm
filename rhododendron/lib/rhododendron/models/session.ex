defmodule Rhododendron.Session do
  def ipv4() do
    {:ok, hostname} = :inet.gethostname()
    {:ok, ip} = :inet.getaddr(hostname, :inet)
    ip |> :inet.ntoa() |> to_string()
  end
end

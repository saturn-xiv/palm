defmodule Rhododendron.CacheTest do
  use RhododendronWeb.ConnCase, async: true

  @doc """
  $ redis-cli -c -h 127.0.0.1 -p 6371
  > get testing://hi
  > ttl testing://hi
  """
  test "redis cluster" do
    {:ok, pong} = Rhododendron.Cache.ping()
    IO.puts("Ping: #{inspect(pong)}")

    {:ok, nodes} = Rhododendron.Cache.nodes()
    IO.puts("Cluster Nodes: #{nodes}")

    key = "hi"
    val = "Hello, Palm!"
    {:ok, _} = Rhododendron.Cache.set(key, val)
    tmp = Rhododendron.Cache.get(key)
    assert tmp == val
  end
end

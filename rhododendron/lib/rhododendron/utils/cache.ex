defmodule Rhododendron.Cache do
  # https://redix.hexdocs.pm/real-world-usage.html#name-based-pool
  def child_spec(_args) do
    %{pool_size: pool_size, url: url} = config()

    children =
      for i <- 1..pool_size do
        Supervisor.child_spec(
          {Redix.Cluster, nodes: [url], name: :"redis_#{i}"},
          id: {Redix.Cluster, i}
        )
      end

    %{
      id: RedixSupervisor,
      type: :supervisor,
      start: {Supervisor, :start_link, [children, [strategy: :one_for_one]]}
    }
  end

  def set(key, value, ttl \\ Duration.new!(day: 1)) do
    val = :erlang.term_to_binary(value)
    ttl = div(to_timeout(ttl), 1000)
    command(["SETEX", key(key), ttl, val])
  end

  def get(key) do
    {:ok, buf} = command(["GET", key(key)])
    :erlang.binary_to_term(buf, [:safe])
  end

  def nodes() do
    command(["CLUSTER", "NODES"])
  end

  def ping() do
    command(["PING"])
  end

  defp command(command) do
    %{pool_size: pool_size} = config()
    Redix.Cluster.command(:"redis_#{Enum.random(1..pool_size)}", command)
  end

  defp key(s) do
    %{namespace: namespace} = config()
    "#{namespace}://#{s}"
  end

  defp config() do
    Application.fetch_env!(:rhododendron, :redis)
  end
end

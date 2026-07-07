defmodule Rhododendron.Cache do
  # https://redix.hexdocs.pm/real-world-usage.html#name-based-pool
  def child_spec(_args) do
    children =
      for i <- 0..(pool_size() - 1) do
        Supervisor.child_spec(
          {Redix.Cluster, nodes: ["redis://127.0.0.1:6371"], name: :"redis_#{i}"},
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
    command(["SETEX", key, ttl, val])
  end

  def get(key) do
    {:ok, buf} = command(["GET", key])
    :erlang.binary_to_term(buf, [:safe])
  end

  def nodes() do
    command(["CLUSTER", "NODES"])
  end

  def ping() do
    command(["PING"])
  end

  defp command(command) do
    Redix.Cluster.command(:"redis_#{random_index()}", command)
  end

  defp random_index do
    Enum.random(0..(pool_size() - 1))
  end

  defp pool_size() do
    32
  end
end

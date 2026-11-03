defmodule Hibiscus do
  def version() do
    Node.start(:"elixir-node@127.0.0.1")
    Node.set_cookie(:"secret-cookie")

    target = {:status, :"java-node@127.0.0.1"}

    send(target, {self(), :version})

    receive do
      result -> result
    after
      5000 -> {:error, "Timeout waiting for Hibiscus node."}
    end
  end

  def uuid() do
    UUID.uuid4()
  end
end

defmodule Hibiscus do
  def version() do
    Node.start(:"elixir-node@127.0.0.1")
    Node.set_cookie(:"secret-cookie")

    target = {:"hibiscus-mailbox", :"java-node@127.0.0.1"}

    send(target, {self(), :version})

    receive do
      result -> result
    after
      5000 -> {:error, "Timeout waiting for Hibiscus node."}
    end
  end
end

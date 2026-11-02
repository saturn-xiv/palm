defmodule Hibiscus do
  def version() do
    Node.start(:"elixir_node@127.0.0.1")
    Node.set_cookie(:secret_cookie)

    target = {:hibiscus_mailbox, :"java_node@127.0.0.1"}

    send(target, {self(), :version})

    receive do
      result -> result
    after
      5000 -> {:error, "Timeout waiting for Hibiscus node."}
    end
  end
end

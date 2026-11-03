defmodule Hibiscus.Argon2 do
  def encode(password) do
    Node.start(:"elixir-node@127.0.0.1")
    Node.set_cookie(:"secret-cookie")

    target = {:argon2, :"java-node@127.0.0.1"}

    send(target, {self(), :encode, password})

    receive do
      result -> result
    after
      5000 -> {:error, "Timeout waiting for Hibiscus node."}
    end
  end

  def verify(password, hashed) do
    Node.start(:"elixir-node@127.0.0.1")
    Node.set_cookie(:"secret-cookie")

    target = {:argon2, :"java-node@127.0.0.1"}

    send(target, {self(), :verify, password, hashed})

    receive do
      result -> result
    after
      5000 -> {:error, "Timeout waiting for Hibiscus node."}
    end
  end
end

defmodule TuberoseWeb.Plugs.Token do
  import Plug.Conn

  def init(default), do: default

  def call(%Plug.Conn{req_cookies: %{"token" => v}} = conn, _default) when is_binary(v) do
    assign(conn, :token, v)
  end

  def call(%Plug.Conn{params: %{"token" => v}} = conn, _default) when is_binary(v) do
    assign(conn, :token, v)
  end

  def call(conn, _) do
    with ["Bearer " <> token] <- get_req_header(conn, "authorization") do
      assign(conn, :token, token)
    else
      _ -> assign(conn, :token, nil)
    end
  end
end

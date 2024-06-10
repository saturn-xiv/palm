defmodule TuberoseWeb.Plugs.ClientIP do
  import Plug.Conn

  def init(default), do: default

  # https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Forwarded
  def call(%Plug.Conn{req_headers: %{"x-real-ip" => v}} = conn, _default)
      when is_binary(v) do
    assign(conn, :client_ip, v)
  end

  def call(conn, default) do
    assign(conn, :client_ip, default)
  end
end

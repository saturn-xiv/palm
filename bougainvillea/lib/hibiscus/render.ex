defmodule Hibiscus.Render do
  import Plug.Conn

  # https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Type
  @text_html "text/html; charset=utf-8"
  @text_plain "text/plain; charset=utf-8"
  @application_json "application/json"

  def html(conn, status, body) do
    conn
    |> put_resp_content_type(@text_html)
    |> send_resp(status, body)
  end

  def json(conn, status, body) do
    conn
    |> put_resp_content_type(@application_json)
    |> send_resp(status, Jason.encode!(body))
  end

  def text(conn, status, body) do
    conn
    |> put_resp_content_type(@text_plain)
    |> send_resp(status, body)
  end
end

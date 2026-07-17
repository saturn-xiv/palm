defmodule RhododendronWeb.Context do
  @behaviour Plug

  import Plug.Conn

  def init(opts), do: opts

  def call(conn, _) do
    context = build_context(conn)
    Absinthe.Plug.put_options(conn, context: context)
  end

  defp build_context(conn) do
    %RhododendronWeb.Session{locale: locale(conn), token: token(conn), client_ip: client_ip(conn)}
  end

  defp client_ip(conn) do
    x_real_ip(conn) || x_forwarded_for(conn) || "n/a"
  end

  defp token(conn) do
    case Enum.find(get_req_header(conn, "Authorization"), fn v ->
           String.starts_with?(v, Rhododendron.Token.bearer())
         end) do
      nil ->
        nil

      auth ->
        String.trim_leading(auth, Rhododendron.Token.bearer())
    end
  end

  defp locale(conn) do
    key = "locale"

    lang =
      locale_from_query_params(conn, key) || locale_from_cookies(conn, key) ||
        locale_from_header(conn)

    if Enum.member?(Application.get_env(:rhododendron, :accept_languages), lang) do
      lang
    else
      Application.get_env(:rhododendron, :default_language)
    end
  end

  defp locale_from_query_params(conn, key) do
    conn = Plug.Conn.fetch_query_params(conn)
    conn.query_params[key]
  end

  defp locale_from_cookies(conn, key) do
    fetch_cookies(conn, signed: key)
  end

  # https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Accept-Language
  defp locale_from_header(conn) do
    case Enum.find(get_req_header(conn, "Accept-Language"), fn v ->
           v != "*"
         end) do
      nil ->
        nil

      val ->
        items = String.split(val, ",")
        items[0]
    end
  end

  defp x_real_ip(conn) do
    case get_req_header(conn, "X-Real-IP") do
      [val] -> val
      _ -> nil
    end
  end

  defp x_forwarded_for(conn) do
    case get_req_header(conn, "X-Forwarded-For") do
      [val] -> val
      _ -> nil
    end
  end
end

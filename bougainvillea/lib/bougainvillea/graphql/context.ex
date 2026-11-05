defmodule Bougainvillea.Graphql.Context do
  # https://absinthe.hexdocs.pm/context-and-authentication.html
  @behaviour Plug
  import Plug.Conn

  def init(opts), do: opts

  def call(conn, _) do
    context = %{token: detect_token(conn), client_ip: detect_client_ip(conn)}
    Absinthe.Plug.put_options(conn, context: context)
  end

  defp detect_client_ip(conn) do
    case get_req_header(conn, "x-forwarded-for") do
      [forwarded_for | _] ->
        forwarded_for
        |> String.split(",")
        |> List.first()
        |> String.trim()

      [] ->
        case get_req_header(conn, "x-real-ip") do
          [real_ip | _] -> String.trim(real_ip)
          [] -> conn.remote_ip |> :inet.ntoa() |> to_string()
        end
    end
  end

  defp detect_token(conn) do
    with ["Bearer " <> token] <- get_req_header(conn, "authorization") do
      token
    else
      _ -> ""
    end
  end
end

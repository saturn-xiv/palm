defmodule Bougainvillea.Graphql.Context do
  # https://absinthe.hexdocs.pm/context-and-authentication.html
  @behaviour Plug
  import Plug.Conn

  def init(opts), do: opts

  def call(conn, _) do
    context = %{token: detect_token(conn)}
    Absinthe.Plug.put_options(conn, context: context)
  end

  defp detect_token(conn) do
    with ["Bearer " <> token] <- get_req_header(conn, "authorization") do
      token
    else
      _ -> ""
    end
  end
end

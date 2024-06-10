defmodule TuberoseWeb.Context do
  require Logger

  def init(opts), do: opts

  def call(conn, _) do
    context = build_context(conn)
    Absinthe.Plug.put_options(conn, context: context)
  end

  @doc """
  Return the current user context based on the authorization header
  """
  def build_context(conn) do
    with {:ok, current_user} <-
           authorize(conn.assigns.token) do
      %{
        locale: conn.assigns.locale,
        client_ip: conn.assigns.client_ip,
        current_user: current_user
      }
    else
      _ ->
        %{
          locale: conn.assigns.locale,
          client_ip: conn.assigns.client_ip
        }
    end
  end

  defp authorize(token) when is_binary(token) do
    Logger.debug("parse user from #{token}")
    # TODO
    {:ok, "who-am-i"}
  end

  defp authorize(nil) do
    {:error, :invalid_authorization_token}
  end
end

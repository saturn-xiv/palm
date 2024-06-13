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

    case Tuberose.Atropa.Client.jwt_verify(token, to_string(:tuberose), "sign-in") do
      {:ok, subject, _} ->
        ss = Tuberose.Repo.get_by(Tuberose.UserSession, uid: subject)

        unless ss do
          raise ArgumentError, message: "Session isn't exists"
        end

        if ss.deleted_at do
          raise ArgumentError, message: "Session is disabled"
        end

        it = Tuberose.Repo.get(Tuberose.User, ss.user_id)

        unless it do
          raise ArgumentError, message: "User isn't exists"
        end

        if it.deleted_at do
          raise ArgumentError, message: "User is disabled"
        end

        if it.locked_at do
          raise ArgumentError, message: "User is locked"
        end

        {:ok,
         %{
           id: ss.user_id,
           session_id: ss.id,
           provider: %{type: String.to_atom(ss.provider_type), id: ss.provider_id}
         }}

      error ->
        error
    end
  end

  defp authorize(nil) do
    {:error, :invalid_authorization_token}
  end
end

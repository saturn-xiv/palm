defmodule TuberoseWeb.Api.UsersController do
  use TuberoseWeb, :controller

  require Logger

  def current(conn, _params) do
    # TODO
    json(conn, %{})
  end

  def logs(conn, _params) do
    # TODO
    json(conn, %{})
  end

  def sign_out(conn, _params) do
    # TODO
    user = conn.assign[:current_user]
    Logger.info("sign #{inspect(user)} out")
    json(conn, %{})
  end
end

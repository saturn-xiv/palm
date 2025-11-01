defmodule BambooWeb.UsersController do
  use BambooWeb, :controller

  def sign_in(conn, _params) do
    # TODO
    render(conn, :"sign-in")
  end

  def sign_out(conn, _params) do
    # TODO
    render(conn, :sign_in)
  end

  def profile(conn, _params) do
    # TODO
    render(conn, :profile)
  end

  def change_password(conn, _params) do
    # TODO
    render(conn, :"change-password")
  end

  def logs(conn, _params) do
    # TODO
    render(conn, :logs)
  end
end

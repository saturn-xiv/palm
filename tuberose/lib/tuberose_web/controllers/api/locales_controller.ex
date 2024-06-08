defmodule TuberoseWeb.Api.LocalesController do
  use TuberoseWeb, :controller

  def create(conn, _params) do
    # TODO
    json(conn, %{id: "in"})
  end

  def index(conn, _params) do
    # TODO
    json(conn, %{
      "sign-in": gettext("Sign in"),
      "sign-up": gettext("Sign up")
    })
  end

  def show(conn, _params) do
    # TODO
    json(conn, %{id: "out"})
  end

  def delete(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end
end

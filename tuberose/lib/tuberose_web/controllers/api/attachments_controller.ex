defmodule TuberoseWeb.Api.AttachmentsController do
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

  def associate(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def dissociate(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end
end

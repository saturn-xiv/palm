defmodule TuberoseWeb.Api.LeaveWordsController do
  use TuberoseWeb, :controller

  def create(conn, _params) do
    # TODO
    json(conn, %{id: "in"})
  end

  def index(conn, _params) do
    # TODO
    json(conn, %{id: "up"})
  end

  def show(conn, _params) do
    # TODO
    json(conn, %{id: "out"})
  end

  def delete(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def publish(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def revoke(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end
end

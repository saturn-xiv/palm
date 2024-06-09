defmodule TuberoseWeb.Api.LeaveWordsController do
  use TuberoseWeb, :controller

  def create(conn, %{
        "content" => content,
        "editor" => editor
      }) do
    %Tuberose.LeaveWord{
      lang: conn.assigns.locale,
      ip: conn.assigns.client_ip,
      body: content,
      editor: editor,
      status: "pending"
    }
    |> Tuberose.Repo.insert()

    json(conn, %{})
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

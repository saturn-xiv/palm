defmodule TuberoseWeb.Api.LocalesController do
  use TuberoseWeb, :controller

  require Logger
  import Ecto.Query

  def create(conn, _params) do
    # TODO
    json(conn, %{id: "in"})
  end

  def by_lang(conn, %{"lang" => lang}) do
    json(
      conn,
      from(p in Tuberose.Locale,
        where: [lang: ^lang],
        select: map(p, [:id, :code, :lang, :message, :updated_at]),
        order_by: [desc: :updated_at]
      )
      |> Tuberose.Repo.all()
    )
  end

  def index(conn, _params) do
    # TODO pager
    json(
      conn,
      from(p in Tuberose.Locale,
        select: map(p, [:id, :code, :lang, :message, :updated_at]),
        order_by: [desc: :updated_at]
      )
      |> Tuberose.Repo.all()
    )
  end

  def show(conn, %{"id" => id}) do
    json(
      conn,
      from(p in Tuberose.Locale,
        where: [id: ^id],
        select: map(p, [:id, :code, :lang, :message, :updated_at]),
        order_by: [desc: :updated_at]
      )
      |> first
      |> Tuberose.Repo.one!()
    )
  end

  def delete(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end
end

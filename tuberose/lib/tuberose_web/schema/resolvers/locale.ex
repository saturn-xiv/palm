defmodule TuberoseWeb.Resolvers.Locale do
  require Logger
  import Ecto.Query

  def show(_parent, %{id: id}, _resolution) do
    item =
      from(p in Tuberose.Locale,
        where: [id: ^id],
        select: map(p, [:id, :code, :lang, :message, :updated_at]),
        order_by: [desc: :updated_at]
      )
      |> first
      |> Tuberose.Repo.one!()

    {:ok, item}
  end

  def index(_parent, %{pager: pager}, _resolution) do
    total =
      Tuberose.Repo.one(
        from(p in Tuberose.Locale,
          select: count()
        )
      )

    pagination = Tuberose.Validation.pagination(pager, total)

    items =
      from(p in Tuberose.Locale,
        select: map(p, [:id, :code, :lang, :message, :updated_at]),
        order_by: [desc: :updated_at]
      )
      |> Tuberose.Repo.all()

    {:ok, %{items: items, pagination: pagination}}
  end

  def by_lang(_parent, %{lang: lang}, _resolution) do
    items =
      from(p in Tuberose.Locale,
        where: [lang: ^lang],
        select: map(p, [:id, :code, :lang, :message, :updated_at]),
        order_by: [desc: :updated_at]
      )
      |> Tuberose.Repo.all()

    {:ok, items}
  end
end

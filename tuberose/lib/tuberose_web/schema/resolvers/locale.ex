defmodule TuberoseWeb.Resolvers.Locale do
  require Logger
  import Ecto.Query
  import Ecto.Changeset

  def set(_parent, %{lang: lang, code: code, message: message}, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      item = Tuberose.Repo.get_by(Tuberose.Locale, lang: lang, code: code)

      if item do
        Tuberose.Repo.update(change(item, %{message: message, version: item.version + 1}))
      else
        %Tuberose.Locale{lang: lang, code: code, message: message} |> Tuberose.Repo.insert()
      end
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def destroy(_parent, %{id: id}, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      Tuberose.Repo.get!(Tuberose.Locale, id) |> Tuberose.Repo.delete()
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def show(_parent, %{id: id}, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

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

  def index(_parent, %{pager: pager}, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    total =
      Tuberose.Repo.one(
        from(p in Tuberose.Locale,
          select: count()
        )
      )

    pagination = Tuberose.Validation.pagination(pager, total)
    offset = (pagination.page - 1) * pagination.size

    items =
      from(p in Tuberose.Locale,
        select: map(p, [:id, :code, :lang, :message, :updated_at]),
        order_by: [desc: :updated_at],
        limit: ^pagination.size,
        offset: ^offset
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

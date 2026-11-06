defmodule Bougainvillea.Hydrangea.Dao.Job do
  import Ecto.Query

  def index(limit, offset) do
    query =
      from it in Bougainvillea.Hydrangea.Models.Job,
        limit: ^limit,
        offset: ^offset,
        order_by: [desc: it.created_at]

    Bougainvillea.Repo.all(query)
  end

  def count() do
    Bougainvillea.Hydrangea.Models.Job |> Bougainvillea.Repo.aggregate(:count)
  end
end

defmodule Jasmine.Models.Currency do
  require Logger
  require Ecto.Query

  def count() do
    Jasmine.Repo.one(Ecto.Query.from(p in Jasmine.Currency, select: count()))
  end
end

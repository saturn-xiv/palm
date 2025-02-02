defmodule Jasmine.Models.Locale do
  require Logger
  require Ecto.Query

  def languages() do
    Jasmine.Repo.all(Ecto.Query.from(p in Jasmine.Locale, distinct: p.lang, select: p.lang))
  end

  def count() do
    Jasmine.Repo.one(Ecto.Query.from(p in Jasmine.Locale, select: count()))
  end
end

defmodule Jasmine.Models.User do
  require Logger
  require Ecto.Query

  def create(lang, timezone) do
    uid = Ecto.UUID.generate()

    %Jasmine.User{
      uid: uid,
      lang: lang,
      timezone: timezone,
      updated_at: DateTime.utc_now()
    }
    |> Jasmine.Repo.insert()

    Jasmine.Repo.get_by(Jasmine.User, uid: uid)
  end

  def count() do
    Jasmine.Repo.one(Ecto.Query.from(p in Jasmine.User, select: count()))
  end
end

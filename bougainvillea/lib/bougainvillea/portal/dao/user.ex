defmodule Bougainvillea.Portal.Dao.User do
  def create(sn, name, lang, timezone) do
    %Bougainvillea.Portal.Models.User{
      sn: sn,
      name: name,
      lang: lang,
      timezone: timezone,
      updated_at: DateTime.utc_now(:microsecond)
    }
    |> Bougainvillea.Repo.insert()
  end
end

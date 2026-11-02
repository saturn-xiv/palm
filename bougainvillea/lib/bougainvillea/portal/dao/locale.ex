defmodule Bougainvillea.Portal.Dao.Locale do
  import Ecto.Query

  def exist?(lang, code) do
    Bougainvillea.Portal.Models.Locale |> where([x], x.lang == ^lang and x.code == ^code) |> Bougainvillea.Repo.aggregate(:count) == 0
  end

  def create(lang, code, message) do
    %Bougainvillea.Portal.Models.Locale{lang: lang, code: code, message: message, updated_at: DateTime.utc_now(:microsecond)} |> Bougainvillea.Repo.insert()
  end
end

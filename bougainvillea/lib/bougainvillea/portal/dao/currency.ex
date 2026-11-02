defmodule Bougainvillea.Portal.Dao.Currency do
  def count() do
    Bougainvillea.Portal.Models.Currency |> Bougainvillea.Repo.aggregate(:count)
  end

  def create(name, code, country, number, units, fund) do
    %Bougainvillea.Portal.Models.Currency{name: name, code: code, country: country, number: number, units: units, fund: fund} |> Bougainvillea.Repo.insert()
  end
end

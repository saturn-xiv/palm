defmodule Rhododendron.Dao.User do
  require Logger
  import Ecto.Changeset

  def set_location!(item, lang, timezone) do
    Logger.info("Set user #{item.uid} location.")

    Rhododendron.Repo.update!(
      change(item, %{version: item.version + 1, lang: lang, timezone: timezone})
    )
  end

  # def has?(user, role) do
  # TODO
  # unless Enum.any?(role.users, fn x -> x.id == user.id end) do
  # end
  # end

  defmodule Validators do
    def locale(s) do
      s = s |> String.trim()

      if Enum.member?(Application.get_env(:rhododendron, :accept_languages), s) do
        {:ok, s}
      else
        {:error, "#{s} is not a valid locale"}
      end
    end

    def timezone(s) do
      s = s |> String.trim()

      if Tzdata.zone_exists?(s) do
        {:ok, s}
      else
        {:error, "Invalid timezone #{s}"}
      end
    end
  end
end

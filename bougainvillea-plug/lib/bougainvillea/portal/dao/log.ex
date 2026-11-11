defmodule Bougainvillea.Portal.Dao.Log do
  def info(user_id, plugin, ip, message) do
    create(user_id, plugin, :info, ip, message)
  end

  defp create(user_id, plugin, level, ip, message) do
    %Bougainvillea.Portal.Models.Log{
      user_id: user_id,
      plugin: plugin |> to_string,
      level: level |> to_string,
      ip: ip |> to_string,
      message: message
    }
    |> Bougainvillea.Repo.insert()
  end
end

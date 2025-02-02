defmodule Jasmine.Models.Log do
  require Logger
  require Ecto.Query

  def error(user_id, plugin, ip, resource_type, resource_id, message) do
    create(user_id, plugin, :error, ip, resource_type, resource_id, message)
  end

  def warning(user_id, plugin, ip, resource_type, resource_id, message) do
    create(user_id, plugin, :warning, ip, resource_type, resource_id, message)
  end

  def info(user_id, plugin, ip, resource_type, resource_id, message) do
    create(user_id, plugin, :info, ip, resource_type, resource_id, message)
  end

  defp create(user_id, plugin, level, ip, resource_type, resource_id, message) do
    %Jasmine.Log{
      user_id: user_id,
      plugin: plugin,
      level: level,
      ip: ip,
      resource_type: resource_type,
      resource_id: resource_id,
      message: message
    }
    |> Jasmine.Repo.insert()
  end

  def count(user_id) do
    Jasmine.Repo.one(
      Ecto.Query.from(p in Jasmine.Log, where: p.user_id == ^user_id, select: count())
    )
  end
end

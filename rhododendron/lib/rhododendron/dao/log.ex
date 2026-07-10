defmodule Rhododendron.Dao.Log do
  import Ecto.Query

  def info!(user_id, plugin, ip, resource, message) do
    create!(user_id, plugin, :info, ip, message, resource)
  end

  def index(user_id, page, size) do
    Rhododendron.Repo.all(
      from(t in Rhododendron.Log,
        where: t.user_id == ^user_id,
        order_by: [desc: t.inserted_at],
        limit: ^page,
        offset: (^page - 1) * ^size
      )
    )
  end

  defp create!(user_id, plugin, level, ip, message, resource) do
    %Rhododendron.Log{
      user_id: user_id,
      plugin: to_string(plugin),
      level: to_string(level),
      ip: ip,
      resource: :erlang.term_to_binary(resource) |> Base.url_encode64(padding: false),
      message: message
    }
    |> Rhododendron.Repo.insert!()
  end
end

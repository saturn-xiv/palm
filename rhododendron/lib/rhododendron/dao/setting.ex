defmodule Rhododendron.Dao.Setting do
  import Ecto.Changeset

  def get(user_id, key) do
    item = Rhododendron.Repo.get_by!(Rhododendron.Setting, user_id: user_id, key: key)

    buf =
      if item.salt do
        %{tag: tag, iv: iv, add: additional} = :erlang.binary_to_term(item.salt, [:safe])
        Rhododendron.SecretBox.decrypt(item.value, tag, iv, additional)
      else
        item.value
      end

    :erlang.binary_to_term(buf, [:safe])
  end

  def get(key) do
    get(nil, key)
  end

  def set(user_id, key, value, encrypt) do
    value = :erlang.term_to_binary(value)

    if encrypt do
      additional = []
      {cipher, tag, iv} = Rhododendron.SecretBox.encrypt(value, 32, additional)
      set_(user_id, key, cipher, :erlang.term_to_binary(%{tag: tag, iv: iv, add: additional}))
    else
      set_(user_id, key, value, nil)
    end
  end

  def set(key, value, encrypt) do
    set(nil, key, value, encrypt)
  end

  defp set_(user_id, key, value, salt) do
    case Rhododendron.Repo.get_by(Rhododendron.Setting, user_id: user_id, key: key) do
      nil ->
        %Rhododendron.Setting{user_id: user_id, key: key, value: value, salt: salt}
        |> Rhododendron.Repo.insert()

      item ->
        Rhododendron.Repo.update!(
          change(item, %{version: item.version + 1, value: value, salt: salt})
        )
    end
  end
end

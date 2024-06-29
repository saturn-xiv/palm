defmodule Tuberose.KV do
  import Ecto.Query
  import Ecto.Changeset
  require Logger

  def set(key, value, encode) do
    value = :erlang.term_to_binary(value)

    {value, salt} =
      if encode do
        Tuberose.Atropa.Client.aes_encrypt(value)
      else
        {value, nil}
      end

    it =
      from(p in Tuberose.Setting,
        where: is_nil(p.user_id) and p.key == ^key
      )
      |> first
      |> Tuberose.Repo.one()

    case it do
      nil ->
        %Tuberose.Setting{key: key, value: value, salt: salt} |> Tuberose.Repo.insert()

      it ->
        Tuberose.Repo.update(change(it, %{value: value, salt: salt, version: it.version + 1}))
    end
  end

  def set(user, key, value, encode) do
    value = :erlang.term_to_binary(value)

    {value, salt} =
      if encode do
        Tuberose.Atropa.Client.aes_encrypt(value)
      else
        {value, nil}
      end

    it =
      from(p in Tuberose.Setting,
        where: p.user_id == ^user and p.key == ^key
      )
      |> first
      |> Tuberose.Repo.one()

    case it do
      nil ->
        %Tuberose.Setting{user_id: user, key: key, value: value, salt: salt}
        |> Tuberose.Repo.insert()

      it ->
        Tuberose.Repo.update(change(it, %{value: value, salt: salt, version: it.version + 1}))
    end
  end

  def get(key) do
    it =
      from(p in Tuberose.Setting,
        where: is_nil(p.user_id) and p.key == ^key,
        select: map(p, [:value, :salt])
      )
      |> first
      |> Tuberose.Repo.one()

    case it do
      nil ->
        nil

      it ->
        val =
          case it[:salt] do
            nil -> it[:value]
            salt -> Tuberose.Atropa.Client.aes_decrypt(it[:value], salt)
          end

        val |> :erlang.binary_to_term()
    end
  end

  def get(user, key) when user do
    it =
      from(p in Tuberose.Setting,
        where: [user_id: ^user, key: ^key],
        select: map(p, [:value, :salt])
      )
      |> first
      |> Tuberose.Repo.one()

    case it do
      nil ->
        nil

      it ->
        val =
          case it[:salt] do
            nil -> it[:value]
            salt -> Tuberose.Atropa.Client.aes_decrypt(it[:value], salt)
          end

        val |> :erlang.binary_to_term()
    end
  end
end

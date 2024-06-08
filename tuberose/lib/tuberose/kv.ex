defmodule Tuberose.Kv do
  import Ecto.Query

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

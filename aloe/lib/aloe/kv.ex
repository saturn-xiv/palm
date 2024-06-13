defmodule Aloe.KV do
  import Ecto.Changeset

  def get(key) do
    it = Aloe.Repo.get_by(Aloe.Setting, key: key)

    case it do
      nil ->
        nil

      it ->
        value = if it.encode, do: Aloe.Vault.decrypt!(it.value), else: it.value
        value |> :erlang.binary_to_term()
    end
  end

  def set(key, value, encode \\ false) do
    value = value |> :erlang.term_to_binary()
    value = if encode, do: Aloe.Vault.encrypt!(value), else: value

    it = Aloe.Repo.get_by(Aloe.Setting, key: key)

    case it do
      nil ->
        %Aloe.Setting{key: key, value: value, encode: encode} |> Aloe.Repo.insert()

      it ->
        it = change(it, %{value: value, encode: encode})
        Aloe.Repo.update(it)
    end
  end
end

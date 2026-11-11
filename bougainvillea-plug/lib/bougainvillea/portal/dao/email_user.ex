defmodule Bougainvillea.Portal.Dao.EmailUser do
  import Ecto.Query
  import Ecto.Changeset

  def all() do
    query = from it in Bougainvillea.Portal.Models.EmailUser, order_by: it.email
    Bougainvillea.Repo.all(query)
  end

  def set_password(id, password) do
    it = Bougainvillea.Repo.get(Bougainvillea.Portal.Models.EmailUser, id)
    now = DateTime.utc_now(:microsecond)

    change(it, %{version: it.version + 1, password: password, updated_at: now})
    |> Bougainvillea.Repo.update()
  end

  def confirm(id) do
    it = Bougainvillea.Repo.get(Bougainvillea.Portal.Models.EmailUser, id)
    now = DateTime.utc_now(:microsecond)

    change(it, %{version: it.version + 1, confirmed_at: now, updated_at: now})
    |> Bougainvillea.Repo.update()
  end

  def create(user_id, name, email, password) do
    %Bougainvillea.Portal.Models.EmailUser{
      sn: Hibiscus.uuid(),
      user_id: user_id,
      name: name,
      email: email,
      password: password,
      avatar: Hibiscus.Gravatar.image(email),
      updated_at: DateTime.utc_now(:microsecond)
    }
    |> Bougainvillea.Repo.insert()
  end

  def exist?(email) do
    Bougainvillea.Portal.Models.EmailUser
    |> where([x], x.email == ^email)
    |> Bougainvillea.Repo.aggregate(:count) > 0
  end
end

defmodule Bougainvillea.Task.User do
  require Logger

  def create_user_by_email(name, email, password) do
    {:ok, email} =
      Bougainvillea.Repo.transaction(fn ->
        {:ok, email_user} = Bougainvillea.Portal.Service.EmailUser.create(name, email, password)

        {:ok, _} =
          Bougainvillea.Portal.Dao.Log.info(
            email_user.user_id,
            :portal,
            Hibiscus.ipv4(),
            "Created by system administrator"
          )

        {:ok, _} = Bougainvillea.Portal.Dao.EmailUser.confirm(email_user.id)
        email_user.email
      end)

    Logger.info("done(#{email}).")
  end
end

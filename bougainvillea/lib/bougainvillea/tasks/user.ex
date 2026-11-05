defmodule Bougainvillea.Tasks.User do
  require Logger

  def list do
    IO.puts("SN                                   EMAIL USER")

    email_users = Bougainvillea.Portal.Dao.EmailUser.all()
    email_users = Bougainvillea.Repo.preload(email_users, :user)

    Enum.each(email_users, fn it ->
      IO.puts("#{it.user.sn} #{it.name}<#{it.email}>")
    end)
  end

  def set_password_for_email_user(email, password) do
    {:ok, email} =
      Bougainvillea.Repo.transaction(fn ->
        {:ok, it} = Bougainvillea.Portal.Services.EmailUser.set_password(email, password)

        {:ok, _} =
          Bougainvillea.Portal.Dao.Log.info(
            it.user_id,
            :portal,
            Hibiscus.ipv4(),
            "Reset password by system administrator"
          )

        it.email
      end)

    Logger.info("done(#{email}).")
  end

  def create_user_by_email(name, email, password) do
    {:ok, email} =
      Bougainvillea.Repo.transaction(fn ->
        {:ok, email_user} = Bougainvillea.Portal.Services.EmailUser.create(name, email, password)

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

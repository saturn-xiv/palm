defmodule Bougainvillea.Tasks.User do
  require Logger

  def add_role_for_user(user, role) do
    Logger.warning("Assigns role #{role} to user #{user}")
    {:ok, role} = Hibiscus.Validator.code(role)
    user = Bougainvillea.Repo.get_by(Bougainvillea.Portal.Models.User, sn: user)
    Dahlia.Rbac.Enforcer.add_role_for_user(user.id, role)
    {:ok, roles} = Dahlia.Rbac.Enforcer.get_implicit_roles_for_user(user.id)
    Logger.debug("current roles: #{roles}")
    Logger.info("done.")
  end

  def delete_role_for_user(user, role) do
    Logger.warning("Removes role #{role} from user #{user}")
    {:ok, role} = Hibiscus.Validator.code(role)
    user = Bougainvillea.Repo.get_by(Bougainvillea.Portal.Models.User, sn: user)
    Dahlia.Rbac.Enforcer.delete_role_for_user(user.id, role)
    {:ok, roles} = Dahlia.Rbac.Enforcer.get_implicit_roles_for_user(user.id)
    Logger.debug("current roles: #{roles}")
    Logger.info("done.")
  end

  def list do
    IO.puts("SN                                   EMAIL USER")

    email_users = Bougainvillea.Portal.Dao.EmailUser.all()
    email_users = Bougainvillea.Repo.preload(email_users, :user)

    Enum.each(email_users, fn it ->
      IO.puts("#{it.user.sn} #{it}>")
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

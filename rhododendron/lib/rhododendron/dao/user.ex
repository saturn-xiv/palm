defmodule Rhododendron.Dao.User do
  require Logger
  import Ecto.Changeset

  @audience_sign_in "user.sign-in"

  def set_location!(item, lang, timezone) do
    Logger.info("Set user #{item.uid} location.")

    Rhododendron.Repo.update!(
      change(item, %{version: item.version + 1, lang: lang, timezone: timezone})
    )
  end

  def build_sign_in_response(user, provider_type, provider_id, ttl \\ Duration.new!(day: 7)) do
    {:ok, token} =
      Rhododendron.Token.sign(
        :erlang.term_to_binary(%{
          type: provider_type,
          id: provider_id,
          action: @audience_sign_in
        })
        |> Base.url_encode64(padding: false),
        Rhododendron.Token.audience_by_web(),
        %{},
        ttl
      )

    %{
      token: token,
      user: user_layout(user),
      site: site_layout(user),
      created_at: DateTime.utc_now(:microsecond)
    }
  end

  def sign_in(user, provider_type, provider_id, client_ip) do
    if user.deleted_at do
      raise ArgumentError, "User #{user.name} is disabled."
    end

    if user.locked_at do
      raise ArgumentError, "User #{user.name} is locked."
    end

    Rhododendron.Repo.update!(
      change(user, %{
        version: user.version + 1,
        sign_in_count: user.sign_in_count + 1,
        last_sign_in_at: user.current_sign_in_at,
        last_sign_in_ip: user.current_sign_in_ip,
        current_sign_in_at: DateTime.utc_now(:microsecond),
        current_sign_in_ip: client_ip
      })
    )

    Rhododendron.Dao.Log.info!(
      user.id,
      :auth,
      client_ip,
      %{},
      "Sign in by (#{provider_type}, #{provider_id})."
    )
  end

  defp user_layout(user) do
    %{
      lang: user.lang,
      timezone: user.timezone,
      name: user.name,
      avatar: user.avatar,
      roles:
        Enum.map(Rhododendron.Dao.Role.get_implicit_roles_for_user(user), fn x -> x.code end),
      permissions:
        Enum.map(Rhododendron.Dao.Policy.get_implicit_permissions_for_user(user), fn x ->
          %{action: x.action, object: %{type: x.resource_type, id: x.resource_id}}
        end)
    }
  end

  defp site_layout(user) do
    %{
      title: Rhododendron.Dao.Site.title(user.lang),
      subhead: Rhododendron.Dao.Site.subhead(user.lang),
      description: Rhododendron.Dao.Site.description(user.lang),
      author: Rhododendron.Dao.Site.author(),
      keywords: Rhododendron.Dao.Site.keywords(),
      copyright: Rhododendron.Dao.Site.copyright(),
      available_languages: Application.get_env(:rhododendron, :accept_languages)
    }
  end

  defmodule Validators do
    def locale(s) do
      s = s |> String.trim()

      if Enum.member?(Application.get_env(:rhododendron, :accept_languages), s) do
        {:ok, s}
      else
        {:error, "#{s} is not a valid locale"}
      end
    end

    def timezone(s) do
      s = s |> String.trim()

      if Tzdata.zone_exists?(s) do
        {:ok, s}
      else
        {:error, "Invalid timezone #{s}"}
      end
    end
  end
end

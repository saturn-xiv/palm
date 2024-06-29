defmodule TuberoseWeb.Resolvers.Site do
  import Ecto.Query
  require Logger

  def delete_index_now_site_verification(_parent, _args, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      it =
        from(p in Tuberose.Setting,
          where: is_nil(p.user_id) and p.key == "index-now.site-verification"
        )
        |> first
        |> Tuberose.Repo.one()

      if it do
        Tuberose.Repo.delete(it)
      end
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def get_index_now_site_verification(_parent, _args, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    item = Tuberose.KV.get("index-now.site-verification")

    if item,
      do: {:ok, %{key: item.key}},
      else: {:ok, %{key: ""}}
  end

  def set_index_now_site_verification(_parent, %{key: key}, %{
        context: context
      }) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      Tuberose.KV.set("index-now.site-verification", %{key: key}, true)
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def delete_baidu_site_verification(_parent, _args, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      it =
        from(p in Tuberose.Setting,
          where: is_nil(p.user_id) and p.key == "baidu.site-verification"
        )
        |> first
        |> Tuberose.Repo.one()

      if it do
        Tuberose.Repo.delete(it)
      end
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def get_baidu_site_verification(_parent, _args, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    item = Tuberose.KV.get("baidu.site-verification")

    if item,
      do: {:ok, %{code: item.code, content: item.content}},
      else: {:ok, %{code: "", content: ""}}
  end

  def set_baidu_site_verification(_parent, %{code: code, content: content}, %{
        context: context
      }) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      Tuberose.KV.set("baidu.site-verification", %{code: code, content: content}, true)
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def delete_google_site_verification(_parent, _args, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      it =
        from(p in Tuberose.Setting,
          where: is_nil(p.user_id) and p.key == "google.site-verification"
        )
        |> first
        |> Tuberose.Repo.one()

      if it do
        Tuberose.Repo.delete(it)
      end
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def get_google_site_verification(_parent, _args, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    item = Tuberose.KV.get("google.site-verification")

    if item,
      do: {:ok, %{code: item.code}},
      else: {:ok, %{code: ""}}
  end

  def set_google_site_verification(_parent, %{code: code}, %{
        context: context
      }) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      Tuberose.KV.set("google.site-verification", %{code: code}, true)
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def delete_google_recaptcha(_parent, _args, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      it =
        from(p in Tuberose.Setting,
          where: is_nil(p.user_id) and p.key == "google.recaptcha"
        )
        |> first
        |> Tuberose.Repo.one()

      if it do
        Tuberose.Repo.delete(it)
      end
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def get_google_recaptcha(_parent, _args, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    item = Tuberose.KV.get("google.recaptcha")

    if item,
      do: {:ok, %{site_key: item.site_key, secret: item.secret}},
      else: {:ok, %{site_key: "", secret: ""}}
  end

  def set_google_recaptcha(_parent, %{site_key: site_key, secret: secret}, %{context: context}) do
    unless Tuberose.Atropa.Client.administrator?(context.current_user.id) do
      raise ArgumentError, message: "Forbidden"
    end

    Tuberose.Repo.transaction(fn ->
      Tuberose.KV.set("google.recaptcha", %{site_key: site_key, secret: secret}, true)
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def routes(_parent, _args, %{context: context}) do
    locale = context.locale
    items = []

    personal =
      if context.current_user do
        [
          %{
            path: "/dashboard/personal",
            name: Tuberose.I18N.t(locale, "personal.index.title"),
            children: [
              %{
                path: "/dashboard/personal/logs",
                name: Tuberose.I18N.t(locale, "personal.logs.title"),
                children: []
              },
              %{
                path: "/dashboard/personal/profile",
                name: Tuberose.I18N.t(locale, "personal.profile.title"),
                children: []
              },
              %{
                path: "/dashboard/personal/attachments",
                name: Tuberose.I18N.t(locale, "personal.attachments.title"),
                children: []
              }
            ]
          }
        ]
      else
        []
      end

    settings =
      if context.current_user != nil and
           Tuberose.Atropa.Client.administrator?(context.current_user.id) do
        [
          %{
            path: "/dashboard/settings",
            name: Tuberose.I18N.t(locale, "settings.index.title"),
            children: [
              %{
                path: "/dashboard/settings/site/status",
                name: Tuberose.I18N.t(locale, "settings.site.status.title"),
                children: []
              },
              %{
                path: "/dashboard/settings/site/info",
                name: Tuberose.I18N.t(locale, "settings.site.info.title"),
                children: []
              },
              %{
                path: "/dashboard/settings/site/seo",
                name: Tuberose.I18N.t(locale, "settings.site.seo.title"),
                children: []
              },
              %{
                path: "/dashboard/settings/locales",
                name: Tuberose.I18N.t(locale, "settings.locales.index.title"),
                children: []
              }
            ]
          }
        ]
      else
        []
      end

    {:ok, items ++ personal ++ settings}
  end

  def layout(_parent, _args, %{context: context}) do
    {:ok,
     %{
       favicon: Tuberose.KV.get("site.author") || "/my/favicon.svg",
       title: Tuberose.I18N.t(context.locale, "site.title"),
       subhead: Tuberose.I18N.t(context.locale, "site.subhead"),
       description: Tuberose.I18N.t(context.locale, "site.description"),
       copyright: Tuberose.I18N.t(context.locale, "site.copyright"),
       author: Tuberose.KV.get("site.author"),
       keywords: Tuberose.KV.get("site.keywords") || [],
       icp: Tuberose.KV.get("site.icp"),
       gab: Tuberose.KV.get("site.gab"),
       locale: context.locale,
       languages: Gettext.known_locales(TuberoseWeb.Gettext)
     }}
  end
end

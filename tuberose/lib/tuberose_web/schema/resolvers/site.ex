defmodule TuberoseWeb.Resolvers.Site do
  require Logger

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

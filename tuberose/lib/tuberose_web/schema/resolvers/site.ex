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
          },
          %{
            path: "aaa.2",
            name: "bbb.2",
            children: [
              %{path: "aaa1", name: "bbb1", children: []},
              %{path: "aaa2", name: "bbb2", children: []}
            ]
          }
        ]
      else
        []
      end

    {:ok, items ++ personal}
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

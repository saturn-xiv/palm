defmodule TuberoseWeb.Resolvers.Site do
  require Logger

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

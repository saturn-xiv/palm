defmodule TuberoseWeb.Api.HomeController do
  use TuberoseWeb, :controller

  require Logger

  def layout(conn, _params) do
    locale = conn.assigns[:locale]

    json(conn, %{
      favicon: Tuberose.I18n.t(locale, "site.favicon"),
      title: Tuberose.I18n.t(locale, "site.title"),
      subhead: Tuberose.I18n.t(locale, "site.subhead"),
      description: Tuberose.I18n.t(locale, "site.description"),
      copyright: Tuberose.I18n.t(locale, "site.copyright"),
      author: Tuberose.Kv.get("site.author"),
      keywords: Tuberose.Kv.get("site.keywords") || [],
      icp: Tuberose.Kv.get("site.icp"),
      gab: Tuberose.Kv.get("site.gab"),
      locale: locale,
      languages: Gettext.known_locales(TuberoseWeb.Gettext),
      version: "v#{Application.spec(:tuberose)[:vsn]}"
    })
  end
end

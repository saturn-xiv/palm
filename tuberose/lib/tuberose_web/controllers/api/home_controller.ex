defmodule TuberoseWeb.Api.HomeController do
  use TuberoseWeb, :controller

  require Logger

  def layout(conn, _params) do
    locale = conn.assigns[:locale]

    json(conn, %{
      favicon: Tuberose.I18N.t(locale, "site.favicon"),
      title: Tuberose.I18N.t(locale, "site.title"),
      subhead: Tuberose.I18N.t(locale, "site.subhead"),
      description: Tuberose.I18N.t(locale, "site.description"),
      copyright: Tuberose.I18N.t(locale, "site.copyright"),
      author: Tuberose.KV.get("site.author"),
      keywords: Tuberose.KV.get("site.keywords") || [],
      icp: Tuberose.KV.get("site.icp"),
      gab: Tuberose.KV.get("site.gab"),
      locale: locale,
      languages: Gettext.known_locales(TuberoseWeb.Gettext),
      version: "v#{Application.spec(:tuberose)[:vsn]}"
    })
  end
end

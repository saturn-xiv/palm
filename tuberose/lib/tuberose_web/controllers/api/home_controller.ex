defmodule TuberoseWeb.Api.HomeController do
  use TuberoseWeb, :controller

  def layout(conn, _params) do
    # TODO
    json(conn, %{
      locale: Gettext.get_locale(TuberoseWeb.Gettext),
      locales: Gettext.known_locales(TuberoseWeb.Gettext)
    })
  end
end

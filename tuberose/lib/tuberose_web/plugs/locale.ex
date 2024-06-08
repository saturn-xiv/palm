defmodule TuberoseWeb.Plugs.Locale do
  import Plug.Conn

  @locales ["en-US", "zh-Hans", "zh-Hant"]

  def init(default), do: default

  def call(%Plug.Conn{req_cookies: %{"locale" => loc}} = conn, _default) when loc in @locales do
    Gettext.put_locale(loc)
    assign(conn, :locale, loc)
  end

  def call(%Plug.Conn{params: %{"locale" => loc}} = conn, _default) when loc in @locales do
    Gettext.put_locale(loc)
    assign(conn, :locale, loc)
  end

  def call(conn, default) do
    Gettext.put_locale(default)
    assign(conn, :locale, default)
  end
end

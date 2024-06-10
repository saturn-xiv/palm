defmodule TuberoseWeb.Plugs.Locale do
  import Plug.Conn

  @locales ["en-US", "zh-Hans", "zh-Hant"]

  def init(default), do: default

  def call(%Plug.Conn{req_cookies: %{"locale" => v}} = conn, _default) when v in @locales do
    Gettext.put_locale(v)
    assign(conn, :locale, v)
  end

  def call(%Plug.Conn{params: %{"locale" => v}} = conn, _default) when v in @locales do
    Gettext.put_locale(v)
    assign(conn, :locale, v)
  end

  def call(conn, default) do
    Gettext.put_locale(default)
    assign(conn, :locale, default)
  end
end

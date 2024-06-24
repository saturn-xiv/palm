defmodule TuberoseWeb.PageController do
  require Logger
  use TuberoseWeb, :controller

  def home(conn, _params) do
    render(conn, :home, layout: false)
  end

  def home_by_lang(conn, %{"lang" => lang}) do
    Logger.debug("home by #{lang}")
    render(conn, :home, layout: false)
  end
end

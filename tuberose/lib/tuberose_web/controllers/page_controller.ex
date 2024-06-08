defmodule TuberoseWeb.PageController do
  use TuberoseWeb, :controller

  def home(conn, _params) do
    render(conn, :home, layout: false)
  end
end

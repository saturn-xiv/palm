defmodule TuberoseWeb.Api.UsersController do
  use TuberoseWeb, :controller

  def sign_in(conn, _params) do
    # TODO
    json(conn, %{id: "in"})
  end

  def sign_up(conn, _params) do
    # TODO
    json(conn, %{id: "up"})
  end

  def sign_out(conn, _params) do
    # TODO
    json(conn, %{id: "out"})
  end

  def confirm_by_email(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def confirm_by_token(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def unlock_by_email(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def unlock_by_token(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def forgot_password(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def reset_password(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def get_profile(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def set_profile(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def change_password(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def current(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end
end

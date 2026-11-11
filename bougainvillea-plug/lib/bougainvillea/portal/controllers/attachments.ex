defmodule Bougainvillea.Portal.Controllers.Attachments do
  def show(_conn, _sn) do
  end

  def create(_conn) do
    # upload = conn.body_params["image"]

    # - upload.path
    # - upload.filename
    # - upload.content_type
    # TODO detect validate magic bytes of file
    {:ok, %{}}
  end
end

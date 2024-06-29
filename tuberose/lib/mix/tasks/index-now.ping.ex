defmodule Mix.Tasks.Tuberose.IndexNow.Ping do
  require Logger

  use Mix.Task

  @shortdoc "Submit urls to IndexNow"

  @moduledoc """
  Usage:
  mix tuberose.index_now.ping --domain www.change-me.org
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    {parsed, _} =
      OptionParser.parse!(args, strict: [domain: :string])

    unless parsed[:domain] do
      raise ArgumentError, message: "empty domain"
    end

    {:ok, domain} = Tuberose.Validation.domain_name(parsed[:domain])
    home = "https://#{domain}"

    # TODO
    Logger.info("Ping #{home}")

    Enum.map(Gettext.known_locales(TuberoseWeb.Gettext), fn lang ->
      # TODO
      Logger.info("Ping #{home}/#{lang}/")
    end)
  end
end

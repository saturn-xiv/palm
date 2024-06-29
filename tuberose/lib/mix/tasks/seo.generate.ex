defmodule Mix.Tasks.Tuberose.Seo.Generate do
  require Logger

  use Mix.Task

  @shortdoc "Generate robots.txt/sitemap.xml/rss.xml"

  @moduledoc """
  Usage:
  mix tuberose.seo.generate --domain www.change-me.org
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
    Logger.info("Generate #{home}/robots.txt")
    # TODO
    Logger.info("Generate #{home}/sitemap.xml")

    Enum.map(Gettext.known_locales(TuberoseWeb.Gettext), fn lang ->
      # TODO
      Logger.info("Generate #{home}/sitemap/#{lang}.xml")
      # TODO
      Logger.info("Generate #{home}/rss/#{lang}.xml")
    end)

    # TODO
    Logger.info("Generate Baidu site-verification file")
    # TODO
    # https://support.google.com/webmasters/answer/9008080?hl=en
    Logger.info("Generate Google site-verification file")
    # TODO
    # https://www.indexnow.org/documentation
    Logger.info("Generate IndexNow site-verification file")
  end
end

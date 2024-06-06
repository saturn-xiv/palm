defmodule Mix.Tasks.Tuberose.Token.Generate do
  require Logger

  use Mix.Task

  @shortdoc "Generate a jwt token for third-party endpoints"

  @moduledoc """
  Usage:
  mix tuberose.token.generate --subject sss --issuer iii --audience a1 --audience a2 --years 20
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    {parsed, _} =
      OptionParser.parse!(args,
        strict: [issuer: :string, subject: :string, audience: [:string, :keep], years: :integer]
      )

    unless parsed[:issuer] do
      raise ArgumentError, message: "empty issuer"
    end

    unless parsed[:subject] do
      raise ArgumentError, message: "empty subject"
    end

    unless parsed[:audience] do
      raise ArgumentError, message: "empty audiences"
    end

    unless parsed[:years] && parsed[:years] > 0 do
      raise ArgumentError, message: "bad years"
    end

    audiences = Keyword.get_values(parsed, :audience)

    Logger.info(
      "Generate token for issuer(#{parsed[:issuer]}) audiences(#{Enum.join(audiences, ",")}) for years(#{parsed[:years]})"
    )

    not_before = %Google.Protobuf.Timestamp{
      seconds: (DateTime.utc_now() |> DateTime.to_unix()) - 1
    }

    expires_at = %Google.Protobuf.Timestamp{
      seconds: DateTime.utc_now() |> Timex.shift(years: parsed[:years]) |> DateTime.to_unix()
    }

    token =
      Tuberose.Atropa.Client.jwt_sign(
        parsed[:issuer],
        parsed[:subject],
        audiences,
        not_before,
        expires_at
      )

    IO.puts("#{token}")
  end
end

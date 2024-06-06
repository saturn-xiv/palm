defmodule Mix.Tasks.Token.Generate do
  use Mix.Task

  @shortdoc "Generate a jwt token"

  @moduledoc """
  Generate a jwt token for third-party endpoints.
  Usage:
  mix token.generate --subject sss --issuer iii --audience a1 --audience a2 --years 20
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

    Mix.shell().info(
      "Generate token for issuer(#{parsed[:issuer]}) audiences(#{Enum.join(audiences, ",")}) for years(#{parsed[:years]})"
    )

    {:ok, channel} = GRPC.Stub.connect(Application.get_env(:tuberose, Tuberose.Atropa)[:host])

    not_before = %Google.Protobuf.Timestamp{
      seconds: (DateTime.utc_now() |> DateTime.to_unix()) - 1
    }

    expires_at = %Google.Protobuf.Timestamp{
      seconds: DateTime.utc_now() |> Timex.shift(years: parsed[:years]) |> DateTime.to_unix()
    }

    request =
      %Palm.Atropa.V1.JwtSignRequest{
        issuer: parsed[:issuer],
        subject: parsed[:subject],
        audiences: audiences,
        not_before: not_before,
        expires_at: expires_at
      }

    {:ok, reply} = channel |> Palm.Atropa.V1.Jwt.Stub.sign(request)
    Mix.shell().info("#{reply.token}")
  end
end

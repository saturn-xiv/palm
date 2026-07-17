defmodule Rhododendron.Token do
  use Joken.Config

  @web "Web"
  def audience_by_web, do: @web
  @app "App"
  def audience_by_app, do: @app
  @mailer "Mailer"
  def audience_by_mailer, do: @mailer

  @impl true
  def token_config do
    default_claims(
      iss: Application.get_application(__MODULE__) |> Atom.to_string() |> String.capitalize(),
      skip: [:aud]
    )
    |> add_claim("aud", nil, &(&1 in [@web, @app, @mailer]))
  end

  def sign(
        subject,
        audience,
        claims \\ %{},
        ttl \\ Duration.new!(day: 1)
      ) do
    {:ok, token, _} =
      generate_and_sign(
        Map.merge(claims, %{
          "sub" => subject,
          "aud" => audience,
          "exp" => DateTime.utc_now() |> DateTime.shift(ttl) |> DateTime.to_unix()
        })
      )

    {:ok, token}
  end

  def parse(token) do
    {:ok, claims} = verify_and_validate(token)

    subject = claims["sub"]
    claims = Map.drop(claims, ["jti", "iss", "sub", "aud", "iat", "nbf", "exp"])
    {:ok, subject, claims}
  end

  def bearer() do
    "Bearer "
  end
end

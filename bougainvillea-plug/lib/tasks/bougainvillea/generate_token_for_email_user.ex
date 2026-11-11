defmodule Mix.Tasks.Bougainvillea.GenerateTokenForEmailUser do
  use Mix.Task

  @shortdoc "Generate token for email user(-e/--email,-a/--audience,-w/-weeks)"

  @impl true
  def run(args) do
    Mix.Task.run("app.start")

    {parsed_options, _remaining_args, invalid_options} =
      OptionParser.parse(args,
        strict: [email: :string, audience: :keep, weeks: :integer],
        aliases: [e: :email, a: :audience, w: :weeks]
      )

    if invalid_options != [] do
      Mix.shell().error("Invalid options provided: #{inspect(invalid_options)}")
    end

    email = Keyword.get(parsed_options, :email)
    weeks = Keyword.get(parsed_options, :weeks)
    audiences = Keyword.get_values(parsed_options, :audience)

    Bougainvillea.Tasks.Token.generate_for_email_user(email, audiences, "P#{weeks}W")
  end
end

defmodule Bougainvillea.Graphql.Schema do
  use Absinthe.Schema

  import_types(Bougainvillea.Portal.Graphql.ContentTypes)
  import_types(Bougainvillea.Hydrangea.Graphql.ContentTypes)
  alias Bougainvillea.Portal.Graphql.Resolvers, as: PortalResolvers
  alias Bougainvillea.Hydrangea.Graphql.Resolvers, as: HydrangeaResolvers

  query do
    # ---------------------------------------------------------------------------
    @desc "Show version"
    field :api_version, non_null(:string) do
      resolve(&PortalResolvers.version/3)
    end

    # ---------------------------------------------------------------------------
    @desc "Fetch background jobs"
    field :hydrangea_index_job, type: non_null(:hydrangea_index_job_response) do
      arg(:name, non_null(:page))

      resolve(&HydrangeaResolvers.Job.index/3)
    end

    # ---------------------------------------------------------------------------
  end

  mutation do
    # ---------------------------------------------------------------------------
    @desc "Sign in by email"
    field :sign_in_by_email, type: non_null(:user_sign_in_response) do
      arg(:email, non_null(:string))
      arg(:password, non_null(:string))

      resolve(&PortalResolvers.EmailUser.sign_in/3)
    end

    @desc "Sign up by email"
    field :sign_up_by_email, type: non_null(:succeed) do
      arg(:form, non_null(:sign_up_by_email_request))

      resolve(&PortalResolvers.EmailUser.sign_up/3)
    end

    # ---------------------------------------------------------------------------

    @desc "Execute a background job"
    field :hydrangea_execute_background_job, type: non_null(:succeed) do
      arg(:name, non_null(:string))
      arg(:args, non_null(list_of(non_null(:string))))

      resolve(&HydrangeaResolvers.Job.execute/3)
    end

    # ---------------------------------------------------------------------------
  end
end

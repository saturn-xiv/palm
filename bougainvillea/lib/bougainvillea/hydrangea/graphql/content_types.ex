defmodule Bougainvillea.Hydrangea.Graphql.ContentTypes do
  use Absinthe.Schema.Notation

  object :hydrangea_index_job_response do
    field :pagination, non_null(:pagination)
    field :items, non_null(list_of(non_null(:hydrangea_job)))
  end

  object :hydrangea_job do
    field :id, non_null(:integer)
    field :sn, non_null(:string)
    field :name, non_null(:string)
    field :summary, non_null(:string)
    field :version, non_null(:string)
    field :args, non_null(list_of(non_null(:string)))
    field :script, non_null(:string)
    field :launched_at, non_null(:datetime)
    field :elapsed, non_null(:string)
    field :status_code, non_null(:integer)
    field :stdout, non_null(:string)
    field :stderr, non_null(:string)
    field :created_at, non_null(:datetime)
  end
end

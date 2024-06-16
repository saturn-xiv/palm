defmodule TuberoseWeb.Resolvers.Attachment do
  require Logger
  import Ecto.Query
  import Ecto.Changeset

  def attach(_parent, %{id: id, resource_type: resource_type, resource_id: resource_id}, %{
        context: context
      }) do
    item = Tuberose.Repo.get(Tuberose.Attachment, id)

    unless item.user_id == context.current_user.id do
      raise ArgumentError, message: "Forbidden"
    end

    unless item.uploaded_at do
      raise ArgumentError, message: "Bad request"
    end

    if item.deleted_at do
      raise ArgumentError, message: "Not found"
    end

    Tuberose.Repo.transaction(fn ->
      %Tuberose.AttachmentResource{
        attachment_id: id,
        resource_type: resource_type,
        resource_id: resource_id
      }
      |> Tuberose.Repo.insert()
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def detach(_parent, %{id: id, resource_type: resource_type, resource_id: resource_id}, %{
        context: context
      }) do
    item = Tuberose.Repo.get(Tuberose.Attachment, id)

    unless item.user_id == context.current_user.id do
      raise ArgumentError, message: "Forbidden"
    end

    unless item.uploaded_at do
      raise ArgumentError, message: "Bad request"
    end

    if item.deleted_at do
      raise ArgumentError, message: "Not found"
    end

    Tuberose.Repo.transaction(fn ->
      Tuberose.Repo.get_by(Tuberose.AttachmentResource,
        attachment_id: id,
        resource_type: resource_type,
        resource_id: resource_id
      )
      |> Tuberose.Repo.delete()
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def set_uploaded(_parent, %{id: id, succeed: succeed}, %{
        context: context
      }) do
    item = Tuberose.Repo.get(Tuberose.Attachment, id)

    unless item.user_id == context.current_user.id do
      raise ArgumentError, message: "Forbidden"
    end

    if item.uploaded_at do
      raise ArgumentError, message: "Bad request"
    end

    Tuberose.Repo.transaction(fn ->
      if succeed do
        Tuberose.Repo.update(
          change(item, %{uploaded_at: DateTime.utc_now(), version: item.version + 1})
        )
      else
        Tuberose.Repo.delete(item)
      end
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def upload_url(
        _parent,
        %{title: title, size: size, content_type: content_type, ttl: ttl},
        %{
          context: context
        }
      ) do
    unless context.current_user do
      raise ArgumentError, message: "Forbidden"
    end

    bucket = Tuberose.Atropa.Client.s3_create_bucket(to_string(:tuberose), false, 0)
    {object, url} = Tuberose.Atropa.Client.s3_upload(bucket, title, ttl)

    Tuberose.Repo.transaction(fn ->
      %Tuberose.Attachment{
        user_id: context.current_user.id,
        title: title,
        bucket: bucket,
        object: object,
        content_type: content_type,
        size: size
      }
      |> Tuberose.Repo.insert()
    end)

    item = Tuberose.Repo.get_by(Tuberose.Attachment, bucket: bucket, object: object)
    {:ok, %{id: item.id, bucket: bucket, object: object, url: url}}
  end

  def update(_parent, %{id: id, title: title}, %{context: context}) do
    item = Tuberose.Repo.get(Tuberose.Attachment, id)

    unless item.user_id == context.current_user.id do
      raise ArgumentError, message: "Forbidden"
    end

    if item.deleted_at do
      raise ArgumentError, message: "Bad request"
    end

    Tuberose.Repo.transaction(fn ->
      it = Tuberose.Repo.get(Tuberose.Attachment, id)
      Tuberose.Repo.update(change(it, %{title: title, version: it.version + 1}))
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def destroy(_parent, %{id: id}, %{context: context}) do
    item = Tuberose.Repo.get(Tuberose.Attachment, id)

    unless item.user_id == context.current_user.id do
      raise ArgumentError, message: "Forbidden"
    end

    total =
      Tuberose.Repo.one(
        from(p in Tuberose.AttachmentResource,
          where: p.attachment_id == ^id,
          select: count()
        )
      )

    unless total == 0 do
      raise ArgumentError, message: "Attachment is in used"
    end

    Tuberose.Repo.transaction(fn ->
      it = Tuberose.Repo.get(Tuberose.Attachment, id)

      Tuberose.Repo.update(change(it, %{deleted_at: DateTime.utc_now()}))
    end)

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def show(_parent, %{id: id, ttl: ttl}, %{context: context}) do
    item = Tuberose.Repo.get(Tuberose.Attachment, id)

    unless item.user_id == context.current_user.id do
      raise ArgumentError, message: "Forbidden"
    end

    content_type =
      if Tuberose.Atropa.Client.s3_inline?(item.content_type), do: item.content_type, else: nil

    url =
      if item.deleted_at,
        do: nil,
        else:
          Tuberose.Atropa.Client.s3_presigned_url(
            item.bucket,
            item.object,
            item.title,
            content_type,
            ttl
          )

    {:ok,
     %{
       url: url,
       item: %{
         id: item.id,
         title: item.title,
         bucket: item.bucket,
         object: item.object,
         size: item.size,
         content_type: item.content_type,
         uploaded_at: item.uploaded_at,
         deleted_at: item.deleted_at,
         updated_at: item.updated_at
       }
     }}
  end

  def index(_parent, %{pager: pager}, %{context: context}) do
    total =
      Tuberose.Repo.one(
        from(p in Tuberose.Attachment,
          where: p.user_id == ^context.current_user.id and is_nil(p.deleted_at),
          select: count()
        )
      )

    pagination = Tuberose.Validation.pagination(pager, total)
    offset = (pagination.page - 1) * pagination.size

    items =
      from(p in Tuberose.Attachment,
        where: p.user_id == ^context.current_user.id and is_nil(p.deleted_at),
        select:
          map(p, [
            :id,
            :title,
            :bucket,
            :object,
            :size,
            :content_type,
            :uploaded_at,
            :deleted_at,
            :updated_at
          ]),
        order_by: [desc: :updated_at],
        limit: ^pagination.size,
        offset: ^offset
      )
      |> Tuberose.Repo.all()

    {:ok, %{items: items, pagination: pagination}}
  end

  def by_image(_parent, _args, %{context: context}) do
    items =
      from(p in Tuberose.Attachment,
        where: p.user_id == ^context.current_user.id,
        select:
          map(p, [
            :id,
            :title,
            :bucket,
            :name,
            :size,
            :content_type,
            :uploaded_at,
            :deleted_at,
            :updated_at
          ]),
        order_by: [desc: :updated_at]
      )
      |> Tuberose.Repo.all()

    {:ok, items}
  end
end

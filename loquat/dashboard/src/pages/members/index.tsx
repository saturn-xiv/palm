import { useEffect, useCallback, useState } from "react";
import { FormattedMessage } from "react-intl";

import Timestamp from "../../components/Timestamp";
import { useAppDispatch } from "../../hooks";
import { danger as show_danger } from "../../reducers/notification";
import { index as index_member, type IMember } from "../../api/members";

export const INDEX = "/dashboard/members";

const Widget = () => {
  const dispatch = useAppDispatch();
  const [items, setItems] = useState<IMember[]>([]);
  const onSelect = useCallback(async () => {
    const res = await index_member();
    if (res.data?.indexMember) {
      setItems(res.data.indexMember);
    } else if (res.errors) {
      dispatch(show_danger(res.errors));
    }
  }, [dispatch]);
  useEffect(() => {
    (async () => {
      await onSelect();
    })();
  }, [onSelect]);
  return (
    <>
      <div className="is-size-2">
        <FormattedMessage id="pages.members.index.title" />
      </div>
      <table className="table is-hoverable is-fullwidth">
        <thead>
          <tr>
            <th>
              <FormattedMessage id="tables.column.label.id" />
            </th>
            <th>
              <FormattedMessage id="tables.column.label.sn" />
            </th>
            <th>
              <FormattedMessage id="tables.column.label.name" />
            </th>
            <th>
              <FormattedMessage id="tables.column.label.memo" />
            </th>
            <th>
              <FormattedMessage id="tables.column.label.updated-at" />
            </th>
          </tr>
        </thead>
        <tbody>
          {items.map((it, id) => (
            <tr key={id}>
              <th>{it.id}</th>
              <td>{it.sn}</td>
              <td>{it.name}</td>
              <td>{it.memo}</td>
              <td>
                <Timestamp value={it.updatedAt} />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
};

export default Widget;

import { useEffect, useState } from "react";
import { FormattedMessage } from "react-intl";

import { type ILogsResponse, get_logs } from "../../api/members";
import PaginationBar from "../../components/PaginationBar";
import Timestamp from "../../components/Timestamp";
import type { IPage } from "../../api";

const Widget = () => {
  const [item, setItem] = useState<ILogsResponse>({
    items: [],
    pagination: { total: 0, index: 1, size: 12 },
  });
  useEffect(() => {
    (async () => {
      const tmp = await get_logs({ index: 1, size: 12 });
      setItem(tmp);
    })();
  }, []);
  return (
    <>
      <div className="is-size-2">
        <FormattedMessage id="pages.users.logs.title" />
      </div>
      <table className="table is-hoverable is-fullwidth">
        <thead>
          <tr>
            <th>
              <FormattedMessage id="tables.column.label.id" />
            </th>
            <th>
              <FormattedMessage id="tables.column.label.ip" />
            </th>
            <th>
              <FormattedMessage id="tables.column.label.message" />
            </th>
            <th>
              <FormattedMessage id="tables.column.label.created-at" />
            </th>
          </tr>
        </thead>
        <tfoot>
          <tr>
            <th colSpan={4}>
              <PaginationBar
                pagination={item.pagination}
                handleSelect={async (page: IPage) => {
                  const tmp = await get_logs(page);
                  setItem(tmp);
                }}
              />
            </th>
          </tr>
        </tfoot>
        <tbody>
          {item.items.map((it, id) => (
            <tr key={id}>
              <th>{it.id}</th>
              <td>{it.ip}</td>
              <td>{it.message}</td>
              <td>
                <Timestamp value={it.createdAt} />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
};

export default Widget;

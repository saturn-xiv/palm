import { Button, List, Modal } from "antd";
import { useState } from "react";
import { FileImageOutlined } from "@ant-design/icons";
import { FormattedMessage } from "react-intl";

import { IEntry } from "../../../api/hyacinth";
import Memo from "../../../components/Memo";
import Upload from "../../attachments/Upload";
import ShowAttachment from "../../attachments/Show";

interface IProps {
  item: IEntry;
}
const Widget = ({ item }: IProps) => {
  const [open, setOpen] = useState(false);
  return (
    <>
      <Button
        size="small"
        icon={<FileImageOutlined />}
        onClick={() => setOpen(true)}
      >
        <FormattedMessage id="pages.accounting.entries.bills.title" />
      </Button>
      <Modal
        title={<FormattedMessage id="pages.accounting.entries.bills.title" />}
        open={open}
        onOk={() => setOpen(false)}
        onCancel={() => setOpen(false)}
      >
        <List
          size="small"
          header={<Memo text={item.memo} />}
          footer={
            <Upload
              action="/api/accounting/entries/bills-upload"
              resourceId={item.id}
            />
          }
          bordered
          dataSource={item.bills}
          renderItem={(x) => (
            <List.Item key={x.id}>
              <ShowAttachment item={x} />
            </List.Item>
          )}
        />
      </Modal>
    </>
  );
};

export default Widget;

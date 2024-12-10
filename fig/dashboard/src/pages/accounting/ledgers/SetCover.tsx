import { PictureOutlined } from "@ant-design/icons";
import { Button, Modal } from "antd";
import { FormattedMessage } from "react-intl";
import { useState } from "react";

import { ILedger } from "../../../api/hyacinth";
import UploadBox from "../../attachments/Upload";

interface IProps {
  item: ILedger;
}
const Widget = ({ item }: IProps) => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      <Button
        size="small"
        icon={<PictureOutlined />}
        onClick={() => setIsOpen(true)}
      >
        <FormattedMessage id="buttons.cover" />
      </Button>
      <Modal
        title={
          <FormattedMessage id="pages.accounting.ledgers.set-cover.title" />
        }
        open={isOpen}
        onOk={() => setIsOpen(false)}
        onCancel={() => setIsOpen(false)}
      >
        <UploadBox action="/api/accounting/ledger/cover" resourceId={item.id} />
      </Modal>
    </>
  );
};

export default Widget;

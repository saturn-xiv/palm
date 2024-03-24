import { useDropzone, Accept } from "react-dropzone";
import { FormattedMessage, useIntl } from "react-intl";

import { upload } from "../../api";
import { useAppDispatch } from "../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";

interface IProps {
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
  accept: Accept;
  handleRefresh: () => void;
}

interface IItem {
  title: string;
}

const Widget = ({ accept, handleRefresh }: IProps) => {
  const dispatch = useAppDispatch();
  const intl = useIntl();
  const { acceptedFiles, getRootProps, getInputProps } = useDropzone({
    accept,
    onDrop: (files: File[]): void => {
      upload<IItem[]>("/api/attachments", files)
        .then((res: IItem[]) => {
          handleRefresh();
          dispatch(
            success_box(
              res.map((x) =>
                intl.formatMessage(
                  { id: "attachments.upload.succeed" },
                  { title: x.title }
                )
              )
            )
          );
        })
        .catch((reason: string) => {
          dispatch(error_box([reason]));
        });
    },
  });

  const files = acceptedFiles.map((file: File) => (
    <li key={file.name}>
      {file.name} - {file.size} bytes
    </li>
  ));

  return (
    <section>
      <div {...getRootProps({ className: "dropzone" })}>
        <input {...getInputProps()} />
        <p>
          <FormattedMessage id="attachments.upload.helper" />
        </p>
      </div>
      <aside>
        <FormattedMessage tagName="h4" id="attachments.upload.files" />
        <ul>{files}</ul>
      </aside>
    </section>
  );
};

export default Widget;

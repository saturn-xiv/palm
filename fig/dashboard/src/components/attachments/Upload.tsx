import { useDropzone, Accept } from "react-dropzone";
import { FormattedMessage } from "react-intl";

import { upload } from "../../api";

interface IProps {
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
  accept: Accept;
  handleRefresh: () => void;
}

const Widget = ({ accept, handleRefresh }: IProps) => {
  const { acceptedFiles, getRootProps, getInputProps } = useDropzone({
    accept,
    onDrop: (files: File[]): void => {
      for (const file of files) {
        upload("/api/attachments", file).then(() => {
          handleRefresh();
        });
      }
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

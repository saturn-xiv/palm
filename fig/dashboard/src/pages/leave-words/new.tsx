import { useState } from "react";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import CommentOutlinedIcon from "@mui/icons-material/CommentOutlined";
import { useIntl } from "react-intl";
import { string as yup_string, object as yup_object } from "yup";

import AnonymousForm from "../../layouts/application/Form";
import { create_leave_word, EDITOR_TEXTAREA } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import MessageBox, { IState as IMessageBox } from "../../components/MessageBox";

const validationSchema = yup_object({
  content: yup_string().required().min(15).max(1023),
});

export const Component = () => {
  const [messageBox, setMessageBox] = useState<IMessageBox>({
    messages: [],
    severity: "info",
  });

  const intl = useIntl();
  const formik = useFormik({
    initialValues: {
      content: "",
    },
    validationSchema,
    onSubmit: (values) => {
      create_leave_word(values.content, EDITOR_TEXTAREA)
        .then(() => {
          setMessageBox({
            messages: [intl.formatMessage({ id: "flashes.succeed" })],
            severity: "success",
          });
        })
        .catch((reason: IErrorMessage[]) => {
          setMessageBox({
            messages: reason.map((x) => x.message),
            severity: "error",
          });
        });
    },
  });

  return (
    <>
      <MessageBox
        severity={messageBox.severity}
        messages={messageBox.messages}
        handleClose={() => setMessageBox({ messages: [], severity: "info" })}
      />
      <AnonymousForm
        icon={<CommentOutlinedIcon />}
        title="leave-words.new.title"
        handleSubmit={formik.handleSubmit}
      >
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.content.label" })}
          name="content"
          multiline
          rows={4}
          value={formik.values.content}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.content && Boolean(formik.errors.content)}
          helperText={formik.touched.content && formik.errors.content}
        />
      </AnonymousForm>
    </>
  );
};

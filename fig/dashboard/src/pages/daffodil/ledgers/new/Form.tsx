import { useState, useEffect } from "react";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import InputLabel from "@mui/material/InputLabel";
import { FormattedMessage, useIntl } from "react-intl";
import {
  string as yup_string,
  number as yup_number,
  object as yup_object,
} from "yup";
import { useNavigate } from "react-router-dom";
import FormControl from "@mui/material/FormControl";
import Stack from "@mui/material/Stack";

import { IErrorMessage } from "../../../../api/graphql";
import { create_ledger } from "../../../../api/daffodil";
import { IAttachment, index_picture } from "../../../../api/camelia";
import { useAppDispatch } from "../../../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../../../reducers/message-box";

const validationSchema = yup_object({
  name: yup_string().required().min(1).max(31),
  summary: yup_string().required().min(1).max(511),
  cover: yup_number().required(),
});

function Widget() {
  const navigate = useNavigate();
  const dispatch = useAppDispatch();
  const intl = useIntl();
  const [pictures, setPictures] = useState<IAttachment[]>([]);
  useEffect(() => {
    index_picture().then((res) => {
      setPictures(res);
    });
  }, []);

  const formik = useFormik({
    initialValues: {
      name: "",
      summary: "",
      cover: undefined,
    },
    validationSchema,
    onSubmit: (values) => {
      if (values.cover === undefined) {
        dispatch(
          error_box([
            intl.formatMessage({
              id: "attachments.select-picture.required",
            }),
          ])
        );
        return;
      }
      create_ledger(values.name, values.summary, values.cover)
        .then(() => {
          dispatch(
            success_box([intl.formatMessage({ id: "flashes.succeed" })])
          );
          navigate("/dashboard/daffodil/ledgers");
        })
        .catch((reason: IErrorMessage[]) => {
          dispatch(error_box(reason.map((x) => x.message)));
        });
    },
  });
  return (
    <>
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="daffodil.ledgers.new.title" />
      </Typography>
      <Box
        component="form"
        onSubmit={formik.handleSubmit}
        noValidate
        sx={{ mt: 1 }}
      >
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.name.label" })}
          name="name"
          value={formik.values.name}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.name && Boolean(formik.errors.name)}
          helperText={formik.touched.name && formik.errors.name}
        />
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.summary.label" })}
          name="summary"
          multiline
          rows={4}
          value={formik.values.summary}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.summary && Boolean(formik.errors.summary)}
          helperText={formik.touched.summary && formik.errors.summary}
        />
        <FormControl margin="normal" required fullWidth>
          <InputLabel id="ledger-cover-select-label">
            <FormattedMessage id="form.fields.cover.label" />
          </InputLabel>
          <Select
            labelId="ledger-cover-select-label"
            name="cover"
            value={formik.values.cover || ""}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          >
            {pictures.map((x, i) => (
              <MenuItem key={i} value={x.id}>
                {x.title}
              </MenuItem>
            ))}
          </Select>
        </FormControl>

        <Stack spacing={2} direction="row" justifyContent="flex-end">
          <Button
            variant="contained"
            color="inherit"
            onClick={() => {
              navigate(`/dashboard/daffodil/ledgers`);
            }}
          >
            <FormattedMessage id="buttons.go-back" />
          </Button>
          <Button variant="contained" type="submit">
            <FormattedMessage id="buttons.submit" />
          </Button>
        </Stack>
      </Box>
    </>
  );
}

export default Widget;

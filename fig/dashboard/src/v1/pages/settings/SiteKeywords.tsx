import { useState } from "react";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import Chip from "@mui/material/Chip";
import Autocomplete from "@mui/material/Autocomplete";

import { IErrorMessage } from "../../api/graphql";
import { useAppDispatch } from "../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";
import { set_site_keywords } from "../../api/camelia";

interface IProps {
  keywords: string[];
  handleRefresh: () => void;
}

const Widget = ({ keywords, handleRefresh }: IProps) => {
  const [items, setItems] = useState<string[]>(keywords);
  const intl = useIntl();
  const dispatch = useAppDispatch();

  return (
    <>
      <Typography variant="h6" gutterBottom>
        <FormattedMessage id="settings.info.keywords.title" />
      </Typography>
      <Box
        component="form"
        onSubmit={(e) => {
          e.preventDefault();
          set_site_keywords(items)
            .then(() => {
              handleRefresh();
              dispatch(
                success_box([intl.formatMessage({ id: "flashes.succeed" })])
              );
            })
            .catch((reason: IErrorMessage[]) => {
              dispatch(error_box(reason.map((x) => x.message)));
            });
        }}
        noValidate
        sx={{ mt: 1 }}
      >
        <Autocomplete
          multiple
          options={[]}
          freeSolo
          value={items}
          renderTags={(value: readonly string[], getTagProps) =>
            value.map((option: string, index: number) => (
              <Chip
                variant="outlined"
                label={option}
                {...getTagProps({ index })}
              />
            ))
          }
          onChange={(_event, values: string[]) => {
            setItems(values);
          }}
          renderInput={(params) => (
            <TextField
              name="items"
              {...params}
              variant="filled"
              label={intl.formatMessage({ id: "form.fields.keywords.label" })}
            />
          )}
        />

        <Button type="submit" variant="contained" sx={{ mt: 3, mb: 2 }}>
          <FormattedMessage id="buttons.submit" />
        </Button>
      </Box>
    </>
  );
};
export default Widget;

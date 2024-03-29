import Button from "@mui/material/Button";
import ButtonGroup from "@mui/material/ButtonGroup";
import Typography from "@mui/material/Typography";
import { FormattedMessage, useIntl } from "react-intl";

import { home_url } from "../../utils";
import { ping_baidu, ping_index_now, ping_google } from "../../api/camelia";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";
import { useAppDispatch } from "../../hooks";
import { IErrorMessage } from "../../api/graphql";

const Widget = () => {
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const home = home_url();
  return (
    <>
      <Typography variant="h6" gutterBottom>
        <FormattedMessage id="settings.seo.ping.title" />
      </Typography>
      <ButtonGroup color="inherit" variant="contained" aria-label="ping">
        <Button
          onClick={() => {
            ping_baidu(home)
              .then(() => {
                dispatch(
                  success_box([intl.formatMessage({ id: "flashes.succeed" })])
                );
              })
              .catch((reason: IErrorMessage[]) => {
                dispatch(error_box(reason.map((x) => x.message)));
              });
          }}
        >
          <FormattedMessage id="settings.seo.ping.baidu.label" />
        </Button>
        <Button
          onClick={() => {
            ping_google(home)
              .then(() => {
                dispatch(
                  success_box([intl.formatMessage({ id: "flashes.succeed" })])
                );
              })
              .catch((reason: IErrorMessage[]) => {
                dispatch(error_box(reason.map((x) => x.message)));
              });
          }}
        >
          <FormattedMessage id="settings.seo.ping.google.label" />
        </Button>
        <Button
          onClick={() => {
            ping_index_now(home)
              .then(() => {
                dispatch(
                  success_box([intl.formatMessage({ id: "flashes.succeed" })])
                );
              })
              .catch((reason: IErrorMessage[]) => {
                dispatch(error_box(reason.map((x) => x.message)));
              });
          }}
        >
          <FormattedMessage id="settings.seo.ping.index-now.label" />
        </Button>
      </ButtonGroup>
    </>
  );
};

export default Widget;

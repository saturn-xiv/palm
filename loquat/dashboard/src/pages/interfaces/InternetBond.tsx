import { FormattedMessage, useIntl } from "react-intl";
import * as Yup from "yup";
import { withFormik, type FormikProps, Form, Field } from "formik";
import { useCallback, useEffect, useState } from "react";

import {
  get_internet_bond,
  set_internet_bond,
  type IInternetBond,
} from "../../api/interface";
import {
  NotificationBar,
  type INotificationBarState,
} from "../../components/NotificationBar";
import { useAppDispatch } from "../../hooks";
import { danger as show_danger } from "../../reducers/notification";

interface IProps {
  name: string;
  devices: string[];
}

interface IFormValues {
  enable: boolean;
  interfaces: string[];
}

const InnerForm = (
  props: {
    devices: string[];
    onSubmit: (value: IFormValues) => Promise<void>;
  } & FormikProps<IFormValues>
) => {
  const { devices, values, isSubmitting } = props;
  return (
    <Form>
      <div className="field">
        <div className="control">
          <label className="checkbox">
            <Field type="checkbox" name="enable" />
            <FormattedMessage id="buttons.enable" />
          </label>
        </div>
      </div>
      {values.enable && (
        <div className="field">
          <div className="control">
            <div className="checkboxes">
              {devices.map((it, id) => (
                <label key={id} className="checkbox">
                  <Field type="checkbox" name="interfaces" value={it} />
                  {it}
                </label>
              ))}
            </div>
          </div>
        </div>
      )}

      <div className="field is-grouped">
        <div className="control">
          <button
            className="button is-link"
            type="submit"
            disabled={isSubmitting}
          >
            <FormattedMessage id="buttons.submit" />
          </button>
        </div>
        <div className="control">
          <button type="reset" className="button is-link is-light">
            <FormattedMessage id="buttons.reset" />
          </button>
        </div>
      </div>
    </Form>
  );
};

const IForm = withFormik<
  {
    devices: string[];
    bond?: IInternetBond;
    onSubmit: (value: IFormValues) => Promise<void>;
  },
  IFormValues
>({
  mapPropsToValues: (props) => {
    return {
      enable: props.bond?.enable || false,
      interfaces: props.bond?.interfaces || [],
    };
  },
  enableReinitialize: true,
  validationSchema: Yup.object().shape({}),
  handleSubmit: async (values, { props }) => {
    props.onSubmit(values);
  },
})(InnerForm);

const Widget = ({ name, devices }: IProps) => {
  const intl = useIntl();
  const [item, setItem] = useState<IInternetBond>();
  const [notification, setNotification] = useState<INotificationBarState>();
  const dispatch = useAppDispatch();
  const loadBond = useCallback(async () => {
    const res = await get_internet_bond(name);
    if (res.data?.internetBond) {
      setItem(res.data.internetBond);
    } else if (res.errors) {
      dispatch(show_danger(res.errors));
    }
  }, [name, dispatch]);
  useEffect(() => {
    (async () => {
      await loadBond();
    })();
  }, [loadBond]);
  return (
    <>
      {notification && (
        <NotificationBar
          hidden={async () => {
            setNotification(undefined);
          }}
          state={notification}
        />
      )}
      <IForm
        devices={devices}
        bond={item}
        onSubmit={async (values) => {
          const res = await set_internet_bond(
            name,
            values.interfaces,
            values.enable
          );
          if (res.data?.internetBond) {
            setNotification({
              action: "success",
              messages: [intl.formatMessage({ id: "flashes.succeed" })],
            });
          } else if (res.errors) {
            setNotification({
              action: "danger",
              messages: res.errors.map((it) => it.message),
            });
          }
        }}
      />
    </>
  );
};

export default Widget;

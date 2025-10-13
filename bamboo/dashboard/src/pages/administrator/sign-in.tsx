import { FormattedMessage, useIntl } from "react-intl";
import { useSelector, useDispatch } from "react-redux";
import * as Yup from "yup";
import { withFormik, type FormikProps, Form, Field } from "formik";

import { selectName, signIn, signOut } from "../../reducers/session";

interface IInnerFormValues {
  username: string;
  password: string;
}

const InnerForm = (
  props: { title: string } & FormikProps<IInnerFormValues>
) => {
  const { touched, errors, isSubmitting, title } = props;
  return (
    <Form>
      <div className="is-size-2">{title}</div>
      <div className="field">
        <label className="label">
          <FormattedMessage id="forms.fields.label.username" />
        </label>
        <div className="control">
          <Field className="input" type="text" name="username" />
        </div>
        {touched.username && errors.username && (
          <p className="help is-danger">{errors.username}</p>
        )}
      </div>

      <div className="field">
        <label className="label">
          <FormattedMessage id="forms.fields.label.password" />
        </label>
        <div className="control">
          <Field className="input" type="password" name="password" />
        </div>
        {touched.password && errors.password && (
          <p className="help is-danger">{errors.password}</p>
        )}
      </div>

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

const IForm = withFormik<{ title: string }, IInnerFormValues>({
  mapPropsToValues: () => {
    return {
      username: "",
      password: "",
    };
  },
  validationSchema: Yup.object().shape({
    username: Yup.string().min(2).max(31).required(),
    password: Yup.string().min(6).max(31).required(),
  }),
  handleSubmit: (values) => {
    // TODO
    console.log(values);
  },
})(InnerForm);

const Widget = () => {
  const intl = useIntl();
  return (
    <IForm
      title={intl.formatMessage({ id: "pages.administrator.sign-in.title" })}
    />
  );
};

export const Widget1 = () => {
  // TODO remove
  const name = useSelector(selectName);
  const dispatch = useDispatch();
  return (
    <>
      <div className="is-size-2">
        <FormattedMessage id="pages.administrator.sign-in.title" />
      </div>
      <div className="field">
        <label className="label">
          <FormattedMessage id="forms.fields.label.username" />
        </label>
        <div className="control">
          <input className="input" type="text" />
        </div>
      </div>
      <div className="field">
        <label className="label">
          <FormattedMessage id="forms.fields.label.password" />
        </label>
        <div className="control">
          <input className="input" type="password" />
        </div>
      </div>
      <div className="field is-grouped">
        <div className="control">
          <button className="button is-link">
            <FormattedMessage id="buttons.submit" />
          </button>
        </div>
        <div className="control">
          <button className="button is-link is-light">
            <FormattedMessage id="buttons.reset" />
          </button>
        </div>
      </div>

      <div>
        <div>#{name || "n/a"}#</div>
        <br />
        <button onClick={() => dispatch(signIn({ token: "change-me" }))}>
          sign in
        </button>
        &nbsp;
        <button onClick={() => dispatch(signOut())}>sign out</button>
      </div>
    </>
  );
};

export default Widget;

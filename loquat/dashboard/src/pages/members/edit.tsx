import { FormattedMessage, useIntl } from "react-intl";
import * as Yup from "yup";
import { withFormik, type FormikProps, Form, Field } from "formik";
import { useNavigate } from "react-router";

import {
  danger as show_danger,
  success as show_success,
} from "../../reducers/notification";
import { useAppDispatch } from "../../hooks";
import { update as update_member, type IMember } from "../../api/members";
import { INDEX as INDEX_MEMBER } from ".";

interface IProps {
  item: IMember;
}

interface IFormValues {
  name: string;
  memo: string;
}

const InnerForm = (
  props: {
    title: string;
    onSubmit: (value: IFormValues) => Promise<void>;
  } & FormikProps<IFormValues>
) => {
  const { touched, errors, isSubmitting, title } = props;
  return (
    <Form>
      <div className="is-size-3">{title}</div>

      <div className="field">
        <label className="label">
          <FormattedMessage id="forms.fields.label.name" />
        </label>
        <div className="control">
          <Field className="input" name="name" />
        </div>
        {touched.name && errors.name && (
          <p className="help is-danger">{errors.name}</p>
        )}
      </div>
      <div className="field">
        <label className="label">
          <FormattedMessage id="forms.fields.label.memo" />
        </label>
        <div className="control">
          <Field className="textarea" component="textarea" name="memo" />
        </div>
        {touched.memo && errors.memo && (
          <p className="help is-danger">{errors.memo}</p>
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

const IForm = withFormik<
  {
    title: string;
    member: IMember;
    onSubmit: (value: IFormValues) => Promise<void>;
  },
  IFormValues
>({
  mapPropsToValues: (props) => {
    return {
      name: props.member.name,
      memo: props.member.memo,
    };
  },
  enableReinitialize: true,
  validationSchema: Yup.object().shape({
    name: Yup.string().min(2).max(63).required(),
    memo: Yup.string().min(1).max(2047).required(),
  }),
  handleSubmit: async (values, { props, resetForm }) => {
    props.onSubmit(values);
    resetForm();
  },
})(InnerForm);

const Widget = ({ item }: IProps) => {
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  return (
    <IForm
      title={intl.formatMessage(
        { id: "pages.members.update.title" },
        { sn: item.sn }
      )}
      member={item}
      onSubmit={async (values) => {
        const res = await update_member(item.id, values.name, values.memo);
        if (res.data?.updateMember) {
          navigate(INDEX_MEMBER);
          dispatch(
            show_success([intl.formatMessage({ id: "flashes.succeed" })])
          );
        } else if (res.errors) {
          dispatch(show_danger(res.errors));
        }
      }}
    />
  );
};

export default Widget;

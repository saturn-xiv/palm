import { FormattedMessage } from "react-intl";
import { useSelector, useDispatch } from "react-redux";

import { selectName, signIn, signOut } from "../../reducers/session";

const Widget = () => {
  const name = useSelector(selectName);
  const dispatch = useDispatch();
  return (
    <>
      <h1>Users sign in</h1>
      <div>
        <FormattedMessage id="pages.administrator.sign-in.title" />
        <div>#{name || "n/a"}#</div>
        <br />
        <button onClick={() => dispatch(signIn({ name: "Aaa" }))}>
          sign in
        </button>
        &nbsp;
        <button onClick={() => dispatch(signOut())}>sign out</button>
      </div>
    </>
  );
};

export default Widget;

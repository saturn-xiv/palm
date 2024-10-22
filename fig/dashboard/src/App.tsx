import { Provider } from "react-redux";

import Router from "./Router";
import store from "./store";

const Widget = () => {
  return (
    <Provider store={store}>
      <Router />
    </Provider>
  );
};

export default Widget;

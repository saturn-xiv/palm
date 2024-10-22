import { configureStore } from "@reduxjs/toolkit";

import currentUserReducer from "./reducers/current-user";
import siteReducer from "./reducers/site";

const store = configureStore({
  reducer: {
    currentUser: currentUserReducer,
    site: siteReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;

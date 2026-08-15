import { configureStore } from "@reduxjs/toolkit";

import layoutReducer from "./reducers/layout";
import currentUserReducer from "./reducers/current-user";
import sideBarReducer from "./reducers/side-bar";

const store = configureStore({
  reducer: {
    layout: layoutReducer,
    currentUser: currentUserReducer,
    sideBar: sideBarReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;

export type AppDispatch = typeof store.dispatch;

export default store;

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

export type AppStore = typeof store;
export type RootState = ReturnType<AppStore["getState"]>;
export type AppDispatch = AppStore["dispatch"];

export default store;

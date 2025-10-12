import { Suspense } from "react";

import Loading from "./components/Loading";
import Router from "./Router";

const Widget = () => {
  return (
    <Suspense fallback={<Loading />}>
      <Router />
    </Suspense>
  );
};

export default Widget;

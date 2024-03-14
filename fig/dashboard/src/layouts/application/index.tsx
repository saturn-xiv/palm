import { Outlet } from "react-router-dom";

import SharedLinks from "./SharedLinks";

export function Component() {
  return (
    <>
      <div>application header</div>
      <div>
        <Outlet />
      </div>
      <div>
        <SharedLinks />
      </div>
      <div>application footer</div>
    </>
  );
}

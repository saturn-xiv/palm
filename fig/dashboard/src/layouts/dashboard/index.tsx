import { Outlet } from "react-router-dom";

export function Component() {
  return (
    <>
      <div>dashboard header</div>
      <div>
        <Outlet />
      </div>
      <div>dashboard footer</div>
    </>
  );
}

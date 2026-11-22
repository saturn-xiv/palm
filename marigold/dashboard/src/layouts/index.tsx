import { type ReactNode, Fragment } from "react";
import CssBaseline from "@mui/material/CssBaseline";

interface IProps {
  children: ReactNode;
}

const Widget = ({ children }: IProps) => {
  return (
    <Fragment>
      <CssBaseline enableColorScheme />
      {children}
    </Fragment>
  );
};
export default Widget;

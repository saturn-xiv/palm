import { Link } from "react-router-dom";

function Widget() {
  return (
    <>
      <Link to="/anonymous/users/sign-in">Sign in</Link>
      <Link to="/anonymous/users/sign-up">Sign up</Link>
    </>
  );
}

export default Widget;

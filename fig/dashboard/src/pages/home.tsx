import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Flex, Layout } from "antd";

import { useAppSelector } from "../hooks";
import {
  isSignIn,
  PERSONAL_PATH,
  SIGN_IN_PATH,
} from "../reducers/current-user";
import Copyright from "../layouts/Copyright";

const { Footer } = Layout;

const Widget = () => {
  const is_sign_in = useAppSelector(isSignIn);
  const navigate = useNavigate();
  useEffect(() => {
    navigate(is_sign_in ? PERSONAL_PATH : SIGN_IN_PATH);
  }, [navigate, is_sign_in]);
  return (
    <Flex gap="middle" wrap>
      <Layout>
        <Footer
          style={{
            textAlign: "center",
          }}
        >
          <Copyright />
        </Footer>
      </Layout>
    </Flex>
  );
};

export default Widget;

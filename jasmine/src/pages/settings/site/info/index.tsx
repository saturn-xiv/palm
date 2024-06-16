import { useEffect } from "react";
import { Col } from "antd";

import { useAppDispatch } from "../../../../hooks";
import { set_pathname } from "../../../../reducers/side-bar";
import { SETTINGS_SITE_INFO_PATH } from "../../../../Router";
import Author from "./Author";
import Base from "./Base";
import Favicon from "./Favicon";
import GabCode from "./GabCode";
import IcpCode from "./IcpCode";
import Keywords from "./Keywords";

export const Component = () => {
  const dispatch = useAppDispatch();
  const handleRefresh = () => {};
  useEffect(() => {
    dispatch(set_pathname(SETTINGS_SITE_INFO_PATH));
  }, [dispatch]);

  return (
    <>
      <Col sm={24} md={8}>
        <Base handleRefresh={handleRefresh} />
      </Col>
      <Col sm={24} md={8}>
        <Favicon handleRefresh={handleRefresh} />
      </Col>
      <Col sm={24} md={8}>
        <Author handleRefresh={handleRefresh} />
      </Col>
      <Col sm={24} md={8}>
        <Keywords handleRefresh={handleRefresh} />
      </Col>
      <Col sm={24} md={8}>
        <IcpCode handleRefresh={handleRefresh} />
      </Col>
      <Col sm={24} md={8}>
        <GabCode handleRefresh={handleRefresh} />
      </Col>
    </>
  );
};

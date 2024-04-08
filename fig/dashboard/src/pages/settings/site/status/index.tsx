import { useEffect, useState } from "react";
import { Col } from "antd";

import { useAppDispatch } from "../../../../hooks";
import { set_pathname } from "../../../../reducers/side-bar";
import { SETTINGS_SITE_STATUS_PATH } from "../../../../Router";
import { ISystemStatusResponse, system_status } from "../../../../api/camelia";
import PostgreSql from "./PostgreSql";
import Redis from "./Redis";

export const Component = () => {
  const [item, setItem] = useState<ISystemStatusResponse | undefined>();
  const dispatch = useAppDispatch();
  useEffect(() => {
    dispatch(set_pathname(SETTINGS_SITE_STATUS_PATH));
    system_status().then((res) => {
      setItem(res);
    });
  }, [dispatch]);
  return item ? (
    <>
      <Col sm={24} md={10}>
        <PostgreSql item={item.postgresql} />
      </Col>

      <Col sm={24} md={10}>
        <Redis item={item.redis} />
      </Col>
    </>
  ) : (
    <></>
  );
};

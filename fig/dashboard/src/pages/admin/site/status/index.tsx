import { Row, Col, message } from "antd";
import { useEffect, useState } from "react";

import PostgreSql from "./PostgreSql";
import Redis from "./Redis";
import OpenSearch from "./OpenSearch";
import RabbitMQ from "./RabbitMQ";
import Minio from "./Minio";
import {
  get_site_status,
  IGetSiteStatusResponse,
} from "../../../../api/daffodil";
import { IError } from "../../../../api";

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const [status, setStatus] = useState<IGetSiteStatusResponse>();
  useEffect(() => {
    get_site_status()
      .then((res) => {
        setStatus(res);
      })
      .catch((reason: IError[]) => {
        messageApi.error(reason.map((x) => x.message).join("\n"));
      });
  }, [messageApi]);

  return (
    <Row gutter={[{ md: 24 }, { md: 24 }]}>
      <Col md={24}>{contextHolder}</Col>
      <Col md={12}>
        <PostgreSql item={status?.postgresql} />
      </Col>
      <Col md={8}>
        <RabbitMQ item={status?.rabbitmq} />
      </Col>
      <Col md={8}>
        <OpenSearch item={status?.opensearch} />
      </Col>
      <Col md={8}>
        <Redis item={status?.redis} />
      </Col>
      <Col md={8}>
        <Minio item={status?.minio} />
      </Col>
    </Row>
  );
};

export default Widget;

import graphql from "../../../graphql";

interface IItem {
  redis: IRedis;
  postgresql: IPostgreSql;
  rabbitmq: IRabbitMq;
  clientIp?: string;
  version: string;
  buildTime: string;
  launchedAt: Date;
  createdAt: Date;
}

export interface IRedis {
  info: string;
}
export interface IPostgreSql {
  version: string;
  timestamp: string;
}
export interface IRabbitMq {
  online: boolean;
}

export const get = async (): Promise<IItem> => {
  const res: { siteStatus: IItem } = await graphql(
    `
      query call {
        siteStatus {
          redis {
            info
          }
          postgresql {
            version
            timestamp
          }
          rabbitmq {
            online
          }
          clientIp
          version
          buildTime
          launchedAt
          createdAt
        }
      }
    `,
    {},
  );
  return res.siteStatus;
};

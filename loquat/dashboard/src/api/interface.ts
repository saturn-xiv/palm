import type { IOk } from ".";
import { graphql, type IGraphqlResponse } from "../request";

export const WAN = "bond-wan";
export const LAN = "bond-lan";
export const DMZ = "bond-dmz";

// export const wan_ethernets = (o: IInterfaces): IEthernet[] => {
//   const items = [];
//   if (o.wan && o.wan.enable) {
//     for (const it in o.wan.interfaces) {
//       for (const jt in o.ethernets) {
//         if (o.wan.interfaces[it] === o.ethernets[jt].name) {
//           items.push(o.ethernets[jt]);
//         }
//       }
//     }
//   }
//   return items;
// };

interface IInterfaces {
  wan?: IInternetBond;
  dmz?: IInternetBond;
  ethernets: IEthernet[];
}
export interface IEthernet {
  name: string;
  profile?: IEthernetProfile;
}
interface IInterfacesResponse {
  interfaces: IInterfaces;
}
export const interfaces = async (): Promise<
  IGraphqlResponse<IInterfacesResponse>
> => {
  const res: IGraphqlResponse<IInterfacesResponse> = await graphql(
    `
      query call {
        interfaces {
          wan {
            interfaces
            enable
          }
          dmz {
            interfaces
            address
            dns
            enable
          }
          lan {
            interfaces
            address
            dns
            enable
          }
          ethernets {
            name
            profile {
              __typename
              ... on StaticIp {
                label
                isp
                address
                netmask
                gateway
                dns
                memo
                enable
              }
              ... on DynamicIp {
                label
                isp
                memo
                enable
              }
            }
          }
        }
      }
    `,
    {}
  );
  return res;
};

export interface IIntranetBond {
  interfaces: string[];
  address: string;
  dns: string;
  enable: boolean;
}
interface IGetIntranetBondResponse {
  intranetBond: IIntranetBond;
}
export const get_intranet_bond = async (
  name: string
): Promise<IGraphqlResponse<IGetIntranetBondResponse>> => {
  const res: IGraphqlResponse<IGetIntranetBondResponse> = await graphql(
    `
      query call($name: String!) {
        intranetBond(name: $name) {
          interfaces
          address
          dns
          enable
        }
      }
    `,
    { name }
  );
  return res;
};
export interface IInternetBond {
  interfaces: string[];
  enable: boolean;
}
interface IGetInternetBondResponse {
  internetBond: IInternetBond;
}
export const get_internet_bond = async (
  name: string
): Promise<IGraphqlResponse<IGetInternetBondResponse>> => {
  const res: IGraphqlResponse<IGetInternetBondResponse> = await graphql(
    `
      query call($name: String!) {
        internetBond(name: $name) {
          interfaces
          enable
        }
      }
    `,
    { name }
  );
  return res;
};

interface ISetIntranetBondResponse {
  intranetBond: IOk;
}
export const set_intranet_bond = async (
  name: string,
  interfaces: string[],
  address: string,
  dns: string,
  enable: boolean
): Promise<IGraphqlResponse<ISetIntranetBondResponse>> => {
  const res: IGraphqlResponse<ISetIntranetBondResponse> = await graphql(
    `
      mutation call(
        $name: String!
        $interfaces: [String!]!
        $address: String!
        $dns: Dns!
        $enable: Boolean!
      ) {
        intranetBond(
          name: $name
          interfaces: $interfaces
          address: $address
          dns: $dns
          enable: $enable
        ) {
          createdAt
        }
      }
    `,
    { name, interfaces, address, dns, enable }
  );
  return res;
};
interface ISetInternetBondResponse {
  internetBond: IOk;
}
export const set_internet_bond = async (
  name: string,
  interfaces: string[],
  enable: boolean
): Promise<IGraphqlResponse<ISetInternetBondResponse>> => {
  const res: IGraphqlResponse<ISetInternetBondResponse> = await graphql(
    `
      mutation call(
        $name: String!
        $interfaces: [String!]!
        $enable: Boolean!
      ) {
        internetBond(name: $name, interfaces: $interfaces, enable: $enable) {
          createdAt
        }
      }
    `,
    { name, interfaces, enable }
  );
  return res;
};

interface ISetInterfaceStaticIpResponse {
  setNetworkInterfacePublicStaticIp: IOk;
}
export const set_interface_static_ip = async (
  name: string,
  label: string,
  isp: string,
  address: string,
  netmask: string,
  gateway: string,
  dns: string[],

  memo: string
): Promise<IGraphqlResponse<ISetInterfaceStaticIpResponse>> => {
  const res: IGraphqlResponse<ISetInterfaceStaticIpResponse> = await graphql(
    `
      mutation call(
        $name: String!
        $label: String!
        $isp: Isp!
        $address: String!
        $netmask: String!
        $gateway: String!
        $dns: [String!]!
        $memo: String!
      ) {
        setNetworkInterfacePublicStaticIp(
          name: $name
          label: $label
          isp: $isp
          address: $address
          netmask: $netmask
          gateway: $gateway
          dns: $dns
          memo: $memo
        ) {
          createdAt
        }
      }
    `,
    { name, label, isp, address, netmask, gateway, dns, memo }
  );
  return res;
};

interface ISetInterfaceDhcpResponse {
  setNetworkInterfacePublicDhcp: IOk;
}
export const set_interface_dhcp = async (
  name: string,
  label: string,
  isp: string,
  memo: string
): Promise<IGraphqlResponse<ISetInterfaceDhcpResponse>> => {
  const res: IGraphqlResponse<ISetInterfaceDhcpResponse> = await graphql(
    `
      mutation call(
        $name: String!
        $label: String!
        $isp: Isp!
        $memo: String!
      ) {
        setNetworkInterfacePublicDhcp(
          name: $name
          label: $label
          isp: $isp
          memo: $memo
        ) {
          createdAt
        }
      }
    `,
    { name, label, isp, memo }
  );
  return res;
};

interface IDisableInterfaceResponse {
  disableNetworkInterface: IOk;
}
export const disable_interface = async (
  name: string
): Promise<IGraphqlResponse<IDisableInterfaceResponse>> => {
  const res: IGraphqlResponse<IDisableInterfaceResponse> = await graphql(
    `
      mutation call($name: String!) {
        disableNetworkInterface(name: $name) {
          createdAt
        }
      }
    `,
    { name }
  );
  return res;
};

export interface IDhcp {
  __typename: string;
  label: string;
  isp: string;
  memo: string;
  enable: boolean;
}

export interface IStaticIp {
  __typename: string;
  label: string;
  isp: string;
  address: string;
  netmask: string;
  gateway: string;
  dns: string[];
  memo: string;
  enable: boolean;
}

type IEthernetProfile = IStaticIp | IDhcp;

interface IGetNetworkInterfaceResponse {
  getNetworkInterface: IEthernetProfile;
}

export const get_interface = async (
  name: string
): Promise<IGraphqlResponse<IGetNetworkInterfaceResponse>> => {
  const res: IGraphqlResponse<IGetNetworkInterfaceResponse> = await graphql(
    `
      query call($name: String!) {
        getNetworkInterface(name: $name) {
          __typename
          ... on StaticIp {
            label
            isp
            address
            netmask
            gateway
            dns
            memo
            enable
          }
          ... on DynamicIp {
            label
            isp
            memo
            enable
          }
        }
      }
    `,
    { name }
  );
  return res;
};

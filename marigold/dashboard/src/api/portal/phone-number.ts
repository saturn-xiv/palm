import graphql from "../../graphql";

export interface ICountryDialCode {
  code: number;
  name: string;
  region: string;
}

export const indexCountryDialCodes = async (
  lang: string,
): Promise<ICountryDialCode[]> => {
  const res: { countryDialCodes: ICountryDialCode[] } = await graphql(
    `
      query call($lang: String!) {
        countryDialCodes(lang: $lang) {
          code
          name
          region
        }
      }
    `,
    { lang },
  );
  return res.countryDialCodes;
};

export const DEFAULT_COUNTRY_DIAL_REGION: string =
  import.meta.env.VITE_DEFAULT_COUNTRY_DIAL_REGION || "US";

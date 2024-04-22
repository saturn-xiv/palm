import { GlobalOutlined } from "@ant-design/icons";
import { Dropdown } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { Popconfirm } from "antd";

import { useAppSelector } from "../hooks";
import { siteInfo as selectSiteInfo } from "../reducers/site-info";
import { set as set_locale } from "../locales";

const Widget = () => {
  const intl = useIntl();
  const site_info = useAppSelector(selectSiteInfo);

  return (
    <Dropdown
      menu={{
        items: site_info.languages.map((lang) => {
          return {
            label: (
              <Popconfirm
                title={<FormattedMessage id="flashes.are-you-sure" />}
                description={
                  <FormattedMessage
                    id="layouts.language-bar.confirm"
                    values={{
                      lang: intl.formatMessage({ id: `languages.${lang}` }),
                    }}
                  />
                }
                onConfirm={() => {
                  set_locale(lang, true);
                }}
                okText={<FormattedMessage id="buttons.ok" />}
                cancelText={<FormattedMessage id="buttons.cancel" />}
              >
                <span>
                  <FormattedMessage id={`languages.${lang}`} />
                </span>
              </Popconfirm>
            ),
            key: lang,
          };
        }),
      }}
    >
      <GlobalOutlined />
    </Dropdown>
  );
};

export default Widget;

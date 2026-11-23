import ByEmail from "./by-email/SignIn";
import BySms from "./by-sms/SignIn";
import ByEmailOrSms from "./ByEmailOrSms";

const Widget = () => (
  <ByEmailOrSms
    title="portal.pages.anonymous.users.sign-in.title"
    sms={<BySms />}
    email={<ByEmail />}
  />
);

export default Widget;

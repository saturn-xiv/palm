import ByEmail from "./by-email/SignUp";
import BySms from "./by-sms/SignUp";
import ByEmailOrSms from "./ByEmailOrSms";

const Widget = () => (
  <ByEmailOrSms
    title="portal.pages.anonymous.users.sign-up.title"
    sms={<BySms />}
    email={<ByEmail />}
  />
);

export default Widget;

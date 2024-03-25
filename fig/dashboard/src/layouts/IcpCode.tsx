interface IProps {
  code: string;
}

const Widget = ({ code }: IProps) => {
  return (
    <a href="https://beian.miit.gov.cn/" target="_blank">
      {code}
    </a>
  );
};

export default Widget;

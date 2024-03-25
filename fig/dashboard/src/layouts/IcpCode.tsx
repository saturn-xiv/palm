interface IProps {
  code: string;
}

const Widget = ({ code }: IProps) => {
  return (
    <a href="https://beian.miit.gov.cn/" rel="noreferrer" target="_blank">
      {code}
    </a>
  );
};

export default Widget;

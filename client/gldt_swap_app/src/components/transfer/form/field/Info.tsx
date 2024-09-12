const Info = ({ errors }) => {
  return errors ? (
    <em className="text-dark-orange text-sm ml-4">{errors?.message}</em>
  ) : null;
};

export default Info;

import ContentLoader from 'react-content-loader';

const LoaderDataCard = (props) => (
  <ContentLoader
    speed={2}
    width={400}
    height={150}
    viewBox="0 0 400 150"
    backgroundColor="#f3f3f3"
    foregroundColor="#ecebeb"
    {...props}
  >
    <rect x="12" y="12" rx="5" ry="5" width="240" height="24" />
    <rect x="14" y="64" rx="5" ry="5" width="320" height="48" />
    <circle cx="320" cy="23" r="19" />
  </ContentLoader>
);

export default LoaderDataCard;

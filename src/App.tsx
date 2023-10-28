import AppContainer from './containers/AppContainer.tsx';
import ContentBox from './containers/ContentBox/ContentBox.tsx';

const App = () => (
  <AppContainer>
    <ContentBox title={'Github'} />
    <ContentBox title={'Team City'} />
    <ContentBox title={'Kubernetes'} />
  </AppContainer>
);

export default App;

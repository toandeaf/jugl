import { FC, ReactNode } from 'react';
import './styles.css';

interface Props {
  children: ReactNode;
}

const AppContainer: FC<Props> = ({ children }) => {
  return <div className='container'>{children}</div>;
};

export default AppContainer;

import './styles.css';
import { FC, useEffect, useState } from 'react';
import { tauri } from '@tauri-apps/api';
import PullRequest from './PullRequest.tsx';

interface Props {
  title: string;
}

const testThis = async (): Promise<Array<PullRequest>> => {
  return await tauri.invoke('get_prs');
};

const ContentBox: FC<Props> = ({ title }) => {
  const [vals, setVals] = useState<Array<PullRequest>>([]);

  useEffect(() => {
    testThis().then(prs => {
      setVals(prs);
    });
  }, []);

  return (
    <div className='box'>
      <h1>{title}</h1>
      {vals.map(pr => (
        <PullRequest pr={pr} />
      ))}
    </div>
  );
};

export default ContentBox;

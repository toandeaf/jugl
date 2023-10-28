import './styles.css';
import { FC } from 'react';
import { tauri } from '@tauri-apps/api';

interface Props {
  title: string;
}
const ContentBox: FC<Props> = ({ title }) => {
  const testThis = async () => {
    const dad = await tauri.invoke('get_prs');
    console.log(dad);
  };

  return (
    <div className='box'>
      <h1>{title}</h1>
      <input type='button' value='About' onClick={testThis}></input>
      <input type='button' value='Programming Stuff'></input>
      <input type='button' value='Tiny CV'></input>
      <input type='button' value='Blog?'></input>
    </div>
  );
};

export default ContentBox;

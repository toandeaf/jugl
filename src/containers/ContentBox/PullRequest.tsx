import { FC } from 'react';
import { Box, Flex, Text } from '@chakra-ui/react';
import './styles.css';
interface Props {
  pr: PullRequest;
}

const PullRequest: FC<Props> = ({ pr }) => {
  return (
    <Flex
      gap={20}
      justifyContent={'flex-start'}
      alignItems={'center'}
      borderRadius={'4px'}
      // width={'100%'}
    >
      <Box flex={3} className={'prTitle'} p={'10px'}>
        <Text fontWeight={'bold'}>{pr.title}</Text>
      </Box>
      <Box flex={1} id='stage box'>
        <input type={'button'} value={'1 Approval'} />
      </Box>
      <Box flex={1} id='ownice box'>
        <input type={'button'} value={'PR Owner'} />
      </Box>
      <Box flex={1} id='ownice box'>
        <input type={'button'} value={'Actions'} />
      </Box>
    </Flex>
  );
};

export default PullRequest;

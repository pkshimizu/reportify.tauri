import { Box } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  align?: 'start' | 'end' | 'center' | 'stretch';
  justify?: 'start' | 'end' | 'center' | 'stretch';
  children: ReactNode;
}

export default function RGridItem(props: Props) {
  return (
    <Box sx={{ alignSelf: props.align, justifySelf: props.justify }}>
      {props.children}
    </Box>
  );
}

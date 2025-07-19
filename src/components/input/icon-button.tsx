import { IconButton } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

export default function RIconButton(props: Props) {
  return <IconButton>{props.children}</IconButton>;
}

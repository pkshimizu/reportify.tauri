import { AppBar, Toolbar } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

export default function RAppBar(props: Props) {
  return (
    <AppBar position='static' color='default' elevation={1}>
      <Toolbar>{props.children}</Toolbar>
    </AppBar>
  );
}

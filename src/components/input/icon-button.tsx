import { IconButton } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  children: ReactNode;
  onClick?: () => void;
}

export default function RIconButton(props: Props) {
  const handleClick = () => {
    if (props.onClick) {
      props.onClick();
    }
  };

  return <IconButton onClick={handleClick}>{props.children}</IconButton>;
}

import { Button } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  onClick: () => void;
  loading?: boolean;
  children: ReactNode;
}

export default function RButton({ onClick, loading, children }: Props) {
  return (
    <Button loading={loading} onClick={onClick}>
      {children}
    </Button>
  );
}

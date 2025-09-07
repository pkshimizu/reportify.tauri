import { Button } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  onClick: () => void;
  loading?: boolean;
  width?: number;
  height?: number;
  fullWidth?: boolean;
  fullHeight?: boolean;
  children: ReactNode;
}

export default function RButton({
  onClick,
  loading,
  width,
  height,
  fullWidth,
  fullHeight,
  children,
}: Props) {
  return (
    <Button
      loading={loading}
      onClick={onClick}
      sx={{
        ...(width && { width: width }),
        ...(height && { height: height }),
        ...(fullWidth && { width: '100%' }),
        ...(fullHeight && { height: '100%' }),
      }}
    >
      {children}
    </Button>
  );
}

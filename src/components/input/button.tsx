import { MainColor, bgColor } from '@/models/color';
import { Button, ButtonProps } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  variant?: 'contained' | 'outlined' | 'text';
  onClick: () => void;
  loading?: boolean;
  width?: number;
  height?: number;
  color?: MainColor;
  fullWidth?: boolean;
  fullHeight?: boolean;
  children: ReactNode;
}

export default function RButton({
  variant,
  onClick,
  loading,
  width,
  height,
  color,
  fullWidth,
  fullHeight,
  children,
}: Props) {
  return (
    <Button
      variant={variant}
      loading={loading}
      onClick={onClick}
      color={bgColor(color) as ButtonProps['color']}
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

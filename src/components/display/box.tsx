import { Box } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  width?: number;
  height?: number;
  bgcolor?: 'primary' | 'secondary' | 'error' | 'warning' | 'info' | 'success';
  align?: 'center' | 'start' | 'end';
  justify?: 'center' | 'start' | 'end';
  aspectRatio?: number;
  children: ReactNode;
}

export default function RBox(props: Props) {
  return (
    <Box
      sx={{
        ...(props.width && { width: props.width }),
        ...(props.height && { height: props.height }),
        ...(props.bgcolor && { bgcolor: props.bgcolor }),
        ...(props.align && { alignContent: props.align }),
        ...(props.justify && { justifyContent: props.justify }),
        ...(props.aspectRatio && { aspectRatio: props.aspectRatio }),
      }}
    >
      {props.children}
    </Box>
  );
}

export function RSquareBox(
  props: Omit<Props, 'width' | 'height'> & { size?: number }
) {
  return (
    <RBox {...props} width={props.size} height={props.size} aspectRatio={1} />
  );
}

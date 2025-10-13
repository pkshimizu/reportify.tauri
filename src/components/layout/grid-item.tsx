import { Box } from '@mui/material';
import { ReactNode } from 'react';

interface Area {
  top: number;
  left: number;
  bottom: number;
  right: number;
}

interface Props {
  align?: 'start' | 'end' | 'center' | 'stretch';
  justify?: 'start' | 'end' | 'center' | 'stretch';
  area?: Area;
  fullWidth?: boolean;
  fullHeight?: boolean;
  children: ReactNode;
}

export default function RGridItem(props: Props) {
  const { area } = props;
  const gridArea = area
    ? `${area.top} / ${area.left} / ${area.bottom} / ${area.right}`
    : undefined;
  return (
    <Box
      sx={{
        alignSelf: props.align,
        justifySelf: props.justify,
        gridArea,
        ...(props.fullWidth && { width: '100%' }),
        ...(props.fullHeight && { height: '100%' }),
      }}
    >
      {props.children}
    </Box>
  );
}

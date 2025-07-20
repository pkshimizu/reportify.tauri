import { Typography } from '@mui/material';
import { Color, textColor } from '../../models/color';

interface Props {
  align?: 'center' | 'start' | 'end';
  color?: Color;
  children: string | number;
}

export default function RText(props: Props) {
  return (
    <Typography sx={{ textAlign: props.align, color: textColor(props.color) }}>
      {props.children}
    </Typography>
  );
}

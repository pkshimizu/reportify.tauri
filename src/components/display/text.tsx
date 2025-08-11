import { Color, textColor } from '@/models/color';
import { Typography } from '@mui/material';

type Text = string | number | Text[];

interface Props {
  align?: 'center' | 'start' | 'end';
  color?: Color;
  children: Text;
}

export default function RText(props: Props) {
  return (
    <Typography sx={{ textAlign: props.align, color: textColor(props.color) }}>
      {props.children}
    </Typography>
  );
}

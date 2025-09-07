import { Color, textColor } from '@/models/color';
import { Typography } from '@mui/material';

type Text = string | number | Text[];

interface Props {
  align?: 'center' | 'start' | 'end';
  color?: Color;
  size?: 'small' | 'medium' | 'large';
  whiteSpace?: 'normal' | 'nowrap' | 'pre' | 'pre-wrap' | 'pre-line';
  overflow?: 'hidden' | 'visible' | 'scroll' | 'auto';
  children: Text;
}

export default function RText(props: Props) {
  return (
    <Typography
      sx={{
        textAlign: props.align,
        color: textColor(props.color),
        fontSize: props.size,
        whiteSpace: props.whiteSpace,
        overflow: props.overflow,
        textOverflow: 'ellipsis',
      }}
    >
      {props.children}
    </Typography>
  );
}

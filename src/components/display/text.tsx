import { Typography } from '@mui/material';

interface Props {
  align?: 'center' | 'start' | 'end';
  children: string | number;
}

export default function RText(props: Props) {
  return (
    <Typography sx={{ textAlign: props.align }}>{props.children}</Typography>
  );
}

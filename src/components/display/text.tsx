import { Typography } from '@mui/material';

interface Props {
  children: string | number;
}

export default function RText(props: Props) {
  return <Typography>{props.children}</Typography>;
}

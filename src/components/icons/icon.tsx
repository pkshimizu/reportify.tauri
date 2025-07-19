import { SvgIcon } from '@mui/material';
import { ElementType } from 'react';

export interface IconProps {
  color?:
    | 'inherit'
    | 'primary'
    | 'secondary'
    | 'error'
    | 'info'
    | 'success'
    | 'warning';
}

interface Props extends IconProps {
  icon: ElementType;
}

export default function RIcon(props: Props) {
  return <SvgIcon component={props.icon} color={props.color} />;
}

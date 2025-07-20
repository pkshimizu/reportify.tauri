import { SvgIcon } from '@mui/material';
import { ElementType } from 'react';
import { MainColor, textColor } from '../../models/color';

export interface IconProps {
  color?: MainColor;
}

interface Props extends IconProps {
  icon: ElementType;
}

export default function RIcon(props: Props) {
  return <SvgIcon component={props.icon} color={textColor(props.color)} />;
}

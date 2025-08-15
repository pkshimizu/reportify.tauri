import { SvgIcon } from '@mui/material';
import { ElementType } from 'react';
import { MainColor, iconColor } from '@/models/color';

export interface IconProps {
  color?: MainColor;
  size?: 'small' | 'medium' | 'large';
}

interface Props extends IconProps {
  icon: ElementType;
}

export default function RIcon(props: Props) {
  return (
    <SvgIcon
      component={props.icon}
      color={iconColor(props.color)}
      fontSize={props.size}
    />
  );
}

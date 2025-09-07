import { blue, grey, red } from '@mui/material/colors';

export type MainColor =
  | 'inherit'
  | 'primary'
  | 'secondary'
  | 'error'
  | 'warning'
  | 'info'
  | 'success';

export type DayColor = 'weekday' | 'sat' | 'sun' | 'outside';

export type Color = MainColor | DayColor;

export function textColor(color?: Color) {
  if (!color) return undefined;
  switch (color) {
    case 'inherit':
      return 'inherit';
    case 'weekday':
      return grey[900];
    case 'sat':
      return blue[900];
    case 'sun':
      return red[900];
    case 'outside':
      return 'text.disabled';
    default:
      return `${color}.contrastText`;
  }
}

export function bgColor(color?: Color) {
  if (!color) return undefined;
  switch (color) {
    case 'inherit':
      return 'inherit';
    case 'weekday':
      return grey[100];
    case 'sat':
      return blue[100];
    case 'sun':
      return red[100];
    default:
      return `${color}.main`;
  }
}

export function iconColor(color?: MainColor) {
  if (!color) return undefined;
  return color;
}

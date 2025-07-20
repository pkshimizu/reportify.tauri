export type MainColor =
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
    case 'weekday':
      return 'text.primary';
    case 'sat':
      return '#0000ff';
    case 'sun':
      return '#ff0000';
    case 'outside':
      return 'text.disabled';
    default:
      return `${color}.contrastText`;
  }
}

export function bgColor(color?: Color) {
  if (!color) return undefined;
  switch (color) {
    case 'weekday':
      return 'background.paper';
    case 'sat':
      return 'background.secondary';
    case 'sun':
      return 'background.tertiary';
    default:
      return `${color}.main`;
  }
}

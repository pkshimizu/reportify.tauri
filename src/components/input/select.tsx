import { MenuItem, Select } from '@mui/material';
import { ReactNode } from 'react';

interface Item {
  value: string;
  label: ReactNode;
}

interface Props {
  value?: string;
  items: Item[];
  onChange: (value: string) => void;
}

export default function RSelect(props: Props) {
  return (
    <Select
      value={props.value}
      size='small'
      onChange={event => props.onChange(event.target.value)}
    >
      {props.items.map(item => (
        <MenuItem key={item.value} value={item.value}>
          {item.label}
        </MenuItem>
      ))}
    </Select>
  );
}

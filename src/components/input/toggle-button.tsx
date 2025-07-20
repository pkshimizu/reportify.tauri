import { ToggleButton, ToggleButtonGroup } from '@mui/material';
import { MouseEvent, ReactNode } from 'react';

interface Item {
  value: string;
  content: ReactNode;
}

interface Props {
  items: Item[];
  value: string;
  onChange: (value: string) => void;
}

export default function RToggleButton(props: Props) {
  const handleChange = (_: MouseEvent<HTMLElement>, value: string) => {
    console.log('value', value);
    props.onChange(value);
  };

  return (
    <ToggleButtonGroup
      value={props.value}
      exclusive
      size='small'
      onChange={handleChange}
    >
      {props.items.map(item => (
        <ToggleButton key={item.value} value={item.value}>
          {item.content}
        </ToggleButton>
      ))}
    </ToggleButtonGroup>
  );
}

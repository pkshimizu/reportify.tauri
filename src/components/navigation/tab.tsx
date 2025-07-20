import { Tab, Tabs } from '@mui/material';
import { ReactNode } from 'react';
import RBox from '../display/box';

interface Item {
  label: string;
  value: string;
  panel: ReactNode;
}

interface Props {
  items: Item[];
  value: string;
  orientation?: 'horizontal' | 'vertical';
  onChange: (value: string) => void;
}

export default function RTab(props: Props) {
  return (
    <RBox>
      <Tabs
        value={props.value}
        orientation={props.orientation}
        onChange={(_, value) => props.onChange(value)}
      >
        {props.items.map(item => (
          <Tab
            key={item.value}
            label={item.label}
            value={item.value}
            sx={{ textTransform: 'none' }}
          />
        ))}
      </Tabs>
      <RBox m={2}>
        {props.items.find(item => item.value === props.value)?.panel}
      </RBox>
    </RBox>
  );
}

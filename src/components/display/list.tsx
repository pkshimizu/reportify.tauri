import {
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
} from '@mui/material';
import { ReactNode } from 'react';

interface Item {
  id: string;
  text: string;
  icon?: ReactNode;
  selected?: boolean;
  onClick?: () => void;
}

interface Props {
  items: Item[];
}

function RListItem(props: { item: Item }) {
  const item = props.item;

  if (item.onClick) {
    return (
      <ListItemButton onClick={item.onClick} selected={item.selected}>
        {item.icon && <ListItemIcon>{item.icon}</ListItemIcon>}
        <ListItemText primary={item.text} />
      </ListItemButton>
    );
  }
  return (
    <ListItem>
      {item.icon && <ListItemIcon>{item.icon}</ListItemIcon>}
      <ListItemText primary={item.text} />
    </ListItem>
  );
}

export default function RList(props: Props) {
  return (
    <List>
      {props.items.map(item => (
        <RListItem key={item.id} item={item} />
      ))}
    </List>
  );
}

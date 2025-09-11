import {
  Dialog,
  DialogActions,
  DialogContent,
  DialogProps,
  DialogTitle,
} from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  open: boolean;
  title?: string;
  size?: DialogProps['maxWidth'];
  children: ReactNode;
  actions?: ReactNode;
  onClose: () => void;
}

export default function RDialog(props: Props) {
  return (
    <Dialog
      open={props.open}
      fullWidth
      maxWidth={props.size}
      onClose={props.onClose}
    >
      {props.title && <DialogTitle>{props.title}</DialogTitle>}
      <DialogContent>{props.children}</DialogContent>
      {props.actions && <DialogActions>{props.actions}</DialogActions>}
    </Dialog>
  );
}

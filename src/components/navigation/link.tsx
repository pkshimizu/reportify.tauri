import { Link } from '@mui/material';
import { Link as RouterLink } from '@tanstack/react-router';
import { ReactNode } from 'react';

interface Props {
  href: string;
  children: ReactNode;
}

export default function RLink(props: Props) {
  return (
    <Link
      component={RouterLink}
      href={props.href}
      sx={{ textDecoration: 'none', color: 'inherit' }}
    >
      {props.children}
    </Link>
  );
}

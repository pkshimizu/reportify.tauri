import { Link } from '@mui/material';
import { Link as RouterLink } from '@tanstack/react-router';
import { ReactNode } from 'react';

interface Props {
  href: string;
  overflow?: 'hidden' | 'visible' | 'scroll' | 'auto';
  children: ReactNode;
}

export default function RLink(props: Props) {
  const external =
    props.href.startsWith('http') ||
    props.href.startsWith('mailto:') ||
    props.href.startsWith('tel:');
  const component = external ? 'a' : RouterLink;
  const target = external ? '_blank' : '_self';

  return (
    <Link
      component={component}
      href={props.href}
      target={target}
      sx={{
        textDecoration: 'none',
        color: 'inherit',
        overflow: props.overflow,
      }}
    >
      {props.children}
    </Link>
  );
}

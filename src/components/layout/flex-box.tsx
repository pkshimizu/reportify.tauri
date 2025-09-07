import { Box } from '@mui/material';
import React from 'react';

interface Props {
  direction?: 'row' | 'column' | 'row-reverse' | 'column-reverse';
  justify?:
    | 'flex-start'
    | 'center'
    | 'flex-end'
    | 'space-between'
    | 'space-around'
    | 'space-evenly';
  align?: 'flex-start' | 'center' | 'flex-end' | 'stretch' | 'baseline';
  alignContent?:
    | 'flex-start'
    | 'center'
    | 'flex-end'
    | 'space-between'
    | 'space-around'
    | 'space-evenly'
    | 'stretch';
  wrap?: 'nowrap' | 'wrap' | 'wrap-reverse';
  gap?: number | string;
  rowGap?: number | string;
  columnGap?: number | string;
  grow?: number;
  shrink?: number;
  basis?: number | string;
  flex?: string | number;
  overflow?: 'hidden' | 'visible' | 'scroll' | 'auto';
  height?: number;
  fullHeight?: boolean;
  children: React.ReactNode;
}

export default function RFlexBox({
  direction = 'row',
  justify = 'flex-start',
  align = 'stretch',
  alignContent,
  wrap = 'nowrap',
  gap,
  rowGap,
  columnGap,
  grow,
  shrink,
  basis,
  flex,
  overflow,
  height,
  fullHeight,
  children,
}: Props) {
  const flexStyles = {
    display: 'flex',
    flexDirection: direction,
    justifyContent: justify,
    alignItems: align,
    ...(alignContent && { alignContent }),
    flexWrap: wrap,
    ...(gap !== undefined && { gap }),
    ...(rowGap !== undefined && { rowGap }),
    ...(columnGap !== undefined && { columnGap }),
    ...(grow !== undefined && { flexGrow: grow }),
    ...(shrink !== undefined && { flexShrink: shrink }),
    ...(basis !== undefined && { flexBasis: basis }),
    ...(flex !== undefined && { flex }),
    ...(overflow !== undefined && { overflow }),
    width: '100%',
    ...(height !== undefined && { height }),
    ...(fullHeight && { height: '100%' }),
  };

  return <Box sx={{ ...flexStyles }}>{children}</Box>;
}

// Convenience components for common flex layouts
export function RRow(props: Omit<Props, 'direction'>) {
  return <RFlexBox direction='row' {...props} />;
}

export function RColumn(props: Omit<Props, 'direction'>) {
  return <RFlexBox direction='column' {...props} />;
}

export function RCenter(props: Omit<Props, 'justify' | 'align'>) {
  return <RFlexBox justify='center' align='center' {...props} />;
}

export function RSpaceBetween(props: Omit<Props, 'justify' | 'grow'>) {
  return <RFlexBox justify='space-between' grow={1} {...props} />;
}

export function RSpaceAround(props: Omit<Props, 'justify' | 'grow'>) {
  return <RFlexBox justify='space-around' grow={1} {...props} />;
}

export function RSpaceEvenly(props: Omit<Props, 'justify' | 'grow'>) {
  return <RFlexBox justify='space-evenly' grow={1} {...props} />;
}

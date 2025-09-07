import { Color, bgColor } from '@/models/color';
import { Box } from '@mui/material';
import { ReactNode } from 'react';

interface Props {
  width?: number;
  height?: number;
  bgcolor?: Color;
  align?: 'center' | 'start' | 'end';
  justify?: 'center' | 'start' | 'end';
  aspectRatio?: number;
  m?: number;
  mt?: number;
  mr?: number;
  mb?: number;
  ml?: number;
  mx?: number;
  my?: number;
  p?: number;
  pt?: number;
  pr?: number;
  pb?: number;
  pl?: number;
  px?: number;
  py?: number;
  children: ReactNode;
}

export default function RBox(props: Props) {
  return (
    <Box
      display='flex'
      sx={{
        ...(props.width && { width: props.width }),
        ...(props.height && { height: props.height }),
        ...(props.bgcolor && { bgcolor: bgColor(props.bgcolor) }),
        ...(props.align && { alignItems: props.align }),
        ...(props.justify && { justifyContent: props.justify }),
        ...(props.aspectRatio && { aspectRatio: props.aspectRatio }),
        ...(props.m && { margin: props.m }),
        ...(props.mt && { marginTop: props.mt }),
        ...(props.mr && { marginRight: props.mr }),
        ...(props.mb && { marginBottom: props.mb }),
        ...(props.ml && { marginLeft: props.ml }),
        ...(props.mx && { marginLeft: props.mx, marginRight: props.mx }),
        ...(props.my && { marginTop: props.my, marginBottom: props.my }),
        ...(props.p && { padding: props.p }),
        ...(props.pt && { paddingTop: props.pt }),
        ...(props.pr && { paddingRight: props.pr }),
        ...(props.pb && { paddingBottom: props.pb }),
        ...(props.pl && { paddingLeft: props.pl }),
        ...(props.px && { paddingLeft: props.px, paddingRight: props.px }),
        ...(props.py && { paddingTop: props.py, paddingBottom: props.py }),
        flexGrow: 1,
      }}
    >
      {props.children}
    </Box>
  );
}

export function RSquareBox(
  props: Omit<Props, 'width' | 'height'> & { size?: number }
) {
  return (
    <RBox {...props} width={props.size} height={props.size} aspectRatio={1} />
  );
}

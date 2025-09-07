interface Props {
  src: string;
  alt: string;
  width?: number;
  height?: number;
  circle?: boolean;
}

export default function RImage(props: Props) {
  return (
    <img
      {...props}
      style={{
        ...(props.width && { width: props.width }),
        ...(props.height && { height: props.height }),
        ...(props.circle && { borderRadius: '50%' }),
      }}
    />
  );
}

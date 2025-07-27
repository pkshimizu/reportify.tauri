import { TextField } from '@mui/material';
import { ChangeEvent } from 'react';

interface Props {
  value: string;
  readonly?: boolean;
  onChange: (value: string) => void;
}

export default function RTextField(props: Props) {
  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    props.onChange(event.target.value);
  };

  return (
    <TextField
      value={props.value}
      onChange={handleChange}
      size='small'
      inputProps={{ readOnly: props.readonly }}
    />
  );
}

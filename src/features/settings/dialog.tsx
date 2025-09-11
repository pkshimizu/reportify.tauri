import RDialog from '@/components/feedback/dialog';

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function SettingsDialog(props: Props) {
  return (
    <RDialog onClose={props.onClose} open={props.open} size='md'>
      aaaa
    </RDialog>
  );
}

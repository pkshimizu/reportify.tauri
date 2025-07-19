import { createFileRoute } from '@tanstack/react-router';
import RText from '../components/display/text';
import { RCenter } from '../components/layout/flex-box';

export const Route = createFileRoute('/calendar')({
  component: Calendar,
});

function Calendar() {
  return (
    <RCenter>
      <RText>Calendar</RText>
    </RCenter>
  );
}

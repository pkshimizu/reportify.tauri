import { createFileRoute } from '@tanstack/react-router';
import { RCenter } from '../components/layout/flex-box';
import ActivityCalendar from '../features/activity-calendar/calendar';

export const Route = createFileRoute('/calendar')({
  component: Calendar,
});

function Calendar() {
  return (
    <RCenter>
      <ActivityCalendar />
    </RCenter>
  );
}

import RBox from '@/components/display/box';
import { RCenter, RColumn } from '@/components/layout/flex-box';
import ActivityWeeklyCalendar from '@/features/activity/weekly-calendar';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <RCenter>
      <RColumn gap={2}>
        <RBox m={2}>
          <ActivityWeeklyCalendar />
        </RBox>
      </RColumn>
    </RCenter>
  );
}

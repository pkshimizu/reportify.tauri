import { RCenter } from '@/components/layout/flex-box';
import ActivityCalendar from '@/features/activity-calendar/calendar';
import useActivities from '@/hooks/activities';
import { Activity } from '@/models/activity';
import { createFileRoute } from '@tanstack/react-router';
import { useEffect, useState } from 'react';

export const Route = createFileRoute('/calendar')({
  component: Calendar,
});

function Calendar() {
  const [date, setDate] = useState(() => {
    const date = new Date();
    date.setDate(1);
    return date;
  });
  const { loadActivities } = useActivities();
  const [activities, setActivities] = useState<Activity[]>([]);

  useEffect(() => {
    const fetchActivities = async (year: number, month: number) => {
      const activities = await loadActivities(year, month);
      setActivities(activities);
    };
    fetchActivities(date.getFullYear(), date.getMonth() + 1);
  }, [date, loadActivities]);

  return (
    <RCenter>
      <ActivityCalendar
        date={date}
        activities={activities}
        onChangeDate={setDate}
      />
    </RCenter>
  );
}

import RButton from '@/components/input/button';
import { RCenter, RColumn } from '@/components/layout/flex-box';
import ActivityCalendar from '@/features/activity-calendar/calendar';
import useActivities from '@/hooks/activities';
import { Activity } from '@/models/activity';
import { createFileRoute } from '@tanstack/react-router';
import dayjs from 'dayjs';
import { useCallback, useEffect, useState } from 'react';

export const Route = createFileRoute('/calendar')({
  component: Calendar,
});

function Calendar() {
  const [date, setDate] = useState(() => {
    const date = new Date();
    date.setDate(1);
    return date;
  });
  const { loadActivities, fetchGitHubEventsWithRange, fetching } =
    useActivities();
  const [activities, setActivities] = useState<Activity[]>([]);

  useEffect(() => {
    const fetchActivities = async (year: number, month: number) => {
      const activities = await loadActivities(year, month);
      setActivities(activities);
    };
    fetchActivities(date.getFullYear(), date.getMonth() + 1);
  }, [date, loadActivities]);

  const handleFetch = useCallback(() => {
    const startDate = dayjs(date).startOf('month');
    const endDate = dayjs(date).endOf('month');
    fetchGitHubEventsWithRange(startDate.toISOString(), endDate.toISOString());
  }, [date, fetchGitHubEventsWithRange]);

  return (
    <RCenter>
      <RColumn>
        <RButton loading={fetching} onClick={handleFetch}>
          Fetch GitHub Events
        </RButton>
        <ActivityCalendar
          date={date}
          activities={activities}
          onChangeDate={setDate}
        />
      </RColumn>
    </RCenter>
  );
}

import { RSquareBox } from '@/components/display/box';
import RText from '@/components/display/text';
import RIconButton from '@/components/input/icon-button';
import {
  RCenter,
  RColumn,
  RRow,
  RSpaceBetween,
} from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import RLink from '@/components/navigation/link';
import { Activity } from '@/models/activity';
import {
  ChevronLeft as ChevronLeftIcon,
  ChevronRight as ChevronRightIcon,
} from '@mui/icons-material';
import dayjs from 'dayjs';
import { useCallback } from 'react';
import ActivityCounts from './counts';

const weekDays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const monthNames = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
];

interface Props {
  date: Date;
  activities: Activity[];
  onChangeDate: (date: Date) => void;
}

export default function ActivityCalendar({
  date,
  activities,
  onChangeDate,
}: Props) {
  const findActivitiesByDate = useCallback(
    (date: Date) => {
      return activities.filter(activity => {
        return dayjs(activity.createdAt).isSame(date, 'day');
      });
    },
    [activities]
  );

  // Navigate to previous month
  const goToPreviousMonth = () => {
    const newMonth = new Date(date);
    newMonth.setMonth(newMonth.getMonth() - 1);
    onChangeDate(newMonth);
  };

  // Navigate to next month
  const goToNextMonth = () => {
    const newMonth = new Date(date);
    newMonth.setMonth(newMonth.getMonth() + 1);
    onChangeDate(newMonth);
  };

  // Generate calendar days
  const generateCalendarDays = () => {
    const year = date.getFullYear();
    const month = date.getMonth();

    // First day of the month
    const firstDay = new Date(year, month, 1);
    // Last day of the month
    const lastDay = new Date(year, month + 1, 0);

    // Start from the beginning of the week containing the first day
    const startDate = new Date(firstDay);
    startDate.setDate(startDate.getDate() - startDate.getDay());

    // End at the end of the week containing the last day
    const endDate = new Date(lastDay);
    endDate.setDate(endDate.getDate() + (6 - endDate.getDay()));

    const days: Date[] = [];
    const currentDate = new Date(startDate);

    while (currentDate <= endDate) {
      days.push(new Date(currentDate));
      currentDate.setDate(currentDate.getDate() + 1);
    }

    return days;
  };

  const isCurrentMonth = (targetDate: Date) => {
    return (
      targetDate.getMonth() === date.getMonth() &&
      targetDate.getFullYear() === date.getFullYear()
    );
  };

  const getWeekDayColor = (day: string) => {
    if (day === 'Sun') return 'sun';
    if (day === 'Sat') return 'sat';
    return 'weekday';
  };

  const getDayColor = (date: Date) => {
    if (!isCurrentMonth(date)) return 'outside';
    const day = date.getDay();
    if (day === 0) return 'sun';
    if (day === 6) return 'sat';
    return 'weekday';
  };

  const calendarDays = generateCalendarDays();

  return (
    <RColumn>
      <RSpaceBetween align='center' gap={2}>
        <RIconButton onClick={goToPreviousMonth}>
          <ChevronLeftIcon />
        </RIconButton>

        <RRow gap={1}>
          <RText>{monthNames[date.getMonth()]}</RText>
          <RText>{date.getFullYear()}</RText>
        </RRow>

        <RIconButton onClick={goToNextMonth}>
          <ChevronRightIcon />
        </RIconButton>
      </RSpaceBetween>

      <RGrid
        columns={['1fr', '1fr', '1fr', '1fr', '1fr', '1fr', '1fr']}
        gap={0.5}
      >
        {weekDays.map(day => (
          <RCenter key={day}>
            <RText align='center' color={getWeekDayColor(day)}>
              {day}
            </RText>
          </RCenter>
        ))}
      </RGrid>

      <RGrid
        columns={['1fr', '1fr', '1fr', '1fr', '1fr', '1fr', '1fr']}
        gap={0.5}
      >
        {calendarDays.map((date, index) => {
          const dateString = dayjs(date).format('YYYY-MM-DD');
          return (
            <RLink href={`/dates/${dateString}`} key={index}>
              <RSquareBox size={80} align='center' justify='center'>
                <RColumn gap={1}>
                  <RText align='center' color={getDayColor(date)}>
                    {date.getDate()}
                  </RText>
                  <ActivityCounts activities={findActivitiesByDate(date)} />
                </RColumn>
              </RSquareBox>
            </RLink>
          );
        })}
      </RGrid>
    </RColumn>
  );
}

import {
  ChevronLeft as ChevronLeftIcon,
  ChevronRight as ChevronRightIcon,
} from '@mui/icons-material';
import { useState } from 'react';
import { RSquareBox } from '../../components/display/box';
import RText from '../../components/display/text';
import RIconButton from '../../components/input/icon-button';
import {
  RCenter,
  RColumn,
  RRow,
  RSpaceBetween,
} from '../../components/layout/flex-box';
import RGrid from '../../components/layout/grid';

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

export default function ActivityCalendar() {
  const [currentMonth, setCurrentMonth] = useState(() => {
    const date = new Date();
    date.setDate(1);
    return date;
  });

  // Navigate to previous month
  const goToPreviousMonth = () => {
    const newMonth = new Date(currentMonth);
    newMonth.setMonth(newMonth.getMonth() - 1);
    setCurrentMonth(newMonth);
  };

  // Navigate to next month
  const goToNextMonth = () => {
    const newMonth = new Date(currentMonth);
    newMonth.setMonth(newMonth.getMonth() + 1);
    setCurrentMonth(newMonth);
  };

  // Generate calendar days
  const generateCalendarDays = () => {
    const year = currentMonth.getFullYear();
    const month = currentMonth.getMonth();

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

  const calendarDays = generateCalendarDays();

  return (
    <RColumn>
      <RSpaceBetween align='center' gap={2}>
        <RIconButton onClick={goToPreviousMonth}>
          <ChevronLeftIcon />
        </RIconButton>

        <RRow gap={1}>
          <RText>{monthNames[currentMonth.getMonth()]}</RText>
          <RText>{currentMonth.getFullYear()}</RText>
        </RRow>

        <RIconButton onClick={goToNextMonth}>
          <ChevronRightIcon />
        </RIconButton>
      </RSpaceBetween>

      <RGrid columnFrs={[1, 1, 1, 1, 1, 1, 1]} gap={1}>
        {weekDays.map(day => (
          <RCenter key={day}>
            <RText align='center'>{day}</RText>
          </RCenter>
        ))}
      </RGrid>

      <RGrid columnFrs={[1, 1, 1, 1, 1, 1, 1]}>
        {calendarDays.map((date, index) => {
          return (
            <RSquareBox size={80} key={index} align='center' justify='center'>
              <RText align='center'>{date.getDate()}</RText>
            </RSquareBox>
          );
        })}
      </RGrid>
    </RColumn>
  );
}

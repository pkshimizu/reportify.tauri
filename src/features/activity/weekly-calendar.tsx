import RBox from '@/components/display/box';
import RText from '@/components/display/text';
import RGitHubIcon from '@/components/icons/github';
import RNextIcon from '@/components/icons/next';
import RPrevIcon from '@/components/icons/prev';
import RButton from '@/components/input/button';
import { RColumn, RRow } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import RGridItem from '@/components/layout/grid-item';
import { useState } from 'react';

// 2つの日付が同じ日かどうかを判定
function isSameDay(date1: Date, date2: Date): boolean {
  return (
    date1.getFullYear() === date2.getFullYear() &&
    date1.getMonth() === date2.getMonth() &&
    date1.getDate() === date2.getDate()
  );
}

// 本日を含む月曜日から始まる1週間の日付配列を取得
function getWeekDates(baseDate: Date): Date[] {
  const date = new Date(baseDate);
  const dayOfWeek = date.getDay(); // 0: 日曜, 1: 月曜, ..., 6: 土曜
  const diff = dayOfWeek === 0 ? -6 : 1 - dayOfWeek; // 月曜日までの差分

  date.setDate(date.getDate() + diff);

  const weekDates: Date[] = [];
  for (let i = 0; i < 7; i++) {
    weekDates.push(new Date(date));
    date.setDate(date.getDate() + 1);
  }

  return weekDates;
}

// 曜日名を取得
function getDayOfWeekName(date: Date): string {
  const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  return days[date.getDay()];
}

function getMonthName(month: number): string {
  const months = [
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
  return months[month - 1];
}

function getMonthBars(days: { month: string; index: number }[]) {
  if (days[0].month === days[days.length - 1].month) {
    return [
      <RGridItem
        key='month-bar'
        area={{ top: 1, left: 2, bottom: 2, right: 9 }}
        justify='center'
        align='center'
        fullWidth
      >
        <RBox bgcolor='primary' fullWidth align='center' justify='center'>
          <RText color='primary'>{days[0].month}</RText>
        </RBox>
      </RGridItem>,
    ];
  }

  const nextMonthIndex = days.findIndex(day => day.month !== days[0].month);

  return [
    <RGridItem
      key='month-bar'
      area={{ top: 1, left: 2, bottom: 2, right: 2 + nextMonthIndex }}
      justify='center'
      align='center'
      fullWidth
    >
      <RBox bgcolor='primary' fullWidth align='center' justify='center'>
        <RText color='primary'>{days[0].month}</RText>
      </RBox>
    </RGridItem>,
    <RGridItem
      key='month-bar'
      area={{ top: 1, left: 2 + nextMonthIndex, bottom: 2, right: 9 }}
      justify='center'
      align='center'
      fullWidth
    >
      <RBox bgcolor='primary' fullWidth align='center' justify='center'>
        <RText color='primary'>{days[nextMonthIndex].month}</RText>
      </RBox>
    </RGridItem>,
  ];
}

export default function ActivityWeeklyCalendar() {
  const today = new Date();
  const [baseDate, setBaseDate] = useState(today);

  const weekDates = getWeekDates(baseDate);
  const days = weekDates.map((date, index) => ({
    dayOfWeek: getDayOfWeekName(date),
    date: date.getDate(),
    month: getMonthName(date.getMonth() + 1),
    github: 999,
    isToday: isSameDay(date, today),
    index: index,
  }));

  const handlePrevWeek = () => {
    setBaseDate(prev => {
      const newDate = new Date(prev);
      newDate.setDate(newDate.getDate() - 7);
      return newDate;
    });
  };

  const handleNextWeek = () => {
    setBaseDate(prev => {
      const newDate = new Date(prev);
      newDate.setDate(newDate.getDate() + 7);
      return newDate;
    });
  };
  return (
    <RGrid
      columns={[
        '64px',
        '1fr',
        '1fr',
        '1fr',
        '1fr',
        '1fr',
        '1fr',
        '1fr',
        '64px',
      ]}
      rowGap={0.5}
      columnGap={1}
    >
      <RGridItem
        area={{ top: 1, left: 1, bottom: 3, right: 2 }}
        justify='center'
        align='center'
        alignContent='center'
        fullHeight
      >
        <RButton fullHeight onClick={handlePrevWeek}>
          <RPrevIcon size='small' />
        </RButton>
      </RGridItem>
      {getMonthBars(days).map(monthBar => monthBar)}
      {days.map((day, index) => {
        const color =
          day.dayOfWeek === 'Sat'
            ? 'sat'
            : day.dayOfWeek === 'Sun'
              ? 'sun'
              : 'weekday';
        const style = day.isToday ? 'bold' : 'normal';
        return (
          <RGridItem
            key={`${weekDates[index].toISOString()}-${day.date}`}
            area={{
              top: 2,
              left: day.index + 2,
              bottom: 3,
              right: day.index + 3,
            }}
          >
            <RBox px={2} py={1} align='center' justify='center' bgcolor={color}>
              <RColumn align='center'>
                <RText color={color} style={style}>
                  {day.dayOfWeek}
                </RText>
                <RText color={color} size='large' style={style}>
                  {day.date}
                </RText>
                <RBox height={64} justify='center' align='center'>
                  <RColumn>
                    <RRow gap={1} align='center'>
                      <RGitHubIcon />
                      <RText>{day.github}</RText>
                    </RRow>
                  </RColumn>
                </RBox>
              </RColumn>
            </RBox>
          </RGridItem>
        );
      })}
      <RGridItem
        area={{ top: 1, left: 9, bottom: 3, right: 10 }}
        justify='center'
        align='center'
        alignContent='center'
        fullHeight
      >
        <RButton fullHeight onClick={handleNextWeek}>
          <RNextIcon />
        </RButton>
      </RGridItem>
    </RGrid>
  );
}

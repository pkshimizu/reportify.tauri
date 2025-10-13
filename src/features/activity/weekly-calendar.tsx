import RBox from '@/components/display/box';
import RText from '@/components/display/text';
import RGitHubIcon from '@/components/icons/github';
import RNextIcon from '@/components/icons/next';
import RPrevIcon from '@/components/icons/prev';
import RButton from '@/components/input/button';
import { RColumn, RRow } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
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

export default function ActivityWeeklyCalendar() {
  const today = new Date();
  const [baseDate, setBaseDate] = useState(today);

  const weekDates = getWeekDates(baseDate);
  const days = weekDates.map(date => ({
    dayOfWeek: getDayOfWeekName(date),
    date: date.getDate(),
    github: 999,
    isToday: isSameDay(date, today),
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
      gap={1}
    >
      <RButton onClick={handlePrevWeek}>
        <RPrevIcon size='small' />
      </RButton>
      {days.map((day, index) => {
        const color =
          day.dayOfWeek === 'Sat'
            ? 'sat'
            : day.dayOfWeek === 'Sun'
              ? 'sun'
              : 'weekday';
        const style = day.isToday ? 'bold' : 'normal';
        return (
          <RBox
            key={`${weekDates[index].toISOString()}-${day.date}`}
            px={2}
            py={1}
            align='center'
            justify='center'
            bgcolor={color}
          >
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
        );
      })}
      <RButton onClick={handleNextWeek}>
        <RNextIcon />
      </RButton>
    </RGrid>
  );
}

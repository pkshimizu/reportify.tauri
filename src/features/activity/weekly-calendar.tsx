import RBox from '@/components/display/box';
import RText from '@/components/display/text';
import RGitHubIcon from '@/components/icons/github';
import RNextIcon from '@/components/icons/next';
import RPrevIcon from '@/components/icons/prev';
import RButton from '@/components/input/button';
import { RColumn, RRow } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';

export default function ActivityWeeklyCalendar() {
  const days = [
    {
      dayOfWeek: 'Mon',
      date: 1,
      github: 999,
    },
    {
      dayOfWeek: 'Tue',
      date: 2,
      github: 999,
    },
    {
      dayOfWeek: 'Wed',
      date: 3,
      github: 999,
    },
    {
      dayOfWeek: 'Thu',
      date: 4,
      github: 999,
    },
    {
      dayOfWeek: 'Fri',
      date: 5,
      github: 999,
    },
    {
      dayOfWeek: 'Sat',
      date: 6,
      github: 999,
    },
    {
      dayOfWeek: 'Sun',
      date: 7,
      github: 999,
    },
  ];
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
      <RButton onClick={() => {}}>
        <RPrevIcon size='small' />
      </RButton>
      {days.map(day => {
        const color =
          day.dayOfWeek === 'Sat'
            ? 'sat'
            : day.dayOfWeek === 'Sun'
              ? 'sun'
              : 'weekday';
        return (
          <RBox
            key={day.date}
            px={2}
            py={1}
            align='center'
            justify='center'
            bgcolor={color}
          >
            <RColumn align='center'>
              <RText color={color}>{day.dayOfWeek}</RText>
              <RText color={color}>{day.date}</RText>
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
      <RButton onClick={() => {}}>
        <RNextIcon />
      </RButton>
    </RGrid>
  );
}

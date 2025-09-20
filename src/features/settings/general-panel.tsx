import RText from '@/components/display/text';
import RButton from '@/components/input/button';
import { RRow } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import RGridItem from '@/components/layout/grid-item';
import { useState } from 'react';

export default function GeneralPanel() {
  const [theme, setTheme] = useState<string>('light');
  return (
    <RGrid columns={['120px', '1fr']} gap={2} alignContent='start'>
      <RGridItem align='center'>
        <RText>Theme</RText>
      </RGridItem>
      <RGridItem align='center'>
        <RRow gap={1}>
          <RButton
            variant='outlined'
            color={theme === 'light' ? 'primary' : undefined}
            onClick={() => setTheme('light')}
          >
            Light
          </RButton>
          <RButton
            variant='outlined'
            color={theme === 'dark' ? 'primary' : undefined}
            onClick={() => setTheme('dark')}
          >
            Dark
          </RButton>
        </RRow>
      </RGridItem>
    </RGrid>
  );
}

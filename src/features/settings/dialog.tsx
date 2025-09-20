import RList from '@/components/display/list';
import RDialog from '@/components/feedback/dialog';
import RGitHubIcon from '@/components/icons/github';
import RSettingIcon from '@/components/icons/setting';
import { RColumn } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import { useState } from 'react';

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function SettingsDialog(props: Props) {
  const [selected, setSelected] = useState<string>('general');
  return (
    <RDialog onClose={props.onClose} open={props.open} size='md'>
      <RGrid columns={['240px', '1fr']} gap={2}>
        <RColumn height={640}>
          <RList
            items={[
              {
                id: 'general',
                text: 'General',
                icon: <RSettingIcon />,
                selected: selected === 'general',
                onClick: () => setSelected('general'),
              },
              {
                id: 'github',
                text: 'GitHub',
                icon: <RGitHubIcon />,
                selected: selected === 'github',
                onClick: () => setSelected('github'),
              },
            ]}
          />
        </RColumn>
      </RGrid>
    </RDialog>
  );
}

import RList from '@/components/display/list';
import RDialog from '@/components/feedback/dialog';
import RGitHubIcon from '@/components/icons/github';
import RSettingIcon from '@/components/icons/setting';
import RButton from '@/components/input/button';
import { RColumn } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import { useState } from 'react';
import GeneralPanel from './general-panel';
import GitHubPanel from './github-panel';

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function SettingsDialog(props: Props) {
  const [selected, setSelected] = useState<string>('general');
  return (
    <RDialog
      onClose={props.onClose}
      open={props.open}
      size='md'
      actions={<RButton onClick={props.onClose}>Close</RButton>}
    >
      <RGrid columns={['240px', '1fr']} gap={2}>
        <RColumn height={480}>
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
        {selected === 'general' && <GeneralPanel />}
        {selected === 'github' && <GitHubPanel />}
      </RGrid>
    </RDialog>
  );
}

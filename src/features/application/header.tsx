import RSettingIcon from '@/components/icons/setting';
import RIconButton from '@/components/input/icon-button';
import { RRow } from '@/components/layout/flex-box';
import RAppBar from '@/components/surface/app-bar';
import { useState } from 'react';
import SettingsDialog from '../settings/dialog';

export default function ApplicationHeader() {
  const [settingsDialog, setSettingsDialog] = useState(false);

  return (
    <RAppBar>
      <RRow justify='flex-end' fullWidth>
        <RIconButton onClick={() => setSettingsDialog(true)}>
          <RSettingIcon />
        </RIconButton>
        <SettingsDialog
          open={settingsDialog}
          onClose={() => setSettingsDialog(false)}
        />
      </RRow>
    </RAppBar>
  );
}

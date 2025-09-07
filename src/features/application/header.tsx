import RSettingIcon from '@/components/icons/setting';
import RIconButton from '@/components/input/icon-button';
import { RRow } from '@/components/layout/flex-box';
import RAppBar from '@/components/surface/app-bar';

export default function ApplicationHeader() {
  return (
    <RAppBar>
      <RRow justify='flex-end' fullWidth>
        <RIconButton>
          <RSettingIcon />
        </RIconButton>
      </RRow>
    </RAppBar>
  );
}

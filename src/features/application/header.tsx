import { useNavigate } from '@tanstack/react-router';
import RText from '../../components/display/text';
import RCalendarIcon from '../../components/icons/calendar';
import RHomeIcon from '../../components/icons/home';
import RSettingIcon from '../../components/icons/setting';
import RIconButton from '../../components/input/icon-button';
import { RRow, RSpaceBetween } from '../../components/layout/flex-box';
import RAppBar from '../../components/surface/app-bar';

export default function ApplicationHeader() {
  const navigate = useNavigate();

  return (
    <RAppBar>
      <RSpaceBetween align='center'>
        <RText>reportify</RText>
        <RRow gap={1}>
          <RIconButton onClick={() => navigate({ to: '/' })}>
            <RHomeIcon />
          </RIconButton>
          <RIconButton onClick={() => navigate({ to: '/calendar' })}>
            <RCalendarIcon />
          </RIconButton>
          <RIconButton onClick={() => navigate({ to: '/settings' })}>
            <RSettingIcon />
          </RIconButton>
        </RRow>
      </RSpaceBetween>
    </RAppBar>
  );
}

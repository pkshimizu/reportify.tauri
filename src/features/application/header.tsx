import { useRouterState } from '@tanstack/react-router';
import { RSquareBox } from '../../components/display/box';
import RText from '../../components/display/text';
import RCalendarIcon from '../../components/icons/calendar';
import RHomeIcon from '../../components/icons/home';
import RSettingIcon from '../../components/icons/setting';
import { RRow, RSpaceBetween } from '../../components/layout/flex-box';
import RLink from '../../components/navigation/link';
import RAppBar from '../../components/surface/app-bar';

export default function ApplicationHeader() {
  const routerState = useRouterState();

  return (
    <RAppBar>
      <RSpaceBetween align='center'>
        <RText>reportify</RText>
        <RRow gap={1}>
          <RLink href='/'>
            <RSquareBox size={40} align='center' justify='center'>
              <RHomeIcon
                color={
                  routerState.location.pathname === '/' ? 'primary' : 'inherit'
                }
              />
            </RSquareBox>
          </RLink>
          <RLink href='/calendar'>
            <RSquareBox size={40} align='center' justify='center'>
              <RCalendarIcon
                color={
                  routerState.location.pathname === '/calendar'
                    ? 'primary'
                    : 'inherit'
                }
              />
            </RSquareBox>
          </RLink>
          <RLink href='/settings'>
            <RSquareBox size={40} align='center' justify='center'>
              <RSettingIcon
                color={
                  routerState.location.pathname === '/settings'
                    ? 'primary'
                    : 'inherit'
                }
              />
            </RSquareBox>
          </RLink>
        </RRow>
      </RSpaceBetween>
    </RAppBar>
  );
}

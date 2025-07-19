import { createRootRoute, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import RText from '../components/display/text';
import RCalendarIcon from '../components/icons/calendar';
import RHomeIcon from '../components/icons/home';
import RSettingIcon from '../components/icons/setting';
import RIconButton from '../components/input/icon-button';
import { RRow, RSpaceBetween } from '../components/layout/flex-box';
import RAppBar from '../components/surface/app-bar';

export const Route = createRootRoute({
  component: () => (
    <>
      <RAppBar>
        <RSpaceBetween align='center'>
          <RText>Reportify</RText>
          <RRow gap={1}>
            <RIconButton>
              <RHomeIcon />
            </RIconButton>
            <RIconButton>
              <RCalendarIcon />
            </RIconButton>
            <RIconButton>
              <RSettingIcon />
            </RIconButton>
          </RRow>
        </RSpaceBetween>
      </RAppBar>
      <Outlet />
      <TanStackRouterDevtools />
    </>
  ),
});

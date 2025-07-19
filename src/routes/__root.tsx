import { createRootRoute, Link, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import RText from '../components/display/text';
import { RRow, RSpaceBetween } from '../components/layout/flex-box';
import RAppBar from '../components/surface/app-bar';

export const Route = createRootRoute({
  component: () => (
    <>
      <RAppBar>
        <RSpaceBetween>
          <RText>Reportify</RText>
          <RRow gap={1}>
            <Link to='/' className='[&.active]:font-bold'>
              Home
            </Link>{' '}
            <Link to='/about' className='[&.active]:font-bold'>
              About
            </Link>
          </RRow>
        </RSpaceBetween>
      </RAppBar>
      <Outlet />
      <TanStackRouterDevtools />
    </>
  ),
});

import RBox from '@/components/display/box';
import ApplicationHeader from '@/features/application/header';
import { createRootRoute, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';

export const Route = createRootRoute({
  component: () => (
    <>
      <ApplicationHeader />
      <RBox mt={4.5}>
        <Outlet />
      </RBox>
      <TanStackRouterDevtools />
    </>
  ),
});

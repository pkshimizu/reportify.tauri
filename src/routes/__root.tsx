import { createRootRoute, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import ApplicationHeader from '../features/application/header';

export const Route = createRootRoute({
  component: () => (
    <>
      <ApplicationHeader />
      <Outlet />
      <TanStackRouterDevtools />
    </>
  ),
});

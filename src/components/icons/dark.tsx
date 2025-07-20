import { DarkMode } from '@mui/icons-material';
import RIcon, { IconProps } from './icon';

export default function RDarkIcon(props: IconProps) {
  return <RIcon {...props} icon={DarkMode} />;
}

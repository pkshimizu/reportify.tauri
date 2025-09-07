import dayjs, { extend } from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';

export default function useDay() {
  extend(relativeTime);
  return {
    fromNow: (date: string) => {
      return dayjs(date).fromNow();
    },
  };
}

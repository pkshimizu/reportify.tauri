import RTable from '@/components/display/table';
import MDialog from '@/components/feedback/dialog';
import MButton from '@/components/input/button';

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function GitHubRepositorySelectDialog(props: Props) {
  return (
    <MDialog onClose={props.onClose} open={props.open} size='sm'>
      <RTable
        columns={[
          { name: 'select', width: 64 },
          { name: 'owner', cell: 'Owner', align: 'left', width: 100 },
          { name: 'repository', cell: 'Repository', align: 'left' },
        ]}
        rows={[
          {
            id: '1',
            cells: {
              select: <MButton onClick={props.onClose}>Select</MButton>,
              owner: 'pkshimizu',
              repository: 'reportify.tauri',
            },
          },
        ]}
      />
    </MDialog>
  );
}

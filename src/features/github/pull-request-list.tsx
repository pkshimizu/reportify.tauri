import RBox from '@/components/display/box';
import RImage from '@/components/display/image';
import RText from '@/components/display/text';
import RCommentIcon from '@/components/icons/comment';
import { RColumn, RRow } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import RLink from '@/components/navigation/link';
import useDay from '@/hooks/day';

export default function GitHubPullRequestList() {
  const day = useDay();

  const pullRequests = [
    {
      id: '1',
      owner: {
        image: 'https://avatars.githubusercontent.com/u/300403?v=4',
        name: 'pkshimizu',
      },
      repository: {
        image: 'https://avatars.githubusercontent.com/u/300403?v=4',
        name: 'reportify.tauri',
      },
      assignee: {
        image: 'https://avatars.githubusercontent.com/u/300403?v=4',
        name: 'pkshimizu',
      },
      reviewers: [
        {
          id: '1',
          image: 'https://avatars.githubusercontent.com/u/300403?v=4',
          name: 'pkshimizu',
          comments: 10,
        },
        {
          id: '2',
          image: 'https://avatars.githubusercontent.com/u/300403?v=4',
          name: 'pkshimizu',
          comments: 0,
        },
      ],
      title: 'Add support for Tauri',
      url: 'https://github.com/pkshimizu/reportify.tauri/pull/1',
      createdAt: day.fromNow('2025-08-01T00:00'),
      updatedAt: day.fromNow('2025-08-02T00:00'),
    },
    {
      id: '1',
      owner: {
        image: 'https://avatars.githubusercontent.com/u/300403?v=4',
        name: 'pkshimizu',
      },
      repository: {
        image: 'https://avatars.githubusercontent.com/u/300403?v=4',
        name: 'reportify.tauri',
      },
      assignee: {
        image: 'https://avatars.githubusercontent.com/u/300403?v=4',
        name: 'pkshimizu',
      },
      reviewers: [
        {
          id: '1',
          image: 'https://avatars.githubusercontent.com/u/300403?v=4',
          name: 'pkshimizu',
          comments: 999,
        },
        {
          id: '2',
          image: 'https://avatars.githubusercontent.com/u/300403?v=4',
          name: 'pkshimizu',
          comments: 0,
        },
      ],
      title:
        'feat: add support for Tauri. fix: fix bugs. WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW',
      url: 'https://github.com/pkshimizu/reportify.tauri/pull/1',
      createdAt: day.fromNow('2025-08-01T00:00'),
      updatedAt: day.fromNow('2025-08-02T00:00'),
    },
  ];

  return (
    <RColumn gap={1}>
      {pullRequests.map(pullRequest => (
        <RColumn key={pullRequest.id}>
          <RColumn>
            <RGrid columns={['1fr', '330px']} gap={1} alignItems='end'>
              <RRow gap={1} align='center'>
                <RImage
                  src={pullRequest.owner.image}
                  alt={pullRequest.owner.name}
                  width={20}
                  height={20}
                  circle
                />
                <RText>{pullRequest.owner.name}</RText>
                <RText>/</RText>
                <RImage
                  src={pullRequest.repository.image}
                  alt={pullRequest.repository.name}
                  width={20}
                  height={20}
                  circle
                />
                <RText>{pullRequest.repository.name}</RText>
              </RRow>
              <RRow gap={2} justify='flex-end'>
                <RRow gap={0.5} align='flex-end'>
                  <RText size='small'>{pullRequest.createdAt}</RText>
                  <RText size='small'>created</RText>
                </RRow>
                <RRow gap={0.5} align='flex-end'>
                  <RText size='small'>{pullRequest.updatedAt}</RText>
                  <RText size='small'>updated</RText>
                </RRow>
              </RRow>
            </RGrid>
            <RBox bgcolor='card' p={1}>
              <RGrid columns={['1fr', '160px', '200px']} gap={1}>
                <RLink href={pullRequest.url} overflow='hidden'>
                  <RRow align='center' overflow='hidden' fullHeight>
                    <RText whiteSpace='nowrap' overflow='hidden'>
                      {pullRequest.title}
                    </RText>
                  </RRow>
                </RLink>
                <RColumn>
                  <RText size='small'>Assignee</RText>
                  <RRow gap={1} align='center'>
                    <RImage
                      src={pullRequest.assignee.image}
                      alt={pullRequest.assignee.name}
                      width={20}
                      height={20}
                      circle
                    />
                    <RText>{pullRequest.assignee.name}</RText>
                  </RRow>
                </RColumn>
                <RColumn>
                  <RText size='small'>Reviewers</RText>
                  {pullRequest.reviewers.map(reviewer => (
                    <RGrid
                      gap={1}
                      key={reviewer.id}
                      columns={['20px', '1fr', '60px']}
                      alignItems='center'
                    >
                      <RImage
                        src={reviewer.image}
                        alt={reviewer.name}
                        width={20}
                        height={20}
                        circle
                      />
                      <RText>{reviewer.name}</RText>
                      <RRow align='center' gap={0.5}>
                        <RCommentIcon size='small' />
                        <RText>{reviewer.comments}</RText>
                      </RRow>
                    </RGrid>
                  ))}
                </RColumn>
              </RGrid>
            </RBox>
          </RColumn>
        </RColumn>
      ))}
    </RColumn>
  );
}

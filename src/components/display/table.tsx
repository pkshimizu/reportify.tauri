import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from '@mui/material';
import { ReactNode } from 'react';

interface Column {
  name: string;
  align?: 'left' | 'center' | 'right';
  width?: number;
}

interface Row {
  id: string;
  cells: { [key: string]: ReactNode };
}

interface Props {
  columns: Column[];
  rows: Row[];
}

function RTableRow({ columns, row }: { columns: Column[]; row: Row }) {
  return (
    <TableRow>
      {columns.map(column => (
        <TableCell key={column.name} width={column.width} align={column.align}>
          {row.cells[column.name]}
        </TableCell>
      ))}
    </TableRow>
  );
}

export default function RTable({ columns, rows }: Props) {
  return (
    <TableContainer>
      <Table size='small'>
        <TableHead>
          {columns.map(column => (
            <TableCell
              key={column.name}
              width={column.width}
              align={column.align}
            >
              {column.name}
            </TableCell>
          ))}
        </TableHead>
        <TableBody>
          {rows.map(row => (
            <RTableRow key={row.id} columns={columns} row={row} />
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}

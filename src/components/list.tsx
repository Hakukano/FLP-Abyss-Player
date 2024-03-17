import { Stack, Table } from "react-bootstrap";
import { ErrorModal, useError } from "./error_modal";
import {
  PlusCircle,
  XCircle,
  ArrowUpCircle,
  ArrowDownCircle,
  SortUp,
} from "react-bootstrap-icons";
import { MetaCmpBy } from "../utils/meta";
import { FormModal, useForm } from "./form_modal";
import { useTranslation } from "react-i18next";
import {
  createColumnHelper,
  useReactTable,
  getCoreRowModel,
  flexRender,
  getPaginationRowModel,
  VisibilityState,
} from "@tanstack/react-table";
import { useState } from "react";

interface Props {
  headers: { [key: string]: string | null };
  data: { [key: string]: any }[];

  handleNew: () => void;
  handleDelete: (id: string) => Promise<void>;
  handleSelect: (id: string) => void;

  handleSort?: (values: { [key: string]: any }) => Promise<void>;
  handleShift?: (id: string, offset: number) => Promise<void>;
}

export default function List(props: Props) {
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>(
    Object.entries(props.headers)
      .filter(([_, v]) => !v)
      .reduce(
        (acc, [k, _]) => {
          acc[k] = false;
          return acc;
        },
        {} as { [key: string]: boolean },
      ),
  );

  const { t } = useTranslation();

  const errorState = useError();
  const sortFormState = useForm();

  const popupSortModal = () => {
    sortFormState.popup({
      header: t("sort.title"),
      rows: [
        {
          name: "by",
          type: "select",
          initial: { value: MetaCmpBy.Default, label: t("sort.by.default") },
          label: t("sort.by.label"),
          options: [
            { value: MetaCmpBy.Default, label: t("sort.by.default") },
            { value: MetaCmpBy.Path, label: t("sort.by.path") },
            { value: MetaCmpBy.CreatedAt, label: t("sort.by.created_at") },
            { value: MetaCmpBy.UpdatedAt, label: t("sort.by.updated_at") },
          ],
        },
        {
          name: "ascend",
          type: "checkbox",
          initial: true,
          label: t("sort.ascend.label"),
        },
      ],
    });
  };

  const columnHelper = createColumnHelper<{ [key: string]: any }>();
  const columns = Object.entries(props.headers)
    .map(([k, v]) => {
      return columnHelper.accessor(k, {
        id: k,
        header: () => v,
        cell: (context) => context.getValue(),
      });
    })
    .concat([
      columnHelper.display({
        id: "actions",
        header: () => (
          <Stack direction="horizontal" gap={1}>
            {props.handleSort && (
              <SortUp
                className="text-warning"
                size={24}
                style={{ cursor: "pointer" }}
                onClick={popupSortModal}
              />
            )}
            <PlusCircle
              className="text-info"
              size={24}
              style={{ cursor: "pointer" }}
              onClick={props.handleNew}
            />
          </Stack>
        ),
        cell: (context) => (
          <Stack direction="horizontal" gap={1}>
            {props.handleShift && (
              <>
                <ArrowUpCircle
                  className="text-warning"
                  size={24}
                  style={{ cursor: "pointer" }}
                  onClick={() =>
                    props.handleShift &&
                    props.handleShift(context.row.getValue("id"), -1)
                  }
                ></ArrowUpCircle>
                <ArrowDownCircle
                  className="text-warning"
                  size={24}
                  style={{ cursor: "pointer" }}
                  onClick={() =>
                    props.handleShift &&
                    props.handleShift(context.row.getValue("id"), 1)
                  }
                ></ArrowDownCircle>
              </>
            )}
            <XCircle
              className="text-danger"
              size={24}
              style={{ cursor: "pointer" }}
              onClick={() => props.handleDelete(context.row.getValue("id"))}
            />
          </Stack>
        ),
      }),
    ]);

  const table = useReactTable({
    columns,
    data: props.data,
    state: {
      columnVisibility,
    },
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
  });

  return (
    <Stack gap={2}>
      <ErrorModal state={errorState} />
      {props.handleSort && (
        <FormModal state={sortFormState} handleSubmit={props.handleSort} />
      )}
      <Table striped bordered hover>
        <thead>
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header, index) => (
                <th
                  key={header.id}
                  style={
                    index === headerGroup.headers.length - 1
                      ? { width: "1px", whiteSpace: "nowrap" }
                      : {}
                  }
                >
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                        header.column.columnDef.header,
                        header.getContext(),
                      )}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.map((row) => (
            <tr key={row.id}>
              {row.getVisibleCells().map((cell, index) =>
                index === row.getVisibleCells().length - 1 ? (
                  <td
                    key={cell.id}
                    style={{ width: "1px", whiteSpace: "nowrap" }}
                  >
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ) : (
                  <td
                    key={cell.id}
                    onClick={() => props.handleSelect(row.getValue("id"))}
                    style={{ cursor: "pointer" }}
                  >
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ),
              )}
            </tr>
          ))}
        </tbody>
      </Table>
    </Stack>
  );
}

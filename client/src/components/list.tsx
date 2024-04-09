import { Button, Col, FormControl, Row, Stack, Table } from "react-bootstrap";
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
  PaginationState,
} from "@tanstack/react-table";
import { useEffect, useState } from "react";
import Select from "react-select";

interface Props {
  headers: { [key: string]: string | null };
  data: { [key: string]: any }[];

  highlightedIds?: Set<string>;

  handleNew: () => void;
  handleDelete: (id: string) => Promise<void>;
  handleSelect: (id: string) => void;

  handleSort?: (values: { [key: string]: any }) => Promise<void>;
  handleShift?: (id: string, offset: number) => Promise<void>;
}

export default function List(props: Props) {
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>(
    Object.entries(props.headers)
      .filter(([, v]) => !v)
      .reduce(
        (acc, [k]) => {
          acc[k] = false;
          return acc;
        },
        {} as { [key: string]: boolean },
      ),
  );
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 20,
  });

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
          <Stack direction="horizontal" gap={1} className="justify-content-end">
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
      pagination,
    },
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onPaginationChange: setPagination,
  });

  useEffect(() => {
    const entryIndex = props.data.findIndex((row) =>
      props.highlightedIds?.has(row["id"]),
    );
    const pageIndex = Math.floor(entryIndex / pagination.pageSize);
    console.log(entryIndex, pageIndex);
    if (entryIndex >= 0) {
      table.setPageIndex(pageIndex);
    }
  }, [props.data]);

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
                    className={
                      props.highlightedIds?.has(row.getValue("id"))
                        ? "bg-success"
                        : ""
                    }
                    style={{
                      cursor: "pointer",
                      whiteSpace: "nowrap",
                      textOverflow: "ellipsis",
                      overflow: "hidden",
                      maxWidth: "1px",
                    }}
                  >
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ),
              )}
            </tr>
          ))}
        </tbody>
      </Table>
      <Row className="align-items-center">
        <Col md={6}>
          <Stack direction="horizontal" gap={2}>
            <Button
              variant="secondary"
              onClick={() => table.firstPage()}
              disabled={!table.getCanPreviousPage()}
            >
              {"<<"}
            </Button>
            <Button
              variant="secondary"
              onClick={() => table.previousPage()}
              disabled={!table.getCanPreviousPage()}
            >
              {"<"}
            </Button>
            <Button
              variant="secondary"
              onClick={() => table.nextPage()}
              disabled={!table.getCanNextPage()}
            >
              {">"}
            </Button>
            <Button
              variant="secondary"
              onClick={() => table.lastPage()}
              disabled={!table.getCanNextPage()}
            >
              {">>"}
            </Button>
            <strong>
              {table.getState().pagination.pageIndex + 1}&nbsp;/&nbsp;
              {(table.getPageCount() || 1).toLocaleString()}
            </strong>
          </Stack>
        </Col>
        <Col md={6}>
          <Stack direction="horizontal" gap={2} className="justify-content-end">
            <span>{t("list.pagination.go_to")}:</span>
            <FormControl
              type="number"
              min="1"
              max={table.getPageCount() || 1}
              defaultValue={table.getState().pagination.pageIndex + 1}
              onChange={(e) => {
                const page = e.target.value ? Number(e.target.value) - 1 : 0;
                table.setPageIndex(page);
              }}
              style={{ width: "80px" }}
            />
            <Select
              options={[20, 30, 40, 50].map((pageSize) => {
                return {
                  value: pageSize,
                  label: pageSize,
                };
              })}
              value={{
                value: table.getState().pagination.pageSize,
                label: table.getState().pagination.pageSize,
              }}
              onChange={(option) => {
                table.setPageSize(Number(option?.value || 0));
              }}
              className="text-dark"
            />
          </Stack>
        </Col>
      </Row>
    </Stack>
  );
}

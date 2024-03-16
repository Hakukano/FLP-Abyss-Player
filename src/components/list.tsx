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

interface Props {
  headers: { [key: string]: string };
  data: { [key: string]: any }[];

  handleNew: () => void;
  handleDelete: (id: string) => Promise<void>;
  handleSelect: (id: string) => void;

  handleSort?: (values: { [key: string]: any }) => Promise<void>;
  handleShift?: (id: string, offset: number) => Promise<void>;
}

export default function List(props: Props) {
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

  return (
    <Stack gap={2}>
      <ErrorModal state={errorState} />
      {props.handleSort && (
        <FormModal state={sortFormState} handleSubmit={props.handleSort} />
      )}
      <Table striped bordered hover>
        <thead>
          <tr>
            {Object.entries(props.headers).map(([k, v]) => {
              return <td key={k}>{v}</td>;
            })}
            <td
              className="text-end"
              style={{ whiteSpace: "nowrap", width: "1px" }}
            >
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
            </td>
          </tr>
        </thead>
        <tbody>
          {props.data.map((row) => {
            return (
              <tr key={row["id"]}>
                {Object.entries(props.headers).map(([k, _]) => {
                  return (
                    <td
                      key={k}
                      onClick={() => props.handleSelect(row["id"])}
                      style={{ cursor: "pointer" }}
                    >
                      {row[k]}
                    </td>
                  );
                })}
                <td
                  className="text-end"
                  style={{ whiteSpace: "nowrap", width: "1px" }}
                >
                  <Stack direction="horizontal" gap={1}>
                    {props.handleShift && (
                      <>
                        <ArrowUpCircle
                          className="text-warning"
                          size={24}
                          style={{ cursor: "pointer" }}
                          onClick={() =>
                            props.handleShift &&
                            props.handleShift(row["id"], -1)
                          }
                        ></ArrowUpCircle>
                        <ArrowDownCircle
                          className="text-warning"
                          size={24}
                          style={{ cursor: "pointer" }}
                          onClick={() =>
                            props.handleShift && props.handleShift(row["id"], 1)
                          }
                        ></ArrowDownCircle>
                      </>
                    )}
                    <XCircle
                      className="text-danger"
                      size={24}
                      style={{ cursor: "pointer" }}
                      onClick={() => props.handleDelete(row["id"])}
                    />
                  </Stack>
                </td>
              </tr>
            );
          })}
        </tbody>
      </Table>
    </Stack>
  );
}

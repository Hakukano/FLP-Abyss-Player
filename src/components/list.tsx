import { Stack, Table } from "react-bootstrap";
import { ErrorModal, useError } from "./error_modal";
import { FormModal, UseForm } from "./form_modal";
import {
  PlusCircle,
  XCircle,
  ArrowUpCircle,
  ArrowDownCircle,
} from "react-bootstrap-icons";

interface Props {
  createFormState: UseForm;
  headers: string[];
  data: { [key: string]: any }[];

  handleNew: () => void;
  handleCreate: (values: { [key: string]: any }) => Promise<void>;
  handleDelete: (id: string) => Promise<void>;
  handleShift?: (id: string, offset: number) => Promise<void>;
}

export default function List(props: Props) {
  const errorState = useError();

  return (
    <Stack gap={2}>
      <ErrorModal state={errorState} />
      <FormModal
        state={props.createFormState}
        handleClose={() => {
          props.createFormState.setShow(false);
        }}
        handleSubmit={props.handleCreate}
      />
      <Table striped bordered hover>
        <thead>
          <tr>
            {props.headers.map((header) => {
              return <td key={btoa(header)}>{header}</td>;
            })}
            <td
              className="text-end"
              style={{ whiteSpace: "nowrap", width: "1px" }}
            >
              <PlusCircle
                className="text-info"
                size={24}
                style={{ cursor: "pointer" }}
                onClick={props.handleNew}
              />
            </td>
          </tr>
        </thead>
        <tbody>
          {props.data.map((row) => {
            return (
              <tr key={row["id"]}>
                <td>{row["name"]}</td>
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

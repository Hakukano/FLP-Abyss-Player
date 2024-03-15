import { Formik } from "formik";
import { Dispatch, SetStateAction, useState } from "react";
import { Button, Form, Modal } from "react-bootstrap";
import { useTranslation } from "react-i18next";
import { ErrorModal, useError } from "./error_modal";

interface FormRow<T> {
  name: string;
  type: string;
  initial: T;
  validator: (value: T) => string | null;
  label: string;
  placeholder?: string;
}

type FormData = {
  header: string;
  rows: FormRow<any>[];
};

export interface UseForm {
  show: boolean;
  setShow: Dispatch<SetStateAction<boolean>>;
  data: FormData | null;
  setData: Dispatch<SetStateAction<FormData | null>>;
  popup: (form_data: FormData) => void;
}

export function useForm(): UseForm {
  const [show, setShow] = useState(false);
  const [data, setData] = useState<FormData | null>(null);

  const popup = (form_data: FormData) => {
    setData(form_data);
    setShow(true);
  };

  return {
    show,
    setShow,
    data,
    setData,
    popup,
  };
}

interface Props {
  state: UseForm;
  handleClose: () => void;
  handleSubmit: (values: { [key: string]: any }) => Promise<any>;
}

export function FormModal(props: Props) {
  const formData = props.state.data;
  if (!formData) {
    return <></>;
  }

  const { t } = useTranslation();

  const errorState = useError();

  const initialValues = formData.rows.reduce(
    (acc, cur) => {
      acc[cur.name] = cur.initial;
      return acc;
    },
    {} as { [key: string]: any },
  );
  const validate = (values: { [key: string]: any }) => {
    const errors = {} as { [key: string]: string };
    Object.entries(values).forEach(([name, value]) => {
      const validator = formData.rows.find(
        (row) => row.name === name,
      )?.validator;
      if (!validator) {
        return;
      }
      const error = validator(value);
      if (!error) {
        return;
      }
      errors[name] = error;
    });
    return errors;
  };

  return (
    <>
      <ErrorModal state={errorState} />
      <Modal
        show={props.state.show}
        onHide={props.handleClose}
        backdrop="static"
      >
        <Modal.Header closeButton>
          <Modal.Title>{formData.header}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Formik
            initialValues={initialValues}
            validate={validate}
            onSubmit={(values, { setSubmitting }) => {
              props
                .handleSubmit(values)
                .then(() => {
                  props.handleClose();
                })
                .catch((err) => {
                  errorState.handleError(err);
                })
                .finally(() => {
                  setSubmitting(false);
                });
            }}
          >
            {({
              values,
              errors,
              touched,
              handleChange,
              handleBlur,
              handleSubmit,
              isSubmitting,
            }) => (
              <Form onSubmit={handleSubmit}>
                {formData.rows.map((row) => {
                  return (
                    <Form.Group
                      className="mb-3"
                      controlId={`form-${row.name}`}
                      key={row.name}
                    >
                      <Form.Label>{row.label}</Form.Label>
                      <Form.Control
                        type={row.type}
                        name={row.name}
                        placeholder={row.placeholder}
                        onChange={handleChange}
                        onBlur={handleBlur}
                        value={values[row.name]}
                      />
                      {errors[row.name] && touched[row.name] && (
                        <Form.Text className="text-danger">
                          {errors[row.name]?.toString()}
                        </Form.Text>
                      )}
                    </Form.Group>
                  );
                })}
                <Button
                  type="submit"
                  disabled={isSubmitting}
                  className="float-end"
                >
                  {t("form.submit")}
                </Button>
              </Form>
            )}
          </Formik>
        </Modal.Body>
      </Modal>
    </>
  );
}

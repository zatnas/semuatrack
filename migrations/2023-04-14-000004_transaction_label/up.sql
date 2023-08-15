-- Your SQL goes here
CREATE TABLE transaction_label (
    transaction_id int8 NOT NULL,
    label_id int8 NOT NULL,
    CONSTRAINT transaction_label_pk PRIMARY KEY (transaction_id, label_id),
    CONSTRAINT transaction_fk FOREIGN KEY (transaction_id) REFERENCES public.transaction (id),
    CONSTRAINT label_fk FOREIGN KEY (label_id) REFERENCES public.label (id)
)

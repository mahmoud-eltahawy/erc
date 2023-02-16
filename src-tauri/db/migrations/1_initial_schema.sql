CREATE TABLE IF NOT EXISTS cud_version(
       version_number              BIGINT                      PRIMARY KEY,
       target_id                   VARCHAR(80)                 NOT NULL,
       other_target_id             VARCHAR(80),
       target_table                SMALLINT                    NOT NULL,
       cud                         SMALLINT                    NOT NULL
);

CREATE TABLE IF NOT EXISTS department(
       id                 VARCHAR(80)                          PRIMARY KEY,
       boss_id            VARCHAR(80),
       department_id      VARCHAR(80),
       name               VARCHAR(20)                          NOT NULL
);

CREATE TABLE IF NOT EXISTS employee(
       id                 VARCHAR(80)                          PRIMARY KEY,
       department_id      VARCHAR(80)                          NOT NULL,
       position           VARCHAR(12)                          NOT NULL,
       first_name         VARCHAR(40)                          NOT NULL,
       middle_name        VARCHAR(40)                          NOT NULL,
       last_name          VARCHAR(40)                          NOT NULL,
       card_id            SMALLINT                             NOT NULL,
       password           TEXT                                 NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_of_employee_card_id ON employee(card_id);

CREATE TABLE IF NOT EXISTS machine(
       id                   VARCHAR(80)                        PRIMARY KEY,
       name                 VARCHAR(100)                       NOT NULL
);

CREATE TABLE IF NOT EXISTS spare_part(
       id                   VARCHAR(80)                        PRIMARY KEY,
       name                 VARCHAR(100)                       NOT NULL
);

CREATE TABLE IF NOT EXISTS problem(
       id                         VARCHAR(80)                         PRIMARY KEY,
       writer_id                  VARCHAR(80)                         NOT NULL,
       department_id              VARCHAR(80)                         NOT NULL,
       title                      VARCHAR(70)                         NOT NULL,
       description                VARCHAR(350)                        NOT NULL
);

CREATE TABLE IF NOT EXISTS shift (
       id                   VARCHAR(80)                               PRIMARY KEY,
       shift_order          SMALLINT                                  NOT NULL,
       shift_date           DATE                                      NOT NULL
);

CREATE TABLE IF NOT EXISTS shift_problem(
       id                   VARCHAR(80)                               PRIMARY KEY,
       shift_id             VARCHAR(80)                               NOT NULL,
       writer_id            VARCHAR(80)                               NOT NULL,
       maintainer_id        VARCHAR(80)                               NOT NULL,
       machine_id           VARCHAR(80)                               NOT NULL,
       begin_time           TIME                                      NOT NULL,
       end_time             TIME                                      NOT NULL
);

CREATE TABLE IF NOT EXISTS note(
       id                   VARCHAR(80)                               PRIMARY KEY,
       shift_id             VARCHAR(80),
       shift_problem_id     VARCHAR(80),
       content              varchar(500)                              NOT NULL
);

CREATE TABLE IF NOT EXISTS shift_problem_problem(
       shift_problem_id     VARCHAR(80)                               NOT NULL,
       problem_id           VARCHAR(80)                               NOT NULL
);

CREATE TABLE IF NOT EXISTS shift_problem_spare_part(
       shift_problem_id     VARCHAR(80)                               NOT NULL,
       spare_part_id        VARCHAR(80)                               NOT NULL
);

INSERT INTO cud_version(version_number,target_id,other_target_id,target_table,cud)
SELECT 1,'00000000-0000-0000-0000-000000000000',
'00000000-0000-0000-0000-000000000000',0,0
WHERE (SELECT count(version_number) from cud_version) = 0;

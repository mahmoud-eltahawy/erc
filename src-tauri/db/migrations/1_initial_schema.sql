CREATE TABLE IF NOT EXISTS cud_version(
       version_number              BIGINT                             NOT NULL        UNIQUE,
       target_id                   VARCHAR(80)                        NOT NULL,
       other_target_id             VARCHAR(80),
       target_table                SMALLINT                           NOT NULL,
       cud                         SMALLINT                           NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cv_vn   ON cud_version (version_number);
CREATE INDEX IF NOT EXISTS idx_cv_tid  ON cud_version (target_id);
CREATE INDEX IF NOT EXISTS idx_cv_otid ON cud_version (other_target_id);

CREATE TABLE IF NOT EXISTS department(
       id                 VARCHAR(80)                                 NOT NULL        UNIQUE,
       boss_id            VARCHAR(80),
       name               VARCHAR(20)                                 NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_dep_id      ON department (id);
CREATE INDEX IF NOT EXISTS idx_dep_bid     ON department (boss_id);

CREATE TABLE IF NOT EXISTS employee(
       id                 VARCHAR(80)                                 NOT NULL        UNIQUE,
       department_id      VARCHAR(80)                                 NOT NULL,
       position           VARCHAR(12)                                 NOT NULL,
       first_name         VARCHAR(40)                                 NOT NULL,
       middle_name        VARCHAR(40)                                 NOT NULL,
       last_name          VARCHAR(40)                                 NOT NULL,
       card_id            SMALLINT                                    NOT NULL,
       password           TEXT                                        NOT NULL
);

CREATE TABLE IF NOT EXISTS permissions(
       id                                                      VARCHAR(80)        NOT NULL UNIQUE,
       write_department_problem                                BOOL               NOT NULL,
       read_department_problems                                BOOL               NOT NULL,
       modify_department_problems                              BOOL               NOT NULL,
       define_problem                                          BOOL               NOT NULL,
       access_history_department_problems                      BOOL               NOT NULL,
       access_history_all_departments_problems                 BOOL               NOT NULL,
       access_history_department_department_problems           BOOL               NOT NULL,
       access_history_all_departments_department_problems      BOOL               NOT NULL,
       access_history_machines                                 BOOL               NOT NULL,
       access_history_spare_parts                              BOOL               NOT NULL,
       access_history_employees                                BOOL               NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_emp_id        ON employee(id);
CREATE INDEX IF NOT EXISTS idx_emp_cid       ON employee(card_id);
CREATE INDEX IF NOT EXISTS idx_emp_depid     ON employee(department_id);

CREATE TABLE IF NOT EXISTS machine(
       id                   VARCHAR(80)                               NOT NULL        UNIQUE,
       name                 VARCHAR(100)                              NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_mac_id        ON machine(id);

CREATE TABLE IF NOT EXISTS spare_part(
       id                   VARCHAR(80)                               NOT NULL        UNIQUE,
       name                 VARCHAR(100)                              NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sp_id         ON spare_part(id);

CREATE TABLE IF NOT EXISTS problem(
       id                         VARCHAR(80)                         NOT NULL        UNIQUE,
       writer_id                  VARCHAR(80)                         NOT NULL,
       department_id              VARCHAR(80)                         NOT NULL,
       title                      VARCHAR(70)                         NOT NULL,
       description                VARCHAR(500)                        NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_pro_id            ON problem(id);
CREATE INDEX IF NOT EXISTS idx_pro_wid           ON problem(writer_id);
CREATE INDEX IF NOT EXISTS idx_pro_depid         ON problem(department_id);
CREATE INDEX IF NOT EXISTS idx_pro_tid           ON problem(title);

CREATE TABLE IF NOT EXISTS shift (
       id                   VARCHAR(80)                               NOT NULL        UNIQUE,
       shift_order          VARCHAR(10)                               NOT NULL,
       shift_date           VARCHAR(50)                               NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sft_id            ON shift(id);
CREATE INDEX IF NOT EXISTS idx_sft_so            ON shift(shift_order);
CREATE INDEX IF NOT EXISTS idx_sft_sd            ON shift(shift_date);

CREATE TABLE IF NOT EXISTS department_shift (
       id                  VARCHAR(80)                               NOT NULL        UNIQUE,
       shift_id            VARCHAR(80)                               NOT NULL,
       department_id       VARCHAR(80)                               NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_dep_sft_id       ON department_shift(id);
CREATE INDEX IF NOT EXISTS idx_dep_sft_sft_id   ON department_shift(shift_id);
CREATE INDEX IF NOT EXISTS idx_dep_sft_dep_id   ON department_shift(department_id);

CREATE TABLE IF NOT EXISTS department_shift_employee (
       department_shift_id  UUID              NOT NULL,
       employee_id          UUID              NOT NULL,
       UNIQUE(department_shift_id,employee_id)
);

CREATE TABLE IF NOT EXISTS shift_problem(
       id                   VARCHAR(80)                               NOT NULL        UNIQUE,
       shift_id             VARCHAR(80)                               NOT NULL,
       writer_id            VARCHAR(80)                               NOT NULL,
       maintainer_id        VARCHAR(80)                               NOT NULL,
       machine_id           VARCHAR(80)                               NOT NULL,
       begin_time           VARCHAR(30)                               NOT NULL,
       end_time             VARCHAR(30)                               NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sftpro_id             ON shift_problem(id);
CREATE INDEX IF NOT EXISTS idx_sftpro_sid            ON shift_problem(shift_id);
CREATE INDEX IF NOT EXISTS idx_sftpro_wid            ON shift_problem(writer_id);
CREATE INDEX IF NOT EXISTS idx_sftpro_mid            ON shift_problem(maintainer_id);
CREATE INDEX IF NOT EXISTS idx_sftpro_mac            ON shift_problem(machine_id);
CREATE INDEX IF NOT EXISTS idx_sftpro_bt             ON shift_problem(begin_time);
CREATE INDEX IF NOT EXISTS idx_sftpro_et             ON shift_problem(end_time);

CREATE TABLE IF NOT EXISTS note(
       id                   VARCHAR(80)                               NOT NULL        UNIQUE,
       shift_id             VARCHAR(80),
       shift_problem_id     VARCHAR(80),
       content              varchar(500)                              NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_nte_id                  ON note(id);
CREATE INDEX IF NOT EXISTS idx_nte_spid                ON note(shift_id);
CREATE INDEX IF NOT EXISTS idx_nte_id                  ON note(shift_problem_id);

CREATE TABLE IF NOT EXISTS shift_problem_problem(
       shift_problem_id     VARCHAR(80)                               NOT NULL,
       problem_id           VARCHAR(80)                               NOT NULL,
       UNIQUE(shift_problem_id,problem_id)
);

CREATE INDEX IF NOT EXISTS idx_spp_spid                  ON shift_problem_problem(shift_problem_id);
CREATE INDEX IF NOT EXISTS idx_spp_pid                   ON shift_problem_problem(problem_id);

CREATE TABLE IF NOT EXISTS shift_problem_spare_part(
       shift_problem_id     VARCHAR(80)                               NOT NULL,
       spare_part_id        VARCHAR(80)                               NOT NULL,
       UNIQUE(shift_problem_id,spare_part_id)
);

CREATE INDEX IF NOT EXISTS idx_spsp_spid                  ON shift_problem_spare_part(shift_problem_id);
CREATE INDEX IF NOT EXISTS idx_spsp_pid                   ON shift_problem_spare_part(spare_part_id);

INSERT INTO cud_version(version_number,target_id,other_target_id,target_table,cud)
SELECT 1,'00000000-0000-0000-0000-000000000000',
'00000000-0000-0000-0000-000000000000',0,0
WHERE (SELECT count(version_number) from cud_version) = 0;

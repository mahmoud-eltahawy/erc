CREATE TABLE IF NOT EXISTS key_value (
       the_key                     VARCHAR(30)                        NOT NULL      UNIQUE,
       the_value                   NUMERIC                            NOT NULL
);

INSERT INTO key_value(the_key,the_value) VALUES('last_cd_version',0);
INSERT INTO key_value(the_key,the_value) VALUES('last_update_version',0);

CREATE TABLE IF NOT EXISTS department(
       id                         VARCHAR(80)                         NOT NULL        UNIQUE,
       boss_id                    VARCHAR(80),
       name                       VARCHAR(20)                         NOT NULL        UNIQUE,
       updater_id                 VARCHAR(80)                         NOT NULL,
       time_stamp                 VARCHAR(50)                         NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_dep_id      ON department (id);
CREATE INDEX IF NOT EXISTS idx_dep_bid     ON department (boss_id);

CREATE TABLE IF NOT EXISTS employee(
       id                          VARCHAR(80)                         NOT NULL        UNIQUE,
       card_id                     SMALLINT                            NOT NULL        UNIQUE,
       department_id               VARCHAR(80)                         NOT NULL,
       position                    VARCHAR(12)                         NOT NULL,
       first_name                  VARCHAR(40)                         NOT NULL,
       middle_name                 VARCHAR(40)                         NOT NULL,
       last_name                   VARCHAR(40)                         NOT NULL,
       password                    TEXT                                NOT NULL,
       updater_id                  VARCHAR(80)                         NOT NULL,
       time_stamp                  VARCHAR(50)                         NOT NULL,
       FOREIGN KEY(department_id)  REFERENCES department(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_emp_id        ON employee(id);
CREATE INDEX IF NOT EXISTS idx_emp_cid       ON employee(card_id);
CREATE INDEX IF NOT EXISTS idx_emp_depid     ON employee(department_id);
CREATE INDEX IF NOT EXISTS idx_emp_fn        ON employee(first_name);
CREATE INDEX IF NOT EXISTS idx_emp_mn        ON employee(middle_name);
CREATE INDEX IF NOT EXISTS idx_emp_ln        ON employee(last_name);

CREATE TABLE IF NOT EXISTS permissions(
       employee_id                 VARCHAR(80)                        NOT NULL,
       permission                  VARCHAR(50)                        NOT NULL,
       updater_id                  VARCHAR(80)                        NOT NULL,
       time_stamp                  VARCHAR(50)                        NOT NULL,
       CONSTRAINT unique_employee_permission UNIQUE(employee_id,permission),
       FOREIGN KEY(employee_id) REFERENCES employee(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS machine(
       id                         VARCHAR(80)                         NOT NULL        UNIQUE,
       name                       VARCHAR(100)                        NOT NULL        UNIQUE,
       updater_id                 VARCHAR(80)                         NOT NULL,
       time_stamp                 VARCHAR(50)                         NOT NULL

);

CREATE INDEX IF NOT EXISTS idx_mac_id        ON machine(id);
CREATE INDEX IF NOT EXISTS idx_mac_name      ON machine(name);

CREATE TABLE IF NOT EXISTS spare_part(
       id                         VARCHAR(80)                         NOT NULL        UNIQUE,
       name                       VARCHAR(100)                        NOT NULL        UNIQUE,
       updater_id                 VARCHAR(80)                         NOT NULL,
       time_stamp                 VARCHAR(50)                         NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sp_id         ON spare_part(id);
CREATE INDEX IF NOT EXISTS idx_sp_name       ON spare_part(name);

CREATE TABLE IF NOT EXISTS problem(
       id                         VARCHAR(80)                         NOT NULL        UNIQUE,
       department_id              VARCHAR(80)                         NOT NULL,
       title                      VARCHAR(70)                         NOT NULL,
       description                VARCHAR(500)                        NOT NULL,
       updater_id                 VARCHAR(80)                         NOT NULL,
       time_stamp                 VARCHAR(50)                         NOT NULL,
       CONSTRAINT unique_department_problem UNIQUE(title,department_id),
       FOREIGN KEY(department_id) REFERENCES department(id)    ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_pro_id            ON problem(id);
CREATE INDEX IF NOT EXISTS idx_pro_depid         ON problem(department_id);
CREATE INDEX IF NOT EXISTS idx_pro_tid           ON problem(title);

CREATE TABLE IF NOT EXISTS shift (
       id                          VARCHAR(80)                        NOT NULL        UNIQUE,
       shift_order                 VARCHAR(10)                        NOT NULL,
       shift_date                  VARCHAR(50)                        NOT NULL,
       updater_id                  VARCHAR(80)                        NOT NULL,
       time_stamp                  VARCHAR(50)                        NOT NULL,
       CONSTRAINT unique_shift_identity UNIQUE(shift_order,shift_date)
);

CREATE INDEX IF NOT EXISTS idx_sft_id            ON shift(id);
CREATE INDEX IF NOT EXISTS idx_sft_so            ON shift(shift_order);
CREATE INDEX IF NOT EXISTS idx_sft_sd            ON shift(shift_date);

CREATE TABLE IF NOT EXISTS department_shift (
       id                          VARCHAR(80)                        NOT NULL        UNIQUE,
       shift_id                    VARCHAR(80)                        NOT NULL,
       department_id               VARCHAR(80)                        NOT NULL,
       updater_id                  VARCHAR(80)                        NOT NULL,
       time_stamp                  VARCHAR(50)                        NOT NULL,
       CONSTRAINT unique_department_shift_id UNIQUE(department_id,shift_id),
       FOREIGN KEY(department_id) REFERENCES department(id) ON DELETE CASCADE,
       FOREIGN KEY(shift_id)      REFERENCES shift(id)      ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_dep_sft_id       ON department_shift(id);
CREATE INDEX IF NOT EXISTS idx_dep_sft_sft_id   ON department_shift(shift_id);
CREATE INDEX IF NOT EXISTS idx_dep_sft_dep_id   ON department_shift(department_id);

CREATE TABLE IF NOT EXISTS shift_note(
       id                  VARCHAR(80)           NOT NULL UNIQUE,
       shift_id            VARCHAR(80)           NOT NULL,
       content             VARCHAR(500)          NOT NULL,
       updater_id          VARCHAR(80)           NOT NULL,
       time_stamp          VARCHAR(50)           NOT NULL,
       FOREIGN KEY(shift_id) REFERENCES department_shift(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sh_nte_id            ON shift_note(id);

CREATE TABLE IF NOT EXISTS department_shift_employee (
       department_shift_id  UUID                        NOT NULL,
       employee_id          UUID                        NOT NULL,
       updater_id           VARCHAR(80)                 NOT NULL,
       time_stamp           VARCHAR(50)                 NOT NULL,
       UNIQUE(department_shift_id,employee_id),
       FOREIGN KEY(employee_id) REFERENCES employee(id) ON DELETE CASCADE,
       FOREIGN KEY(department_shift_id) REFERENCES department_shift(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift_problem(
       id                   VARCHAR(80)                               NOT NULL        UNIQUE,
       shift_id             VARCHAR(80)                               NOT NULL,
       maintainer_id        VARCHAR(80)                               NOT NULL,
       machine_id           VARCHAR(80)                               NOT NULL,
       begin_time           VARCHAR(30)                               NOT NULL,
       end_time             VARCHAR(30)                               NOT NULL,
       updater_id           VARCHAR(80)                               NOT NULL,
       time_stamp           VARCHAR(50)                               NOT NULL,
       FOREIGN             KEY(maintainer_id)   REFERENCES employee(id)          ON DELETE CASCADE,
       FOREIGN             KEY(machine_id)      REFERENCES machine(id)           ON DELETE CASCADE,
       FOREIGN             KEY(shift_id)        REFERENCES department_shift(id)  ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sftpro_id             ON shift_problem(id);
CREATE INDEX IF NOT EXISTS idx_sftpro_sid            ON shift_problem(shift_id);
CREATE INDEX IF NOT EXISTS idx_sftpro_mid            ON shift_problem(maintainer_id);
CREATE INDEX IF NOT EXISTS idx_sftpro_mac            ON shift_problem(machine_id);
CREATE INDEX IF NOT EXISTS idx_sftpro_bt             ON shift_problem(begin_time);
CREATE INDEX IF NOT EXISTS idx_sftpro_et             ON shift_problem(end_time);

CREATE TABLE IF NOT EXISTS shift_problem_note(
       id                  VARCHAR(80)                                NOT NULL    UNIQUE,
       content             VARCHAR(500)                               NOT NULL,
       updater_id          VARCHAR(80)                                NOT NULL,
       time_stamp          VARCHAR(50)                                NOT NULL,
       FOREIGN KEY(id) REFERENCES shift_problem(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sp_nte_id            ON shift_problem_note(id);

CREATE TABLE IF NOT EXISTS shift_problem_problem(
       shift_problem_id    VARCHAR(80)                               NOT NULL,
       problem_id          VARCHAR(80)                               NOT NULL,
       updater_id          VARCHAR(80)                               NOT NULL,
       time_stamp          VARCHAR(50)                               NOT NULL,
       UNIQUE(shift_problem_id,problem_id),
       FOREIGN KEY(problem_id) REFERENCES problem(id) ON DELETE CASCADE,
       FOREIGN KEY(shift_problem_id) REFERENCES shift_problem(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_spp_spid               ON shift_problem_problem(shift_problem_id);
CREATE INDEX IF NOT EXISTS idx_spp_pid                ON shift_problem_problem(problem_id);

CREATE TABLE IF NOT EXISTS shift_problem_spare_part(
       shift_problem_id     VARCHAR(80)                               NOT NULL,
       spare_part_id        VARCHAR(80)                               NOT NULL,
       updater_id           VARCHAR(80)                               NOT NULL,
       time_stamp           VARCHAR(50)                               NOT NULL,
       UNIQUE(shift_problem_id,spare_part_id),
       FOREIGN KEY(spare_part_id) REFERENCES spare_part(id) ON DELETE CASCADE,
       FOREIGN KEY(shift_problem_id) REFERENCES shift_problem(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_spsp_spid              ON shift_problem_spare_part(shift_problem_id);
CREATE INDEX IF NOT EXISTS idx_spsp_pid               ON shift_problem_spare_part(spare_part_id);

use color_eyre::eyre::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::db_access::models::{Battery, BatteryIntake, BatteryType, Sample, Test, TestSession};

#[derive(Clone)]
pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn list_battery_types(&self) -> Result<Vec<BatteryType>> {
        let rows = sqlx::query_as!(
            BatteryType,
            r#"
            SELECT
                id,
                manufacturer,
                model,
                chemistry,
                nominal_voltage_mv,
                nominal_capacity_mah,
                charge_termination_voltage_mv,
                discharge_cutoff_voltage_mv,
                notes
            FROM battery_types
            ORDER BY manufacturer, model
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_battery_type(
        &self,
        manufacturer: &str,
        model: &str,
        chemistry: &str,
        nominal_voltage_mv: i64,
        nominal_capacity_mah: i64,
        charge_termination_voltage_mv: i64,
        discharge_cutoff_voltage_mv: i64,
    ) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO battery_types (
                manufacturer,
                model,
                chemistry,
                nominal_voltage_mv,
                nominal_capacity_mah,
                charge_termination_voltage_mv,
                discharge_cutoff_voltage_mv
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            manufacturer,
            model,
            chemistry,
            nominal_voltage_mv,
            nominal_capacity_mah,
            charge_termination_voltage_mv,
            discharge_cutoff_voltage_mv,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn list_batteries(&self) -> Result<Vec<Battery>> {
        let result = sqlx::query_as!(
            Battery,
            r#"
            SELECT
                battery_id as "battery_id!",
                battery_type_id as "battery_type_id!",
                notes
            FROM batteries
            ORDER BY battery_id
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn get_battery(&self, battery_id: &str) -> Result<Option<Battery>> {
        let result = sqlx::query_as!(
            Battery,
            r#"
            SELECT
                battery_id as "battery_id!",
                battery_type_id as "battery_type_id!",
                notes
            FROM batteries
            WHERE battery_id = ?
            "#,
            battery_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn create_battery(&self, battery_id: &str, battery_type_id: i64) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO batteries (
                battery_id,
                battery_type_id
            )
            VALUES (?, ?)
            "#,
            battery_id,
            battery_type_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_battery_intake(&self, battery_id: &str) -> Result<Option<BatteryIntake>> {
        let result = sqlx::query_as!(
            BatteryIntake,
            r#"
            SELECT
                battery_id as "battery_id!",
                serial_number,
                purchase_date,
                delivery_date,
                voltage_at_delivery_mv,
                internal_resistance_at_delivery_uohm,
                visual_inspection,
                notes
            FROM battery_intake
            WHERE battery_id = ?
            "#,
            battery_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn upsert_battery_intake(&self, intake: &BatteryIntake) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO battery_intake (
                battery_id,
                serial_number,
                purchase_date,
                delivery_date,
                voltage_at_delivery_mv,
                internal_resistance_at_delivery_uohm,
                visual_inspection,
                notes
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)

            ON CONFLICT(battery_id)
            DO UPDATE SET
                serial_number = excluded.serial_number,
                purchase_date = excluded.purchase_date,
                delivery_date = excluded.delivery_date,
                voltage_at_delivery_mv = excluded.voltage_at_delivery_mv,
                internal_resistance_at_delivery_uohm = excluded.internal_resistance_at_delivery_uohm,
                visual_inspection = excluded.visual_inspection,
                notes = excluded.notes
            "#,
            intake.battery_id,
            intake.serial_number,
            intake.purchase_date,
            intake.delivery_date,
            intake.voltage_at_delivery_mv,
            intake.internal_resistance_at_delivery_uohm,
            intake.visual_inspection,
            intake.notes,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn create_test(&self, test: &Test) -> Result<i64> {
        let result = sqlx::query!(
            r#"
        INSERT INTO tests (
            battery_id,
            approved,
            device_id,
            mode,
            voltage_before_test_mv,
            target_current_ma,
            target_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
            charge_voltage_mv,
            charge_cutoff_current_ma,
            measured_capacity_mah,
            measured_energy_mwh,
            end_voltage_mv,
            notes
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
            test.battery_id,
            test.approved,
            test.device_id,
            test.mode,
            test.voltage_before_test_mv,
            test.target_current_ma,
            test.target_power_w,
            test.cutoff_voltage_mv,
            test.cutoff_time_min,
            test.charge_voltage_mv,
            test.charge_cutoff_current_ma,
            test.measured_capacity_mah,
            test.measured_energy_mwh,
            test.end_voltage_mv,
            test.notes,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn list_tests(&self) -> Result<Vec<Test>> {
        let tests = sqlx::query_as!(
            Test,
            r#"
        SELECT
            id as "id!",
            battery_id as "battery_id!",
            approved as "approved!: bool",
            device_id as "device_id!",
            mode as "mode!",
            voltage_before_test_mv,
            target_current_ma,
            target_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
            charge_voltage_mv,
            charge_cutoff_current_ma,
            measured_capacity_mah,
            measured_energy_mwh,
            end_voltage_mv,
            notes
        FROM tests
        ORDER BY id DESC
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tests)
    }

    pub async fn get_test(&self, id: i64) -> Result<Option<Test>> {
        let test = sqlx::query_as!(
            Test,
            r#"
        SELECT
            id as "id!",
            battery_id as "battery_id!",
            approved as "approved!: bool",
            device_id as "device_id!",
            mode as "mode!",
            voltage_before_test_mv,
            target_current_ma,
            target_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
            charge_voltage_mv,
            charge_cutoff_current_ma,
            measured_capacity_mah,
            measured_energy_mwh,
            end_voltage_mv,
            notes
        FROM tests
        WHERE id = ?
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(test)
    }

    pub async fn approve_test(&self, id: i64) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let test = sqlx::query!(
            r#"
        SELECT battery_id as "battery_id!"
        FROM tests
        WHERE id = ?
        "#,
            id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| color_eyre::eyre::eyre!("Test not found: {id}"))?;

        sqlx::query!(
            r#"
        UPDATE tests
        SET approved = 0
        WHERE battery_id = ?
        "#,
            test.battery_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
        UPDATE tests
        SET approved = 1
        WHERE id = ?
        "#,
            id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn delete_test(&self, id: i64) -> Result<()> {
        sqlx::query!(
            r#"
        DELETE FROM tests
        WHERE id = ?
        "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_test_session(&self, test_id: i64, reason: Option<&str>) -> Result<i64> {
        let started_at = chrono::Utc::now().to_rfc3339();

        let result = sqlx::query!(
            r#"
        INSERT INTO test_sessions (
            test_id,
            started_at,
            reason
        )
        VALUES (?, ?, ?)
        "#,
            test_id,
            started_at,
            reason,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn end_test_session(&self, session_id: i64) -> Result<()> {
        let ended_at = chrono::Utc::now().to_rfc3339();

        sqlx::query!(
            r#"
        UPDATE test_sessions
        SET ended_at = ?
        WHERE id = ?
        "#,
            ended_at,
            session_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_test_session(&self, session_id: i64) -> Result<Option<TestSession>> {
        let session = sqlx::query_as!(
            TestSession,
            r#"
        SELECT
            id as "id!",
            test_id as "test_id!",
            started_at as "started_at!",
            ended_at,
            reason
        FROM test_sessions
        WHERE id = ?
        "#,
            session_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn list_test_sessions(&self, test_id: i64) -> Result<Vec<TestSession>> {
        let sessions = sqlx::query_as!(
            TestSession,
            r#"
        SELECT
            id as "id!",
            test_id as "test_id!",
            started_at as "started_at!",
            ended_at,
            reason
        FROM test_sessions
        WHERE test_id = ?
        ORDER BY started_at
        "#,
            test_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }

    pub async fn next_sample_index(&self, session_id: i64) -> Result<i64> {
        let row = sqlx::query!(
            r#"
        SELECT COALESCE(MAX(sample_index) + 1, 0) as "next_index!"
        FROM samples
        WHERE session_id = ?
        "#,
            session_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.next_index)
    }

    pub async fn append_sample(&self, sample: &Sample) -> Result<()> {
        sqlx::query!(
            r#"
        INSERT INTO samples (
            session_id,
            sample_index,
            timestamp,
            elapsed_ms,
            voltage_mv,
            current_ma,
            capacity_mah,
            energy_mwh
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
            sample.session_id,
            sample.sample_index,
            sample.timestamp,
            sample.elapsed_ms,
            sample.voltage_mv,
            sample.current_ma,
            sample.capacity_mah,
            sample.energy_mwh,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn append_sample_auto_index(
        &self,
        session_id: i64,
        elapsed_ms: i64,
        voltage_mv: i64,
        current_ma: i64,
        capacity_mah: i64,
        energy_mwh: Option<i64>,
    ) -> Result<i64> {
        let sample_index = self.next_sample_index(session_id).await?;

        let sample = Sample {
            session_id,
            sample_index,
            timestamp: chrono::Utc::now().to_rfc3339(),
            elapsed_ms,
            voltage_mv,
            current_ma,
            capacity_mah,
            energy_mwh,
        };

        self.append_sample(&sample).await?;

        Ok(sample_index)
    }

    pub async fn list_samples_for_session(&self, session_id: i64) -> Result<Vec<Sample>> {
        let samples = sqlx::query_as!(
            Sample,
            r#"
        SELECT
            session_id as "session_id!",
            sample_index as "sample_index!",
            timestamp as "timestamp!",
            elapsed_ms as "elapsed_ms!",
            voltage_mv as "voltage_mv!",
            current_ma as "current_ma!",
            capacity_mah as "capacity_mah!",
            energy_mwh
        FROM samples
        WHERE session_id = ?
        ORDER BY sample_index
        "#,
            session_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(samples)
    }
}

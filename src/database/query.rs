use crate::database::db::Database;
use crate::database::schema::{ApplicationStatus, JobApplication, PositionCategory}; // Assuming you have a Db type for your database connection/context
use color_eyre::Result;
use rusqlite::{ToSql, params};

//
// ---------------
// --- GETTERS ---
// ---------------
//
pub fn get_all_applications(db: &Database) -> Vec<JobApplication> {
    let conn = db.connection();
    let mut stmt = conn
        .prepare("SELECT id, company_name, position, position_category, work_type, location, location_type, application_date, status, is_active, notes, contact_info, url, files FROM job_applications")
        .unwrap();
    let rows = stmt
        .query_map([], |row| JobApplication::from_row(row))
        .unwrap();
    rows.filter_map(Result::ok).collect()
}

pub fn get_applications_by_status(status: ApplicationStatus, db: &Database) -> Vec<JobApplication> {
    let conn = db.connection();
    let mut stmt = conn
        .prepare("SELECT id, company_name, position, position_category, work_type, location, location_type, application_date, status, is_active, notes, contact_info, url, files FROM job_applications WHERE status = ?1")
        .unwrap();
    let rows = stmt
        .query_map(params![status.to_string()], |row| {
            JobApplication::from_row(row)
        })
        .unwrap();
    rows.filter_map(Result::ok).collect()
}

pub fn get_application_by_company(company_name: &str, db: &Database) -> Option<JobApplication> {
    let conn = db.connection();
    let mut stmt = conn
        .prepare("SELECT id, company_name, position, position_category, work_type, location, location_type, application_date, status, is_active, notes, contact_info, url, files FROM job_applications WHERE company_name = ?1")
        .ok()?;
    stmt.query_row(params![company_name], |row| JobApplication::from_row(row))
        .ok()
}

pub fn get_application_by_id(application_id: i32, db: &Database) -> Option<JobApplication> {
    let conn = db.connection();
    let mut stmt = conn
        .prepare("SELECT id, company_name, position, position_category, work_type, location, location_type, application_date, status, is_active, notes, contact_info, url, files FROM job_applications WHERE id = ?1")
        .ok()?;
    stmt.query_row(params![application_id], |row| JobApplication::from_row(row))
        .ok()
}

pub fn get_application_by_position(
    position: &PositionCategory,
    db: &Database,
) -> Option<JobApplication> {
    let conn = db.connection();
    let mut stmt = conn
        .prepare("SELECT id, company_name, position, position_category, work_type, location, location_type, application_date, status, is_active, notes, contact_info, url, files FROM job_applications WHERE position_category = ?1")
        .ok()?;
    stmt.query_row(params![position.to_string()], |row| {
        JobApplication::from_row(row)
    })
    .ok()
}

//
// ---------------
// --- SETTERS ---
// ---------------
//

pub fn add_application(application: JobApplication, db: &Database) -> Result<()> {
    let conn = db.connection();
    conn.execute(
        "INSERT INTO job_applications (company_name, position, position_category, work_type, location, location_type, application_date, status, is_active, notes, contact_info, url, files)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            application.company_name,
            application.position,
            application.position_category.to_string(),
            application.work_type,
            application.location,
            application.location_type,
            application.application_date,
            application.status.to_string(),
            application.is_active,
            application.notes,
            application.contact_info,
            application.url,
            application.files,
        ],
    )?;
    Ok(())
}

pub fn update_application(application: JobApplication, db: &Database) -> Result<()> {
    let conn = db.connection();
    conn.execute(
        "UPDATE job_applications SET company_name = ?1, position = ?2, position_category = ?3, work_type = ?4, location = ?5, location_type = ?6, application_date = ?7, status = ?8, is_active = ?9, notes = ?10, contact_info = ?11, url = ?12, files = ?13 WHERE id = ?14",
        params![
            application.company_name,
            application.position,
            application.position_category.to_string(),
            application.work_type,
            application.location,
            application.location_type,
            application.application_date,
            application.status.to_string(),
            application.is_active,
            application.notes,
            application.contact_info,
            application.url,
            application.files,
            application.id,
        ],
    )?;
    Ok(())
}

pub fn delete_application(application_id: i32, db: &Database) -> Result<()> {
    let conn = db.connection();
    conn.execute(
        "DELETE FROM job_applications WHERE id = ?1",
        params![application_id],
    )?;
    Ok(())
}

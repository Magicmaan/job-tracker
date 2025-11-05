use rusqlite::ToSql;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum PositionCategory {
    Engineering,
    Development,
    Support,
    DataScience,
    Analyst,
    Design,
}
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum WorkType {
    FullTime,
    PartTime,
    Internship,
    Contract,
    Temporary,
    Volunteer,
    Other,
}
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum LocationType {
    Remote,
    OnSite,
    Hybrid,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Applied,
    Interviewing,
    Offered,
    Rejected,
    Withdrawn,
    Accepted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Files {
    pub cv: String,
    pub cover_letter: String,
    pub additional_documents: Vec<String>,
}
impl ToSql for Files {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(
            self.cv.clone() + "," + &self.cover_letter + "," + &self.additional_documents.join(","),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: i32,
    pub company_name: String,
    pub position: String,
    pub position_category: PositionCategory,
    pub work_type: WorkType,
    pub location: String,
    pub location_type: LocationType,
    pub application_date: String,
    pub status: ApplicationStatus,
    pub is_active: bool, // whether status isn't final
    pub notes: Option<String>,
    pub contact_info: Option<String>,
    pub url: Option<String>,
    pub files: Files, // list of file paths for extra documents
}

impl Default for JobApplication {
    fn default() -> Self {
        JobApplication {
            id: 0,
            company_name: String::new(),
            position: String::new(),
            position_category: PositionCategory::Engineering,
            work_type: WorkType::FullTime,
            location: String::new(),
            location_type: LocationType::Remote,
            application_date: String::new(),
            status: ApplicationStatus::Applied,
            is_active: true,
            notes: None,
            contact_info: None,
            url: None,
            files: Files {
                cv: String::new(),
                cover_letter: String::new(),
                additional_documents: Vec::new(),
            },
        }
    }
}

impl JobApplication {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(JobApplication {
            id: row.get("id")?,
            company_name: row.get("company_name")?,
            position: row.get("position")?,
            position_category: PositionCategory::from_str(
                &row.get::<_, String>("position_category")?,
            )
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    0,
                    "position_category".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?,
            work_type: WorkType::from_str(&row.get::<_, String>("work_type")?).map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    0,
                    "work_type".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?,
            location: row.get("location")?,
            location_type: LocationType::from_str(&row.get::<_, String>("location_type")?)
                .map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        0,
                        "location_type".to_string(),
                        rusqlite::types::Type::Text,
                    )
                })?,
            application_date: row.get("application_date")?,
            status: ApplicationStatus::from_str(&row.get::<_, String>("status")?).map_err(
                |_| {
                    rusqlite::Error::InvalidColumnType(
                        0,
                        "status".to_string(),
                        rusqlite::types::Type::Text,
                    )
                },
            )?,
            is_active: row.get("is_active")?,
            notes: row.get("notes")?,
            contact_info: row.get("contact_info")?,
            url: row.get("url")?,
            files: {
                // Example: "cv.pdf,cover_letter.pdf,doc1.pdf,doc2.pdf"
                let files_str = row.get::<_, Option<String>>("files")?.unwrap_or_default();
                let mut parts = files_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>();
                let cv = parts.get(0).cloned().unwrap_or_default();
                let cover_letter = parts.get(1).cloned().unwrap_or_default();
                let additional_documents = if parts.len() > 2 {
                    parts[2..].to_vec()
                } else {
                    Vec::new()
                };
                Files {
                    cv,
                    cover_letter,
                    additional_documents,
                }
            },
        })
    }

    pub fn test(id: i32) -> Self {
        JobApplication {
            id,
            company_name: "Acme Inc ".to_string() + &id.to_string(),
            position: "Backend Developer ".to_string() + &id.to_string(),
            position_category: PositionCategory::Engineering,
            work_type: WorkType::FullTime,
            location: "San Francisco, CA".to_string(),
            location_type: LocationType::Remote,
            application_date: "2024-05-20".to_string(),
            status: ApplicationStatus::Interviewing,
            is_active: true,
            notes: Some("Interview scheduled for next week.".to_string()),
            contact_info: Some("recruiter@acme.com".to_string()),
            url: Some("https://acme.com/jobs/123".to_string()),
            files: Files {
                cv: "acme_cv.pdf".to_string(),
                cover_letter: "acme_cover.pdf".to_string(),
                additional_documents: vec!["portfolio.pdf".to_string()],
            },
        }
    }
}

//
// ----------------------------------------------------
// --- Conversion and parsing implementations below ---
// ----------------------------------------------------
//
impl PositionCategory {
    pub fn to_string(&self) -> String {
        match self {
            PositionCategory::Engineering => "Engineering".to_string(),
            PositionCategory::Development => "Development".to_string(),
            PositionCategory::Support => "Support".to_string(),
            PositionCategory::DataScience => "Data Science".to_string(),
            PositionCategory::Analyst => "Analyst".to_string(),
            PositionCategory::Design => "Design".to_string(),
        }
    }
}

impl FromStr for PositionCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Engineering" => Ok(PositionCategory::Engineering),
            "Development" => Ok(PositionCategory::Development),
            "Support" => Ok(PositionCategory::Support),
            "Data Science" => Ok(PositionCategory::DataScience),
            "Analyst" => Ok(PositionCategory::Analyst),
            "Design" => Ok(PositionCategory::Design),
            _ => Err(()),
        }
    }
}

impl WorkType {
    pub fn to_string(&self) -> String {
        match self {
            WorkType::FullTime => "Full Time".to_string(),
            WorkType::PartTime => "Part Time".to_string(),
            WorkType::Internship => "Internship".to_string(),
            WorkType::Contract => "Contract".to_string(),
            WorkType::Temporary => "Temporary".to_string(),
            WorkType::Volunteer => "Volunteer".to_string(),
            WorkType::Other => "Other".to_string(),
        }
    }
}

impl FromStr for WorkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Full Time" => Ok(WorkType::FullTime),
            "Part Time" => Ok(WorkType::PartTime),
            "Internship" => Ok(WorkType::Internship),
            "Contract" => Ok(WorkType::Contract),
            "Temporary" => Ok(WorkType::Temporary),
            "Volunteer" => Ok(WorkType::Volunteer),
            "Other" => Ok(WorkType::Other),
            _ => Err(()),
        }
    }
}
impl ToSql for WorkType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.to_string()))
    }
}

impl LocationType {
    pub fn to_string(&self) -> String {
        match self {
            LocationType::Remote => "Remote".to_string(),
            LocationType::OnSite => "On Site".to_string(),
            LocationType::Hybrid => "Hybrid".to_string(),
        }
    }
}
impl ToSql for LocationType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.to_string()))
    }
}
impl FromStr for LocationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Remote" => Ok(LocationType::Remote),
            "On Site" => Ok(LocationType::OnSite),
            "Hybrid" => Ok(LocationType::Hybrid),
            _ => Err(()),
        }
    }
}

impl ApplicationStatus {
    pub fn to_string(&self) -> String {
        match self {
            ApplicationStatus::Applied => "Applied".to_string(),
            ApplicationStatus::Interviewing => "Interviewing".to_string(),
            ApplicationStatus::Offered => "Offered".to_string(),
            ApplicationStatus::Rejected => "Rejected".to_string(),
            ApplicationStatus::Withdrawn => "Withdrawn".to_string(),
            ApplicationStatus::Accepted => "Accepted".to_string(),
        }
    }
}
impl ToSql for ApplicationStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.to_string()))
    }
}

impl FromStr for ApplicationStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Applied" => Ok(ApplicationStatus::Applied),
            "Interviewing" => Ok(ApplicationStatus::Interviewing),
            "Offered" => Ok(ApplicationStatus::Offered),
            "Rejected" => Ok(ApplicationStatus::Rejected),
            "Withdrawn" => Ok(ApplicationStatus::Withdrawn),
            "Accepted" => Ok(ApplicationStatus::Accepted),
            _ => Err(()),
        }
    }
}

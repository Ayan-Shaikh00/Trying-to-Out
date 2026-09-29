use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub fn from_string(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            _ => Err(format!("Invalid priority: {}", value)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Pending => "Pending",
            Status::InProgress => "In Progress",
            Status::Completed => "Completed",
            Status::Cancelled => "Cancelled",
        }
    }

    pub fn from_string(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_str() {
            "pending" => Ok(Status::Pending),
            "progress" | "inprogress" | "in-progress" => {
                Ok(Status::InProgress)
            }
            "completed" | "complete" => Ok(Status::Completed),
            "cancelled" | "canceled" => Ok(Status::Cancelled),
            _ => Err(format!("Invalid status: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        priority: Priority,
        tags: Vec<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            title,
            description,
            priority,
            status: Status::Pending,
            tags,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn matches(&self, query: &str) -> bool {
        let query = query.to_lowercase();

        self.title.to_lowercase().contains(&query)
            || self.description.to_lowercase().contains(&query)
            || self
                .tags
                .iter()
                .any(|tag| tag.to_lowercase().contains(&query))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDatabase {
    pub tasks: Vec<Task>,
}

impl TaskDatabase {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }
}

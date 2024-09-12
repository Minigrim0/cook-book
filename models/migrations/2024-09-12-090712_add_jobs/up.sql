-- Your SQL goes here
-- Create the main job table
CREATE TABLE job (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    status TEXT NOT NULL,
    progress REAL NOT NULL DEFAULT 0,
    details TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create a separate table for logs
CREATE TABLE job_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    log_entry TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (job_id) REFERENCES job(id) ON DELETE CASCADE
);

-- Create an index on the status column for faster queries on the job table
CREATE INDEX idx_job_status ON job(status);

-- Create an index on the job_id column for faster queries on the job_log table
CREATE INDEX idx_job_log_job_id ON job_log(job_id);

-- Trigger to update the updated_at timestamp in the job table
CREATE TRIGGER update_job_timestamp 
AFTER UPDATE ON job
BEGIN
    UPDATE job SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;


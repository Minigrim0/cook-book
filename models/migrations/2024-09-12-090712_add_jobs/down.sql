-- This file should undo anything in `up.sql`
-- Drop the trigger
DROP TRIGGER IF EXISTS update_job_timestamp;

-- Drop the indexes
DROP INDEX IF EXISTS idx_job_log_job_id;
DROP INDEX IF EXISTS idx_job_status;

-- Drop the job_log table
DROP TABLE IF EXISTS job_log;

-- Drop the job table
DROP TABLE IF EXISTS job;

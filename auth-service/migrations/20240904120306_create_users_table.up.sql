-- Copyright 2024 Ibrahim Mbaziira

-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at

--     http://www.apache.org/licenses/LICENSE-2.0

-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.

-- migrate:up
-- Create users table
CREATE TABLE IF NOT EXISTS users(
   email TEXT NOT NULL PRIMARY KEY,
   password_hash TEXT NOT NULL,
   require_2fa BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create index on email
CREATE INDEX idx_users_email ON users(email);

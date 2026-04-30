import dotenv from 'dotenv';
import { Client } from 'pg';

dotenv.config();

const {
  POSTGRES_HOST,
  POSTGRES_DB,
  POSTGRES_USER,
  POSTGRES_PASSWORD,
  POSTGRES_PORT
} = process.env;

if (!POSTGRES_HOST || !POSTGRES_DB || !POSTGRES_USER || !POSTGRES_PASSWORD) {
  console.error('Missing required Postgres environment variables. Make sure .env is set.');
  process.exit(1);
}

const client = new Client({
  host: POSTGRES_HOST,
  database: POSTGRES_DB,
  user: POSTGRES_USER,
  password: POSTGRES_PASSWORD,
  port: POSTGRES_PORT ? Number(POSTGRES_PORT) : 5432,
});

async function main() {
  try {
    await client.connect();
    const result = await client.query('SELECT current_database() AS current_database');
    const currentDatabase = result.rows[0]?.current_database;
    console.log('Connected to Postgres. Current database:', currentDatabase);
  } catch (error) {
    console.error('Database connection failed:', error);
    process.exitCode = 1;
  } finally {
    await client.end();
  }
}

main();

import assert from "node:assert/strict";
import { Client } from "pg";

const connectionString = process.env.DATABASE_URL;
const owner = new Client({ connectionString });
await owner.connect();

try {
  await owner.query(`GRANT SELECT, INSERT, UPDATE ON public.users TO anon, authenticated`);
  await owner.query(`
    INSERT INTO public.users (id, email, name, "updatedAt")
    VALUES
      ('10000000-0000-4000-8000-000000000001', 'one@example.test', 'One', now()),
      ('10000000-0000-4000-8000-000000000002', 'two@example.test', 'Two', now())
  `);
} finally {
  await owner.end();
}

async function withRole(role, userId, run) {
  const client = new Client({ connectionString });
  await client.connect();
  try {
    await client.query(`SET ROLE ${role}`);
    if (userId) {
      await client.query("SELECT set_config('request.jwt.claim.sub', $1, false)", [userId]);
    }
    return await run(client);
  } finally {
    await client.end();
  }
}

const anonymousCount = await withRole("anon", null, async (client) => {
  const result = await client.query("SELECT count(*)::int AS count FROM public.users");
  return result.rows[0].count;
});
assert.equal(anonymousCount, 0, "anonymous users must not read user profiles");

const ownProfileCount = await withRole(
  "authenticated",
  "10000000-0000-4000-8000-000000000001",
  async (client) => {
    const result = await client.query("SELECT count(*)::int AS count FROM public.users");
    return result.rows[0].count;
  },
);
assert.equal(ownProfileCount, 1, "authenticated users must see only their own profile");

const ownInsertCount = await withRole(
  "authenticated",
  "10000000-0000-4000-8000-000000000003",
  async (client) => {
    const result = await client.query(
      `INSERT INTO public.users (id, email, "updatedAt") VALUES ($1, $2, now())`,
      ["10000000-0000-4000-8000-000000000003", "three@example.test"],
    );
    return result.rowCount;
  },
);
assert.equal(ownInsertCount, 1, "authenticated users must be able to create their own profile");

const crossUserUpdateCount = await withRole(
  "authenticated",
  "10000000-0000-4000-8000-000000000001",
  async (client) => {
    const result = await client.query(
      `UPDATE public.users SET name = 'Changed' WHERE id = $1 RETURNING id`,
      ["10000000-0000-4000-8000-000000000002"],
    );
    return result.rowCount;
  },
);
assert.equal(crossUserUpdateCount, 0, "users must not update another profile");

await assert.rejects(
  withRole("anon", null, (client) =>
    client.query(`INSERT INTO public.users (id, email, "updatedAt") VALUES ($1, $2, now())`, [
      "10000000-0000-4000-8000-000000000004",
      "anonymous@example.test",
    ]),
  ),
  (error) => error.code === "42501",
  "anonymous users must not create profiles",
);

await assert.rejects(
  withRole("authenticated", "10000000-0000-4000-8000-000000000001", (client) =>
    client.query(`INSERT INTO public.users (id, email, "updatedAt") VALUES ($1, $2, now())`, [
      "10000000-0000-4000-8000-000000000002",
      "forged@example.test",
    ]),
  ),
  (error) => error.code === "42501",
  "authenticated users must not insert a profile for another user",
);

console.log("PostgreSQL migration and RLS checks passed");

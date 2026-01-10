import "dotenv/config";
import { defineConfig, env } from "prisma/config";

export default defineConfig({
	schema: "crates/database/prisma/schema.prisma",
	datasource: {
		url: env("DATABASE_URL"),
	},
});

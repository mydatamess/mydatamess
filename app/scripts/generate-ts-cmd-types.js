import { compileFromFile } from "json-schema-to-typescript";
import process from "node:process";
import path from "path";
import fs from "fs/promises";
import { fileURLToPath } from "url";

// ESM-friendly __dirname setup
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Get CLI arguments (with fallback defaults)
const args = process.argv.slice(2);
const SCHEMA_FOLDER = path.resolve(__dirname, args[0] || "../schemas");
const TYPES_FOLDER = path.resolve(args[1]);

async function generateTypes(schemaDir, typesDir) {
  try {
    await fs.mkdir(typesDir, { recursive: true });
    await processSchemaFolder(schemaDir, typesDir);
    console.log("TypeScript types generated successfully.");
  } catch (error) {
    if (error instanceof Error) {
      console.error("Error generating TypeScript types:", error.message);
    } else {
      console.error("Unknown error:", error);
    }
  }
}

async function processSchemaFolder(schemaDir, typesDir) {
  const entries = await fs.readdir(schemaDir, { withFileTypes: true });

  for (const entry of entries) {
    const schemaPath = path.join(schemaDir, entry.name);
    const typePath = path.join(
      typesDir,
      entry.name.replace(/\.json$/, ".d.ts")
    );

    if (entry.isDirectory()) {
      await fs.mkdir(typePath, { recursive: true });
      await processSchemaFolder(schemaPath, typePath);
    } else if (entry.isFile() && entry.name.endsWith(".json")) {
      const ts = await compileFromFile(schemaPath);
      await fs.writeFile(typePath, ts);
    }
  }
}

// Run the generator
generateTypes(SCHEMA_FOLDER, TYPES_FOLDER);

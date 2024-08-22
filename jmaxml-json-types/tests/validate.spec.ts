import { describe, it, expect } from "vitest";
import type { Report } from "../index";

import typia from "typia";

const check = typia.createIs<Report>();

const fixtureFiles = import.meta.glob("./fixtures/*.json");

describe("Validate fixtures", () => {
  for (const [filePath, load] of Object.entries(fixtureFiles)) {
    it(`Validate ${filePath}`, async () => {
      const data = await load();
      const result = typia.validate<Report>(data);
      if (!result.success) {
        console.log(result.errors);
      }
      expect(result.success).toBe(true);
    });
  }
});

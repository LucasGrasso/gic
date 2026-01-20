import { Zap, Hash, List, LucideIcon } from "lucide-react";

export type Predicate = {
  name: string;
  signature: string;
  description: string;
  note?: string;
};

export type Category = {
  slug: string;
  title: string;
  description: string;
  predicates: Predicate[];
};

export type CategoryMeta = {
  icon: LucideIcon;
  description: string;
};

export const categoryMeta: Record<string, CategoryMeta> = {
  "all-purpose": {
    icon: Zap,
    description: "General predicates for unification and variable checking",
  },
  integers: {
    icon: Hash,
    description: "Arithmetic operations and numeric comparisons",
  },
  lists: {
    icon: List,
    description: "List manipulation and querying predicates",
  },
};

function slugify(title: string): string {
  return title
    .toLowerCase()
    .replace(/\s+/g, "-")
    .replace(/[^a-z0-9-]/g, "");
}

export function parseBuiltinsMd(content: string): Category[] {
  const categories: Category[] = [];
  let currentCategory: Category | null = null;

  const lines = content.split("\n");

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Match h2 headers (## Category)
    const h2Match = line.match(/^## (.+)$/);
    if (h2Match) {
      if (currentCategory) {
        categories.push(currentCategory);
      }
      const title = h2Match[1].trim();
      const slug = slugify(title);
      currentCategory = {
        slug,
        title,
        description: categoryMeta[slug]?.description || "",
        predicates: [],
      };
      continue;
    }

    // Match h4 headers (#### `Predicate(args)`)
    const h4Match = line.match(/^#### `(.+)`$/);
    if (h4Match && currentCategory) {
      const signature = h4Match[1].trim();
      const nameMatch = signature.match(/^(\w+)\(/);
      const name = nameMatch ? nameMatch[1] : signature;

      // Collect description lines until next header or empty predicate block
      let description = "";
      let note: string | undefined;
      let j = i + 1;

      while (j < lines.length) {
        const nextLine = lines[j];
        // Stop at next header
        if (nextLine.match(/^#{1,4} /)) break;
        // Stop at empty line followed by header
        if (
          nextLine.trim() === "" &&
          j + 1 < lines.length &&
          lines[j + 1].match(/^#{1,4} /)
        )
          break;

        if (nextLine.trim()) {
          const text = nextLine.trim();
          // Check if this line contains instantiation notes
          if (
            text.includes("should be instanciated") ||
            text.includes("should be instantiated")
          ) {
            note = text;
          } else if (text.startsWith("If ") && note) {
            // Append error conditions to note
            note += " " + text;
          } else if (text.startsWith("If ") && !note && description) {
            // Error condition without prior note
            note = text;
          } else {
            description += (description ? " " : "") + text;
          }
        }
        j++;
      }

      currentCategory.predicates.push({
        name,
        signature,
        description,
        note,
      });
    }
  }

  // Push last category
  if (currentCategory) {
    categories.push(currentCategory);
  }

  return categories;
}

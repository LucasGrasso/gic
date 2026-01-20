import { promises as fs } from 'fs'
import path from 'path'
import Navbar from '@/components/navbar/Navbar'
import Footer from '@/components/footer/Footer'
import Link from 'next/link'
import { ChevronLeft } from 'lucide-react'
import { notFound } from 'next/navigation'
import { parseBuiltinsMd, categoryMeta, Category } from '@/lib/builtins'

// Formats text with inline code for backtick-wrapped content
function formatText(text: string) {
	const parts = text.split(/(`[^`]+`)/g)
	return parts.map((part, i) => {
		if (part.startsWith('`') && part.endsWith('`')) {
			return (
				<code key={i} className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">
					{part.slice(1, -1)}
				</code>
			)
		}
		return part
	})
}

async function getCategories(): Promise<Category[]> {
	const filePath = path.join(process.cwd(), '..', 'BuiltIns.md')
	const content = await fs.readFile(filePath, 'utf8')
	return parseBuiltinsMd(content)
}

type PageProps = {
	params: Promise<{ category: string }>
}

export async function generateStaticParams() {
	const categories = await getCategories()
	return categories.map((cat) => ({ category: cat.slug }))
}

export default async function CategoryPage({ params }: PageProps) {
	const { category: categorySlug } = await params
	const categories = await getCategories()
	const category = categories.find((c) => c.slug === categorySlug)

	if (!category) {
		notFound()
	}

	const meta = categoryMeta[categorySlug]
	const Icon = meta?.icon
	const currentIndex = categories.findIndex((c) => c.slug === categorySlug)
	const prevCategory = currentIndex > 0 ? categories[currentIndex - 1] : null
	const nextCategory = currentIndex < categories.length - 1 ? categories[currentIndex + 1] : null

	return (
		<div className="min-h-screen bg-gradient-to-br from-background via-background to-secondary/20 dark:from-background dark:via-background dark:to-secondary/10">
			<Navbar />

			<main className="max-w-4xl mx-auto px-6 py-16">
				<Link
					href="/builtins"
					className="inline-flex items-center gap-2 text-foreground/60 hover:text-foreground transition mb-8"
				>
					<ChevronLeft className="w-4 h-4" />
					All Built-ins
				</Link>

				{/* Header */}
				<div className="flex items-center gap-4 mb-8">
					<div className="p-3 bg-accent/10 rounded-lg">
						<Icon className="w-8 h-8 text-accent" />
					</div>
					<div>
						<h1 className="text-3xl font-bold text-foreground">{category.title}</h1>
						<p className="text-foreground/60">{category.description}</p>
					</div>
				</div>

				{/* Table of Contents */}
				<nav className="bg-card/50 border border-border rounded-xl p-6 mb-10">
					<h2 className="text-sm font-semibold text-foreground/60 uppercase tracking-wider mb-4">
						On this page
					</h2>
					<ul className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
						{category.predicates.map((pred) => (
							<li key={pred.name}>
								<a
									href={`#${pred.name.toLowerCase()}`}
									className="text-sm font-mono text-accent hover:underline"
								>
									{pred.name}
								</a>
							</li>
						))}
					</ul>
				</nav>

				{/* Predicates */}
				<div className="space-y-8">
					{category.predicates.map((pred) => (
						<section
							key={pred.name}
							id={pred.name.toLowerCase()}
							className="scroll-mt-24 bg-card/30 border border-border rounded-xl p-6"
						>
							<h3 className="text-xl font-mono font-semibold text-accent mb-3">
								{pred.signature}
							</h3>
							<p className="text-foreground/80 leading-relaxed">{formatText(pred.description)}</p>
							{pred.note && (
								<p className="mt-3 text-sm text-foreground/60 bg-muted/50 px-3 py-2 rounded-lg">
									<span className="font-medium">Note:</span> {formatText(pred.note)}
								</p>
							)}
						</section>
					))}
				</div>

				{/* Navigation */}
				<div className="flex justify-between items-center mt-12 pt-8 border-t border-border">
					{prevCategory ? (
						<Link
							href={`/builtins/${prevCategory.slug}`}
							className="inline-flex items-center gap-2 text-foreground/60 hover:text-accent transition"
						>
							<ChevronLeft className="w-4 h-4" />
							<span className="font-medium">{prevCategory.title}</span>
						</Link>
					) : (
						<div />
					)}
					{nextCategory ? (
						<Link
							href={`/builtins/${nextCategory.slug}`}
							className="inline-flex items-center gap-2 text-foreground/60 hover:text-accent transition"
						>
							<span className="font-medium">{nextCategory.title}</span>
							<ChevronLeft className="w-4 h-4 rotate-180" />
						</Link>
					) : (
						<div />
					)}
				</div>
			</main>

			<Footer />
		</div>
	)
}

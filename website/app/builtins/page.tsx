import { promises as fs } from 'fs'
import path from 'path'
import Navbar from '@/components/navbar/Navbar'
import Footer from '@/components/footer/Footer'
import Link from 'next/link'
import { ChevronLeft, ChevronRight } from 'lucide-react'
import { parseBuiltinsMd, categoryMeta } from '@/lib/builtins'

export default async function BuiltinsPage() {
	const filePath = path.join(process.cwd(), '..', 'BuiltIns.md')
	const content = await fs.readFile(filePath, 'utf8')
	const categories = parseBuiltinsMd(content)

	return (
		<div className="min-h-screen bg-gradient-to-br from-background via-background to-secondary/20 dark:from-background dark:via-background dark:to-secondary/10">
			<Navbar />

			<main className="max-w-4xl mx-auto px-6 py-16">
				<Link
					href="/"
					className="inline-flex items-center gap-2 text-foreground/60 hover:text-foreground transition mb-8"
				>
					<ChevronLeft className="w-4 h-4" />
					Back to Home
				</Link>

				<h1 className="text-4xl font-bold text-foreground mb-4">Built-in Predicates</h1>
				<p className="text-foreground/70 text-lg mb-12 max-w-2xl">
					GIC comes with a set of built-in predicates for common operations.
					Select a category below to explore the available predicates.
				</p>

				<div className="grid gap-6">
					{categories.map((category) => {
						const meta = categoryMeta[category.slug]
						const Icon = meta?.icon
						return (
							<Link
								key={category.slug}
								href={`/builtins/${category.slug}`}
								className="group block bg-card/50 hover:bg-card border border-border rounded-xl p-6 transition-all hover:shadow-lg hover:border-accent/50"
							>
								<div className="flex items-start justify-between">
									<div className="flex items-start gap-4">
										{Icon && (
											<div className="p-3 bg-accent/10 rounded-lg group-hover:bg-accent/20 transition">
												<Icon className="w-6 h-6 text-accent" />
											</div>
										)}
										<div>
											<h2 className="text-xl font-semibold text-foreground mb-1 group-hover:text-accent transition">
												{category.title}
											</h2>
											<p className="text-foreground/60 mb-3">{category.description}</p>
											<div className="flex flex-wrap gap-2">
												{category.predicates.map((pred) => (
													<span
														key={pred.name}
														className="text-xs font-mono bg-muted px-2 py-1 rounded text-foreground/70"
													>
														{pred.name}
													</span>
												))}
											</div>
										</div>
									</div>
									<ChevronRight className="w-5 h-5 text-foreground/40 group-hover:text-accent transition mt-1" />
								</div>
							</Link>
						)
					})}
				</div>
			</main>

			<Footer />
		</div>
	)
}

"use client"

import * as React from "react"
import Image from "next/image"
import Link from "next/link"

export function Footer() {
	return (
		<footer className="border-t border-border bg-card/50">
			<div className="max-w-6xl mx-auto px-6 py-12">
				<div className="grid md:grid-cols-4 gap-8 mb-8">
					<div>
						<div className="flex items-center gap-3 mb-4">
							<Image src="/logo.png" alt="GIC Logo" width={32} height={32} className="rounded-lg" />
							<span className="font-bold text-foreground">GIC</span>
						</div>
						<p className="text-sm text-foreground/60">Logic programming language for first-order logic.</p>
					</div>

					<div>
						<h4 className="font-semibold text-foreground mb-4">Learn</h4>
						<ul className="space-y-2 text-sm text-foreground/60">
							<li>
								<a href="#syntax" className="hover:text-foreground transition">
									Syntax Guide
								</a>
							</li>
							<li>
								<a href="#features" className="hover:text-foreground transition">
									Features
								</a>
							</li>
							<li>
								<a href="#examples" className="hover:text-foreground transition">
									Examples
								</a>
							</li>
						</ul>
					</div>

					<div>
						<h4 className="font-semibold text-foreground mb-4">Resources</h4>
						<ul className="space-y-2 text-sm text-foreground/60">
							<li>
								<Link
									href="/builtins"
									className="hover:text-foreground transition"
								>
									Built-ins
								</Link>
							</li>
							<li>
								<a
									href="https://github.com/LucasGrasso/gic"
									target="_blank"
									rel="noopener noreferrer"
									className="hover:text-foreground transition"
								>
									GitHub
								</a>
							</li>
						</ul>
					</div>

					<div>
						<h4 className="font-semibold text-foreground mb-4">License</h4>
						<p className="text-sm text-foreground/60">
							GIC is licensed under the <br />
							<a
								href="https://github.com/LucasGrasso/gic/blob/master/LICENSE.md"
								target="_blank"
								rel="noopener noreferrer"
								className="hover:text-foreground transition"
							>
								GNU General Public License v3.0
							</a>
						</p>
					</div>
				</div>

				<div className="border-t border-border pt-8 flex flex-col md:flex-row justify-between items-center text-sm text-foreground/60">
					<p>&copy; 2025 Lucas Grasso. All rights reserved.</p>
					<p>Created with ❤️ for logic programming</p>
				</div>
			</div>
		</footer>
	)
}

export default Footer

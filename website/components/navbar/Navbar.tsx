"use client"

import * as React from "react"
import { ChevronRight, Github } from "lucide-react"
import Logo from "@/components/logo/Logo"
import { Button } from "@/components/ui/button"

export function Navbar() {
	return (
		<nav className="sticky top-0 z-50 bg-background/80 backdrop-blur-lg border-b border-border">
			<div className="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
				<div className="flex items-center gap-3">
					<Logo />
				</div>

				<div className="hidden md:flex items-center gap-8">
					<a href="#features" className="text-foreground/70 hover:text-foreground transition">
						Features
					</a>
					<a href="#syntax" className="text-foreground/70 hover:text-foreground transition">
						Learn
					</a>
					<a href="#examples" className="text-foreground/70 hover:text-foreground transition">
						Examples
					</a>
					<a
						href="https://github.com/LucasGrasso/gic"
						target="_blank"
						rel="noopener noreferrer"
						className="text-foreground/70 hover:text-foreground transition"
					>
						<Github className="w-5 h-5" />
					</a>
				</div>
			</div>
		</nav>
	)
}

export default Navbar

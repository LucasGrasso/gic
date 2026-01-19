"use client"

import Image from "next/image"
import Link from "next/link"
import * as React from "react"

export function Logo({ size = 40 }: { size?: number }) {
	return (
		<Link href="/" className="flex items-center gap-3">
			<Image src="/logo.png" alt="GIC Logo" width={size} height={size} className="rounded-lg" />
			<span className="text-xl font-bold text-foreground">GIC</span>
		</Link>
	)
}

export default Logo

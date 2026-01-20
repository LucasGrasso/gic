"use client"

import * as React from "react"
import katex from "katex"

type TeXProps = {
	children: string
	display?: boolean
	className?: string
}

export function TeX({ children, display = false, className }: TeXProps) {
	let html = ""
	try {
		html = katex.renderToString(children, { throwOnError: false, displayMode: display })
	} catch (e) {
		html = children
	}

	return <span className={className} dangerouslySetInnerHTML={{ __html: html }} />
}

export default TeX

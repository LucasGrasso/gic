"use client"

import * as React from "react"

type ButtonVariant = "default" | "outline" | "ghost"
type ButtonSize = "sm" | "md" | "lg"

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
	variant?: ButtonVariant
	size?: ButtonSize
	className?: string
}

const variantStyles: Record<ButtonVariant, string> = {
	default: "bg-accent text-accent-foreground border-transparent hover:bg-accent/90",
	outline: "bg-transparent border border-accent text-accent hover:bg-accent/10",
	ghost: "bg-transparent text-accent hover:bg-accent/10",
}

const sizeStyles: Record<ButtonSize, string> = {
	sm: "px-3 py-1.5 text-sm",
	md: "px-4 py-2 text-sm",
	lg: "px-6 py-3 text-base",
}

function mergeClassNames(...classes: Array<string | undefined | false>) {
	return classes.filter(Boolean).join(" ")
}

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
	({ variant = "default", size = "md", className, children, ...props }, ref) => {
		const classes = mergeClassNames(
			"inline-flex items-center justify-center rounded-md font-medium transition",
			variantStyles[variant],
			sizeStyles[size],
			className
		)

		return (
			<button ref={ref} className={classes} {...props}>
				{children}
			</button>
		)
	}
)

Button.displayName = "Button"

export default Button

import Image from 'next/image'
import { ChevronRight, Code2, Zap, BookOpen, Github } from 'lucide-react'
import { Button } from '@/components/ui/button'

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-background via-background to-secondary/20 dark:from-background dark:via-background dark:to-secondary/10">
      {/* Navigation */}
      <nav className="sticky top-0 z-50 bg-background/80 backdrop-blur-lg border-b border-border">
        <div className="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Image
              src="/logo.png"
              alt="GIC Logo"
              width={40}
              height={40}
              className="rounded-lg"
            />
            <span className="text-xl font-bold text-foreground">GIC</span>
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

      {/* Hero Section */}
      <section className="max-w-6xl mx-auto px-6 py-24 md:py-32">
        <div className="text-center space-y-6">
          <div className="flex justify-center mb-6">
            <Image
              src="/logo.png"
              alt="GIC Logo"
              width={120}
              height={120}
              className="rounded-2xl shadow-lg"
            />
          </div>

          <div className="inline-block px-4 py-2 bg-accent/10 border border-accent/20 rounded-full">
            <span className="text-sm font-medium text-accent">First Order Logic Programming</span>
          </div>

          <h1 className="text-5xl md:text-7xl font-bold text-balance text-foreground leading-tight">
            Logic Programming
            <span className="block text-primary">Redefined</span>
          </h1>

          <p className="text-lg md:text-xl text-foreground/60 max-w-2xl mx-auto text-balance">
            GIC is a modern programming language for First Order Logic. Parse, resolve, and compute with elegant syntax and powerful semantics.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center pt-8">
            <a href="https://github.com/LucasGrasso/gic/releases" target="_blank" rel="noopener noreferrer">
              <Button size="lg" className="w-full sm:w-auto bg-primary hover:bg-primary/90">
                Get Started
                <ChevronRight className="w-4 h-4 ml-2" />
              </Button>
            </a>
            <a href="https://github.com/LucasGrasso/gic" target="_blank" rel="noopener noreferrer">
              <Button size="lg" variant="outline" className="w-full sm:w-auto bg-transparent">
                View on GitHub
              </Button>
            </a>
          </div>
        </div>

        {/* Code Block */}
        <div className="mt-16 rounded-xl border border-border bg-card/50 backdrop-blur p-8 overflow-x-auto">
          <pre className="font-mono text-sm text-foreground/80 text-balance">
            <code>{`load "examples/family.gic"

query "∃ X. ∃ Y. Grandpa(X, Y)"

// Output:
X := juan(), Y := maria()`}</code>
          </pre>
        </div>
      </section>

      {/* Features Section */}
      <section id="features" className="max-w-6xl mx-auto px-6 py-24 border-t border-border">
        <h2 className="text-4xl font-bold text-center mb-4 text-foreground">Powerful Features</h2>
        <p className="text-center text-foreground/60 mb-16 max-w-2xl mx-auto">
          Everything you need for logic programming, built on solid foundations
        </p>

        <div className="grid md:grid-cols-3 gap-8">
          {[
            {
              icon: Code2,
              title: 'First Order Logic Parsing',
              description:
                'Parse and process first order logic expressions with full support for quantifiers, operators, and predicates.',
            },
            {
              icon: Zap,
              title: 'SLD Resolution',
              description:
                'Leverage SLD resolution for efficient query processing on facts and rules in your logic programs.',
            },
            {
              icon: BookOpen,
              title: 'Built-in Predicates',
              description:
                'Extensive library of built-in predicates including list operations, arithmetic, and logical operations.',
            },
          ].map((feature) => (
            <div key={feature.title} className="p-6 rounded-xl border border-border bg-card/50 hover:border-accent/50 transition">
              <feature.icon className="w-8 h-8 text-accent mb-4" />
              <h3 className="text-lg font-semibold text-foreground mb-2">{feature.title}</h3>
              <p className="text-foreground/60">{feature.description}</p>
            </div>
          ))}
        </div>
      </section>

      {/* Syntax Section */}
      <section id="syntax" className="max-w-6xl mx-auto px-6 py-24 border-t border-border">
        <div className="grid md:grid-cols-2 gap-12 items-center">
          <div>
            <h2 className="text-4xl font-bold text-foreground mb-6">Clean, Intuitive Syntax</h2>
            <p className="text-foreground/60 mb-6">
              GIC syntax is designed to be clear and expressive. Write first-order logic formulas naturally.
            </p>
            <ul className="space-y-4">
              {[
                'Predicates: P(t1, t2, ...)',
                'Logical operators: ∧, ∨, ⇒, ¬',
                'Quantifiers: ∃, ∀',
                'Variables and terms',
              ].map((item, i) => (
                <li key={i} className="flex gap-3 text-foreground/80">
                  <ChevronRight className="w-5 h-5 text-accent flex-shrink-0 mt-0.5" />
                  <span>{item}</span>
                </li>
              ))}
            </ul>
          </div>

          <div className="rounded-xl border border-border bg-card/50 backdrop-blur p-8">
            <pre className="font-mono text-sm text-foreground/80 overflow-x-auto">
              <code>{`// Grammar
L-Formulas f ::=
  P(t1,... tn)       // Predicates
  | ⊥               // Bottom
  | f ∧ f           // And
  | f ∨ f           // Or
  | f ⇒ f           // Implies
  | ¬f              // Not
  | ∃X. f           // Exists
  | ∀X. f           // Forall`}</code>
            </pre>
          </div>
        </div>
      </section>

      {/* Example Section */}
      <section id="examples" className="max-w-6xl mx-auto px-6 py-24 border-t border-border">
        <h2 className="text-4xl font-bold text-center mb-4 text-foreground">See It In Action</h2>
        <p className="text-center text-foreground/60 mb-16 max-w-2xl mx-auto">
          GIC makes it easy to work with logic programs. Here&apos;s what you can build.
        </p>

        <div className="space-y-8">
          {[
            {
              title: 'Family Relations',
              code: `Grandpa(X, Y) ⇒ ∃Z. (Father(X, Z) ∧ Father(Z, Y))`,
              description: 'Define and query complex relationships with logical rules.',
            },
            {
              title: 'List Operations',
              code: `Length([H|T], N) ⇒ Length(T, N-1)
Length([], 0)`,
              description: 'Work with recursive data structures and solve constraints.',
            },
            {
              title: 'Logical Queries',
              code: `query "∃ X. Brother(X, luis)"
// Returns all X where X is a brother of luis`,
              description: 'Express complex queries using quantified logic formulas.',
            },
          ].map((example) => (
            <div
              key={example.title}
              className="grid md:grid-cols-2 gap-8 p-8 rounded-xl border border-border bg-card/50 hover:border-accent/50 transition"
            >
              <div>
                <h3 className="text-xl font-semibold text-foreground mb-3">{example.title}</h3>
                <p className="text-foreground/60">{example.description}</p>
              </div>
              <div className="rounded-lg bg-background/50 border border-border p-4">
                <pre className="font-mono text-sm text-foreground/80 overflow-x-auto">
                  <code>{example.code}</code>
                </pre>
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* CTA Section */}
      <section className="max-w-6xl mx-auto px-6 py-24 border-t border-border">
        <div className="rounded-2xl border border-accent/30 bg-primary/5 p-12 text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-foreground mb-4">Ready to Explore Logic Programming?</h2>
          <p className="text-foreground/60 mb-8 max-w-xl mx-auto">
            Download GIC, explore the examples, and start building powerful logic programs.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <a href="https://github.com/LucasGrasso/gic" target="_blank" rel="noopener noreferrer">
              <Button size="lg" className="w-full sm:w-auto bg-primary hover:bg-primary/90">
                View GitHub Repository
                <ChevronRight className="w-4 h-4 ml-2" />
              </Button>
            </a>
            <a href="https://github.com/LucasGrasso/gic#readme" target="_blank" rel="noopener noreferrer">
              <Button size="lg" variant="outline" className="w-full sm:w-auto bg-transparent">
                Read Documentation
              </Button>
            </a>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-border bg-card/50">
        <div className="max-w-6xl mx-auto px-6 py-12">
          <div className="grid md:grid-cols-4 gap-8 mb-8">
            <div>
              <div className="flex items-center gap-3 mb-4">
                <Image
                  src="/logo.png"
                  alt="GIC Logo"
                  width={32}
                  height={32}
                  className="rounded-lg"
                />
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
                  <a
                    href="https://github.com/LucasGrasso/gic/blob/master/BuiltIns.md"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:text-foreground transition"
                  >
                    Built-ins
                  </a>
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
    </div>
  )
}

import Image from 'next/image'
import { ChevronRight, Code2, Zap, BookOpen } from 'lucide-react'
import { Button } from '@/components/ui/button'
import Navbar from '@/components/navbar/Navbar'
import Footer from '@/components/footer/Footer'

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-background via-background to-secondary/20 dark:from-background dark:via-background dark:to-secondary/10">
      <Navbar />

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

          <h1 className="text-5xl md:text-7xl font-bold text-balance text-foreground leading-tight">
            The progamming language for
            <span className="block text-primary">First Order Logic*</span>
          </h1>

          <p className="text-lg md:text-m text-foreground/60 max-w-2xl mx-auto text-balance">
            * You should check PROLOG out.
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
        <div className="grid md:grid-cols-2 gap-12 items-start">
          <div>
            <h2 className="text-4xl font-bold text-foreground mb-6">FOL-like Syntax</h2>
            <p className="text-foreground/60 mb-6">
              GIC uses a first-order logic syntax. If <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">P</code> is a predicate (Uppercase) and <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">t1, ..., tn</code> are terms (variables or function applications in lowercase), then L-Formulas are defined as:
            </p>

            <div className="space-y-4 text-foreground/80">
              <p>
                A <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">.gic</code> file consists of a set of L-Formulas separated by <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">.</code>
              </p>
              <p>
                You can use either ASCII operators (<code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">and</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">or</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">impl</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">not</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">exists</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">forall</code>) or Unicode symbols (<code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">∧</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">∨</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">⇒</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">¬</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">∃</code>, <code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">∀</code>) interchangeably.
              </p>
              <p>
                Universal quantifiers (<code className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono">forall</code>) may be left implicit.
              </p>
            </div>
          </div>

          <div className="rounded-xl border border-border bg-card/50 backdrop-blur p-8">
            <h3 className="text-sm font-semibold text-foreground/60 uppercase tracking-wider mb-4">Grammar</h3>
            <pre className="font-mono text-sm text-foreground/80 overflow-x-auto">
              <code>{`L-Formulas f ::=
    P(t1, ..., tn)   // Predicate
  | bottom           // ⊥
  | f and f          // f ∧ f
  | f or f           // f ∨ f
  | f impl f         // f ⇒ f
  | not f            // ¬ f
  | exists X. f      // ∃ X. f
  | forall X. f      // ∀ X. f`}</code>
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
              code: `(Father(A,B) and Father(B,C)) ⇒ Grandpa(A,C).`,
              description: 'Define and query complex relationships with logical rules.',
            },
            {
              title: 'List Operations',
              code: `Reverse(XS,XS) ⇒ Palindrome(Xs)`,
              description: 'Work with recursive data structures and solve constraints.',
            },
            {
              title: 'Fibonacci',
              code: `Fib(0,0).
Fib(1,1).
(
	Gt(N,1) and
	Sub(N,1,N1) and
	Fib(N1,F1) and
	Sub(N,2,N2) and
	Fib(N2,F2) and
	Add(F1,F2,F)
) impl Fib(N,F).`,
              description: 'Express complex mathematical expressions with logic formulas.',
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

      <Footer />
    </div>
  )
}

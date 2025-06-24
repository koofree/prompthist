'use client';

import {
  ArrowRight,
  Download,
  Github,
  HardDrive,
  History,
  Lock,
  MonitorSpeaker,
  Shield,
  Star,
  Zap,
} from 'lucide-react';

export default function HomePage() {
  return (
    <div className='min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-gray-900'>
      {/* Navigation */}
      <nav className='border-b border-white/10 bg-black/20 backdrop-blur-sm'>
        <div className='mx-auto max-w-7xl px-4 sm:px-6 lg:px-8'>
          <div className='flex items-center justify-between py-4'>
            <div className='flex items-center space-x-2'>
              <div className='flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-r from-blue-500 to-purple-600'>
                <History className='h-5 w-5 text-white' />
              </div>
              <span className='text-2xl font-bold text-white'>PromptHist</span>
            </div>
            <div className='flex items-center space-x-4'>
              <a
                href='https://github.com/yourusername/prompthist'
                target='_blank'
                rel='noopener noreferrer'
                className='flex items-center space-x-2 rounded-lg border border-white/20 px-4 py-2 text-white transition-colors hover:bg-white/10'
              >
                <Github className='h-4 w-4' />
                <span>GitHub</span>
              </a>
              <a
                href='/dashboard'
                className='flex items-center space-x-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700'
              >
                <span>Dashboard</span>
              </a>
              <button className='rounded-lg bg-green-600 px-4 py-2 text-white transition-colors hover:bg-green-700'>
                Download
              </button>
            </div>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className='relative overflow-hidden'>
        <div className='mx-auto max-w-7xl px-4 py-24 sm:px-6 lg:px-8'>
          <div className='animate-fade-in text-center'>
            <h1 className='mb-6 text-5xl font-bold text-white md:text-7xl'>
              <span className='bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent'>
                PromptHist
              </span>
            </h1>
            <p className='mx-auto mb-8 max-w-3xl text-xl text-gray-300 md:text-2xl'>
              Security-first, offline AI prompt history and auto-completion
              tool. Keep your prompts private while boosting productivity.
            </p>
            <div className='flex flex-col items-center justify-center space-y-4 sm:flex-row sm:space-x-6 sm:space-y-0'>
              <button className='animate-pulse-glow flex transform items-center space-x-2 rounded-xl bg-gradient-to-r from-blue-500 to-purple-600 px-8 py-4 font-semibold text-white transition-all hover:scale-105 hover:from-blue-600 hover:to-purple-700'>
                <Download className='h-5 w-5' />
                <span>Get Started Free</span>
                <ArrowRight className='h-5 w-5' />
              </button>
              <button className='rounded-xl border border-white/30 px-8 py-4 font-semibold text-white transition-all hover:bg-white/10'>
                View Demo
              </button>
            </div>
          </div>
        </div>

        {/* Floating elements for visual interest */}
        <div className='absolute left-10 top-1/4 h-16 w-16 animate-pulse rounded-full bg-blue-500/20 blur-xl'></div>
        <div
          className='absolute right-20 top-1/3 h-24 w-24 animate-pulse rounded-full bg-purple-500/20 blur-xl'
          style={{ animationDelay: '1s' }}
        ></div>
      </section>

      {/* Security-First Banner */}
      <section className='border-y border-green-500/20 bg-gradient-to-r from-green-500/10 to-blue-500/10'>
        <div className='mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8'>
          <div className='flex items-center justify-center space-x-4 text-center'>
            <Shield className='h-8 w-8 text-green-400' />
            <div>
              <h3 className='text-xl font-semibold text-white'>
                100% Offline & Secure
              </h3>
              <p className='text-gray-300'>
                Zero data transmission • Military-grade encryption • Your data
                never leaves your device
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className='py-24'>
        <div className='mx-auto max-w-7xl px-4 sm:px-6 lg:px-8'>
          <div className='mb-16 text-center'>
            <h2 className='mb-4 text-4xl font-bold text-white'>
              Powerful Features
            </h2>
            <p className='mx-auto max-w-2xl text-xl text-gray-300'>
              Everything you need to manage and enhance your AI prompt workflows
            </p>
          </div>

          <div className='grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-3'>
            {/* Feature 1 */}
            <div className='animate-slide-in-left rounded-xl border border-white/10 bg-white/5 p-6 backdrop-blur-sm transition-all hover:bg-white/10'>
              <div className='mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-r from-blue-500 to-cyan-500'>
                <History className='h-6 w-6 text-white' />
              </div>
              <h3 className='mb-2 text-xl font-semibold text-white'>
                Prompt History Tracking
              </h3>
              <p className='text-gray-300'>
                System-wide monitoring captures prompts from ChatGPT, Claude,
                Cursor, and other AI tools with real-time encrypted storage.
              </p>
            </div>

            {/* Feature 2 */}
            <div
              className='animate-slide-in-left rounded-xl border border-white/10 bg-white/5 p-6 backdrop-blur-sm transition-all hover:bg-white/10'
              style={{ animationDelay: '0.1s' }}
            >
              <div className='mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-r from-purple-500 to-pink-500'>
                <Zap className='h-6 w-6 text-white' />
              </div>
              <h3 className='mb-2 text-xl font-semibold text-white'>
                Smart Auto-completion
              </h3>
              <p className='text-gray-300'>
                Context-aware suggestions with priority-based ranking, fuzzy
                matching, and local AI processing via Ollama integration.
              </p>
            </div>

            {/* Feature 3 */}
            <div
              className='animate-slide-in-left rounded-xl border border-white/10 bg-white/5 p-6 backdrop-blur-sm transition-all hover:bg-white/10'
              style={{ animationDelay: '0.2s' }}
            >
              <div className='mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-r from-green-500 to-emerald-500'>
                <Lock className='h-6 w-6 text-white' />
              </div>
              <h3 className='mb-2 text-xl font-semibold text-white'>
                Military-Grade Security
              </h3>
              <p className='text-gray-300'>
                AES-256 encryption, encrypted memory processing, secure
                deletion, and zero network access requirements.
              </p>
            </div>

            {/* Feature 4 */}
            <div
              className='animate-slide-in-left rounded-xl border border-white/10 bg-white/5 p-6 backdrop-blur-sm transition-all hover:bg-white/10'
              style={{ animationDelay: '0.3s' }}
            >
              <div className='mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-r from-orange-500 to-red-500'>
                <Star className='h-6 w-6 text-white' />
              </div>
              <h3 className='mb-2 text-xl font-semibold text-white'>
                Star & Organize
              </h3>
              <p className='text-gray-300'>
                Mark important prompts, add custom tags, and organize your
                prompt library with powerful search and bulk operations.
              </p>
            </div>

            {/* Feature 5 */}
            <div
              className='animate-slide-in-left rounded-xl border border-white/10 bg-white/5 p-6 backdrop-blur-sm transition-all hover:bg-white/10'
              style={{ animationDelay: '0.4s' }}
            >
              <div className='mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-r from-teal-500 to-cyan-500'>
                <HardDrive className='h-6 w-6 text-white' />
              </div>
              <h3 className='mb-2 text-xl font-semibold text-white'>
                Local-Only Storage
              </h3>
              <p className='text-gray-300'>
                All data stays on your machine with encrypted local backups, no
                cloud dependency, and complete privacy control.
              </p>
            </div>

            {/* Feature 6 */}
            <div
              className='animate-slide-in-left rounded-xl border border-white/10 bg-white/5 p-6 backdrop-blur-sm transition-all hover:bg-white/10'
              style={{ animationDelay: '0.5s' }}
            >
              <div className='mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-r from-indigo-500 to-purple-500'>
                <MonitorSpeaker className='h-6 w-6 text-white' />
              </div>
              <h3 className='mb-2 text-xl font-semibold text-white'>
                Cross-Platform Support
              </h3>
              <p className='text-gray-300'>
                Works with web-based and desktop AI applications including
                browser extensions and API integrations.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Supported Applications */}
      <section className='bg-black/20 py-24'>
        <div className='mx-auto max-w-7xl px-4 sm:px-6 lg:px-8'>
          <div className='mb-16 text-center'>
            <h2 className='mb-4 text-4xl font-bold text-white'>
              Supported Applications
            </h2>
            <p className='text-xl text-gray-300'>
              Seamlessly integrates with your favorite AI tools
            </p>
          </div>

          <div className='grid grid-cols-2 gap-6 md:grid-cols-4 lg:grid-cols-6'>
            {[
              'ChatGPT',
              'Claude',
              'Cursor',
              'Grok',
              'Perplexity',
              'Ollama',
              'VS Code',
              'JetBrains',
              'LM Studio',
              'GPT4All',
              'LocalAI',
              'Terminal',
            ].map((app, index) => (
              <div
                key={app}
                className='rounded-lg border border-white/10 bg-white/5 p-4 text-center backdrop-blur-sm transition-all hover:bg-white/10'
                style={{ animationDelay: `${index * 0.1}s` }}
              >
                <div className='mx-auto mb-2 h-8 w-8 rounded-md bg-gradient-to-r from-blue-500 to-purple-500'></div>
                <span className='text-sm font-medium text-white'>{app}</span>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className='py-24'>
        <div className='mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8'>
          <h2 className='mb-6 text-4xl font-bold text-white'>
            Ready to Enhance Your AI Workflow?
          </h2>
          <p className='mb-8 text-xl text-gray-300'>
            Download PromptHist today and experience secure, intelligent prompt
            management.
          </p>
          <div className='flex flex-col space-y-4 sm:flex-row sm:space-x-4 sm:space-y-0'>
            <button className='flex items-center justify-center space-x-2 rounded-lg bg-green-600 px-8 py-3 text-lg font-semibold text-white transition-colors hover:bg-green-700'>
              <Download className='h-5 w-5' />
              <span>Download PromptHist</span>
            </button>
            <a
              href='/dashboard'
              className='flex items-center justify-center space-x-2 rounded-lg border border-white/20 px-8 py-3 text-lg font-semibold text-white transition-colors hover:bg-white/10'
            >
              <span>View Dashboard</span>
            </a>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className='border-t border-white/10 bg-black/40 py-8 backdrop-blur-sm'>
        <div className='mx-auto max-w-7xl px-4 sm:px-6 lg:px-8'>
          <div className='flex flex-col items-center justify-between md:flex-row'>
            <div className='mb-4 flex items-center space-x-2 md:mb-0'>
              <div className='flex h-6 w-6 items-center justify-center rounded-md bg-gradient-to-r from-blue-500 to-purple-600'>
                <History className='h-4 w-4 text-white' />
              </div>
              <span className='font-semibold text-white'>PromptHist</span>
            </div>
            <div className='text-sm text-gray-400'>
              © 2024 PromptHist. Privacy-first prompt management.
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
}

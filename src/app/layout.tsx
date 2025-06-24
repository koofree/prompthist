import type { Metadata } from 'next';
import React from 'react';
import './globals.css';

export const metadata: Metadata = {
  title: 'PromptHist',
  description:
    'AI Prompt History & Insights Tool - Security-first, offline prompt management',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang='en' className='dark'>
      <body className='antialiased'>{children}</body>
    </html>
  );
}

'use client';

import { invoke } from '@tauri-apps/api/core';
import {
  AlertCircle,
  BarChart3,
  CheckCircle,
  Copy,
  EyeOff,
  Filter,
  Pause,
  Play,
  RefreshCw,
  Search,
  Settings,
  Star,
  Trash2,
} from 'lucide-react';
import { useEffect, useState } from 'react';

interface PromptEntry {
  id: string;
  content: string;
  application: string;
  timestamp: string;
  starred: boolean;
  tags: string[];
  usage_count: number;
  is_encrypted: boolean;
}

interface PromptStats {
  total_prompts: number;
  applications: number;
  starred_count: number;
  most_used_prompts: PromptEntry[];
  recent_activity: PromptEntry[];
}

interface PromptFilter {
  application?: string;
  starred?: boolean;
  tags?: string[];
  search_text?: string;
  start_date?: string;
  end_date?: string;
}

export default function DashboardPage() {
  const [prompts, setPrompts] = useState<PromptEntry[]>([]);
  const [stats, setStats] = useState<PromptStats | null>(null);
  const [loading, setLoading] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedPrompt, setSelectedPrompt] = useState<PromptEntry | null>(
    null
  );
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [filter, setFilter] = useState<PromptFilter>({});
  const [showFilters, setShowFilters] = useState(false);
  const [notification, setNotification] = useState<{
    type: 'success' | 'error';
    message: string;
  } | null>(null);

  useEffect(() => {
    loadPrompts();
    loadStats();
    checkMonitoringStatus();
  }, [filter]);

  const loadPrompts = async () => {
    try {
      setLoading(true);
      const result = await invoke<PromptEntry[]>('get_prompts', {
        filter: filter || {},
        limit: 50,
        offset: 0,
      });
      setPrompts(result);
    } catch (error) {
      showNotification('error', `Failed to load prompts: ${String(error)}`);
    } finally {
      setLoading(false);
    }
  };

  const loadStats = async () => {
    try {
      const result = await invoke<PromptStats>('get_prompt_stats');
      setStats(result);
    } catch (error) {
      console.error('Failed to load stats:', error);
    }
  };

  const checkMonitoringStatus = async () => {
    try {
      const result = await invoke<boolean>('get_monitoring_status');
      setIsMonitoring(result);
    } catch (error) {
      console.error('Failed to check monitoring status:', error);
    }
  };

  const toggleMonitoring = async () => {
    try {
      if (isMonitoring) {
        await invoke('stop_monitoring');
        setIsMonitoring(false);
        showNotification('success', 'Monitoring stopped');
      } else {
        await invoke('start_monitoring');
        setIsMonitoring(true);
        showNotification('success', 'Monitoring started');
      }
    } catch (error) {
      showNotification('error', `Failed to toggle monitoring: ${String(error)}`);
    }
  };

  const toggleStar = async (promptId: string) => {
    try {
      const prompt = prompts.find(p => p.id === promptId);
      if (!prompt) return;

      await invoke('update_prompt', {
        id: promptId,
        starred: !prompt.starred,
        tags: prompt.tags,
      });

      setPrompts(prev =>
        prev.map(p => (p.id === promptId ? { ...p, starred: !p.starred } : p))
      );

      showNotification(
        'success',
        prompt.starred ? 'Prompt unstarred' : 'Prompt starred'
      );
    } catch (error) {
      showNotification('error', `Failed to update prompt: ${String(error)}`);
    }
  };

  const deletePrompt = async (promptId: string) => {
    if (!confirm('Are you sure you want to delete this prompt?')) return;

    try {
      await invoke('delete_prompt', { id: promptId });
      setPrompts(prev => prev.filter(p => p.id !== promptId));
      if (selectedPrompt?.id === promptId) {
        setSelectedPrompt(null);
      }
      showNotification('success', 'Prompt deleted');
    } catch (error) {
      showNotification('error', `Failed to delete prompt: ${String(error)}`);
    }
  };

  const copyToClipboard = async (content: string) => {
    try {
      await navigator.clipboard.writeText(content);
      showNotification('success', 'Copied to clipboard');
    } catch (error) {
      showNotification('error', 'Failed to copy to clipboard');
    }
  };

  const searchPrompts = async () => {
    if (!searchTerm.trim()) {
      await loadPrompts();
      return;
    }

    try {
      setLoading(true);
      const result = await invoke<PromptEntry[]>('search_prompts', {
        query: searchTerm,
        limit: 100,
      });
      setPrompts(result);
    } catch (error) {
      showNotification('error', `Search failed: ${String(error)}`);
    } finally {
      setLoading(false);
    }
  };

  const showNotification = (type: 'success' | 'error', message: string) => {
    setNotification({ type, message });
    setTimeout(() => setNotification(null), 3000);
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return isNaN(date.getTime()) ? 'Invalid Date' : date.toLocaleString();
  };

  const getApplicationColor = (app: string) => {
    const colors: Record<string, string> = {
      ChatGPT: 'bg-green-500',
      Claude: 'bg-orange-500',
      Cursor: 'bg-blue-500',
      Ollama: 'bg-purple-500',
      clipboard: 'bg-gray-500',
      web: 'bg-cyan-500',
      desktop: 'bg-indigo-500',
    };
    return colors[app] || 'bg-gray-500';
  };

  return (
    <div className='min-h-screen bg-gradient-to-br from-slate-900 via-indigo-900 to-slate-900 animate-fade-in'>
      {/* Header */}
      <header className='border-b border-white/10 bg-black/30 backdrop-blur-md shadow-lg'>
        <div className='mx-auto max-w-7xl px-4 sm:px-6 lg:px-8'>
          <div className='flex items-center justify-between py-4'>
            <div className='flex items-center space-x-4'>
              <h1 className='text-3xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent'>
                Prompt History Dashboard
              </h1>
              <div className='flex items-center space-x-2'>
                <button
                  onClick={toggleMonitoring}
                  className={`flex items-center space-x-2 rounded-lg px-3 py-1 text-sm font-medium transition-colors ${
                    isMonitoring
                      ? 'bg-green-600 text-white hover:bg-green-700'
                      : 'bg-gray-600 text-white hover:bg-gray-700'
                  }`}
                >
                  {isMonitoring ? (
                    <Pause className='h-4 w-4' />
                  ) : (
                    <Play className='h-4 w-4' />
                  )}
                  <span>
                    {isMonitoring ? 'Monitoring Active' : 'Monitoring Inactive'}
                  </span>
                </button>
              </div>
            </div>

            <div className='flex items-center space-x-4'>
              <button
                onClick={() => setShowFilters(!showFilters)}
                className='flex items-center space-x-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700'
              >
                <Filter className='h-4 w-4' />
                <span>Filters</span>
              </button>
              <a
                href='/settings'
                className='flex items-center space-x-2 rounded-lg bg-gray-600 px-4 py-2 text-white transition-colors hover:bg-gray-700'
              >
                <Settings className='h-4 w-4' />
                <span>Settings</span>
              </a>
            </div>
          </div>
        </div>
      </header>

      {/* Notification */}
      {notification && (
        <div
          className={`fixed right-4 top-4 z-50 flex items-center space-x-2 rounded-lg px-4 py-2 text-white ${
            notification.type === 'success' ? 'bg-green-600' : 'bg-red-600'
          }`}
        >
          {notification.type === 'success' ? (
            <CheckCircle className='h-4 w-4' />
          ) : (
            <AlertCircle className='h-4 w-4' />
          )}
          <span>{notification.message}</span>
        </div>
      )}

      <div className='mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8'>
        {/* Stats Cards */}
        {stats && (
          <div className='mb-6 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4 animate-slide-in-left'>
            <div className='rounded-xl border border-white/20 bg-gradient-to-br from-white/10 to-white/5 p-6 backdrop-blur-md shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-105'>
              <div className='flex items-center space-x-2'>
                <BarChart3 className='h-5 w-5 text-blue-400' />
                <span className='text-sm text-gray-300'>Total Prompts</span>
              </div>
              <p className='text-2xl font-bold text-white'>
                {typeof stats.total_prompts === 'number' ? stats.total_prompts : 0}
              </p>
            </div>

            <div className='rounded-xl border border-white/20 bg-gradient-to-br from-white/10 to-white/5 p-6 backdrop-blur-md shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-105'>
              <div className='flex items-center space-x-2'>
                <Settings className='h-5 w-5 text-green-400' />
                <span className='text-sm text-gray-300'>Applications</span>
              </div>
              <p className='text-2xl font-bold text-white'>
                {typeof stats.applications === 'number' ? stats.applications : 0}
              </p>
            </div>

            <div className='rounded-xl border border-white/20 bg-gradient-to-br from-white/10 to-white/5 p-6 backdrop-blur-md shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-105'>
              <div className='flex items-center space-x-2'>
                <Star className='h-5 w-5 text-yellow-400' />
                <span className='text-sm text-gray-300'>Starred</span>
              </div>
              <p className='text-2xl font-bold text-white'>
                {typeof stats.starred_count === 'number' ? stats.starred_count : 0}
              </p>
            </div>

            <div className='rounded-xl border border-white/20 bg-gradient-to-br from-white/10 to-white/5 p-6 backdrop-blur-md shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-105'>
              <div className='flex items-center space-x-2'>
                <RefreshCw className='h-5 w-5 text-purple-400' />
                <span className='text-sm text-gray-300'>Recent Activity</span>
              </div>
              <p className='text-2xl font-bold text-white'>
                {typeof stats.recent_activity?.length === 'number' ? stats.recent_activity.length : 0}
              </p>
            </div>
          </div>
        )}

        {/* Search and Filters */}
        <div className='mb-6 space-y-4'>
          <div className='flex space-x-4'>
            <div className='flex-1'>
              <div className='relative'>
                <Search className='absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400' />
                <input
                  type='text'
                  placeholder='Search prompts...'
                  value={searchTerm}
                  onChange={e => setSearchTerm(e.target.value)}
                  onKeyDown={e => e.key === 'Enter' && searchPrompts()}
                  className='w-full rounded-xl border border-white/20 bg-white/10 py-3 pl-10 pr-4 text-white placeholder-gray-400 backdrop-blur-md focus:border-blue-400 focus:outline-none focus:ring-2 focus:ring-blue-400/50 transition-all duration-300'
                />
              </div>
            </div>
            <button
              onClick={searchPrompts}
              className='rounded-xl bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-3 text-white font-medium transition-all duration-300 hover:from-blue-700 hover:to-blue-800 hover:shadow-lg hover:scale-105'
            >
              Search
            </button>
          </div>

          {/* Advanced Filters */}
          {showFilters && (
            <div className='rounded-xl border border-white/20 bg-gradient-to-br from-white/10 to-white/5 p-6 backdrop-blur-md shadow-xl animate-fade-in'>
              <div className='grid grid-cols-1 gap-4 md:grid-cols-3'>
                <div>
                  <label className='mb-2 block text-sm font-medium text-gray-300'>
                    Application
                  </label>
                  <select
                    value={filter.application || ''}
                    onChange={e =>
                      setFilter(prev => ({
                        ...prev,
                        application: e.target.value || undefined,
                      }))
                    }
                    className='w-full rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-white backdrop-blur-sm focus:border-blue-500 focus:outline-none'
                  >
                    <option value=''>All Applications</option>
                    <option value='ChatGPT'>ChatGPT</option>
                    <option value='Claude'>Claude</option>
                    <option value='Cursor'>Cursor</option>
                    <option value='Ollama'>Ollama</option>
                    <option value='clipboard'>Clipboard</option>
                  </select>
                </div>

                <div>
                  <label className='mb-2 block text-sm font-medium text-gray-300'>
                    Status
                  </label>
                  <select
                    value={
                      filter.starred === undefined
                        ? ''
                        : filter.starred.toString()
                    }
                    onChange={e =>
                      setFilter(prev => ({
                        ...prev,
                        starred:
                          e.target.value === ''
                            ? undefined
                            : e.target.value === 'true',
                      }))
                    }
                    className='w-full rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-white backdrop-blur-sm focus:border-blue-500 focus:outline-none'
                  >
                    <option value=''>All Prompts</option>
                    <option value='true'>Starred Only</option>
                    <option value='false'>Unstarred Only</option>
                  </select>
                </div>

                <div className='flex items-end space-x-2'>
                  <button
                    onClick={() => setFilter({})}
                    className='rounded-lg bg-gray-600 px-4 py-2 text-white transition-colors hover:bg-gray-700'
                  >
                    Clear Filters
                  </button>
                  <button
                    onClick={loadPrompts}
                    className='rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700'
                  >
                    Apply
                  </button>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Main Content */}
        <div className='grid grid-cols-1 gap-6 lg:grid-cols-3'>
          {/* Prompt List */}
          <div className='lg:col-span-2'>
            <div className='rounded-xl border border-white/20 bg-gradient-to-br from-white/10 to-white/5 backdrop-blur-md shadow-xl'>
              <div className='border-b border-white/10 p-4'>
                <h2 className='text-lg font-semibold text-white'>
                  Prompts ({prompts.length})
                </h2>
              </div>

              <div className='max-h-96 overflow-y-auto custom-scrollbar'>
                {loading ? (
                  <div className='flex items-center justify-center p-8'>
                    <RefreshCw className='h-6 w-6 animate-spin text-blue-400' />
                    <span className='ml-2 text-gray-300'>Loading...</span>
                  </div>
                ) : prompts.length === 0 ? (
                  <div className='p-8 text-center text-gray-300'>
                    <p>No prompts found</p>
                    <p className='text-sm'>
                      Start using AI applications to see your prompt history
                      here
                    </p>
                  </div>
                ) : (
                  <div className='divide-y divide-white/10'>
                    {prompts.map(prompt => (
                      <div
                        key={prompt.id}
                        className={`cursor-pointer p-4 transition-all duration-300 hover:bg-white/10 hover:scale-[1.02] ${
                          selectedPrompt?.id === prompt.id
                            ? 'bg-gradient-to-r from-blue-500/30 to-purple-500/20 border-l-4 border-blue-400'
                            : ''
                        }`}
                        onClick={() => setSelectedPrompt(prompt)}
                      >
                        <div className='flex items-start justify-between'>
                          <div className='min-w-0 flex-1'>
                            <div className='mb-2 flex items-center space-x-2'>
                              <span
                                className={`inline-block h-2 w-2 rounded-full ${getApplicationColor(prompt.application)}`}
                              />
                              <span className='text-sm font-medium text-gray-300'>
                                {prompt.application}
                              </span>
                              <span className='text-xs text-gray-500'>
                                {formatDate(prompt.timestamp)}
                              </span>
                              {prompt.is_encrypted && (
                                <EyeOff className='h-3 w-3 text-yellow-400' />
                              )}
                            </div>
                            <p className='truncate text-white'>
                              {prompt.content}
                            </p>
                            {prompt.tags && prompt.tags.length > 0 && (
                              <div className='mt-2 flex flex-wrap gap-1'>
                                {prompt.tags.map((tag, index) => (
                                  <span
                                    key={index}
                                    className='inline-block rounded-full bg-blue-500/20 px-2 py-1 text-xs text-blue-300'
                                  >
                                    {String(tag)}
                                  </span>
                                ))}
                              </div>
                            )}
                          </div>

                          <div className='ml-4 flex items-center space-x-2'>
                            <button
                              onClick={e => {
                                e.stopPropagation();
                                toggleStar(prompt.id);
                              }}
                              className={`rounded p-1 transition-colors ${
                                prompt.starred
                                  ? 'text-yellow-400 hover:text-yellow-300'
                                  : 'text-gray-400 hover:text-yellow-400'
                              }`}
                            >
                              <Star
                                className={`h-4 w-4 ${prompt.starred ? 'fill-current' : ''}`}
                              />
                            </button>
                            <button
                              onClick={e => {
                                e.stopPropagation();
                                copyToClipboard(prompt.content);
                              }}
                              className='p-1 text-gray-400 transition-colors hover:text-white'
                            >
                              <Copy className='h-4 w-4' />
                            </button>
                            <button
                              onClick={e => {
                                e.stopPropagation();
                                deletePrompt(prompt.id);
                              }}
                              className='p-1 text-gray-400 transition-colors hover:text-red-400'
                            >
                              <Trash2 className='h-4 w-4' />
                            </button>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          </div>

          {/* Prompt Details */}
          <div className='lg:col-span-1'>
            <div className='rounded-xl border border-white/20 bg-gradient-to-br from-gray-800/50 to-gray-900/50 backdrop-blur-md shadow-xl'>
              <div className='border-b border-white/10 p-4'>
                <h2 className='text-lg font-semibold text-white'>
                  Prompt Details
                </h2>
              </div>

              {selectedPrompt ? (
                <div className='space-y-4 p-4'>
                  <div>
                    <label className='mb-1 block text-sm font-medium text-gray-300'>
                      Content
                    </label>
                    <div className='rounded-xl border border-white/20 bg-gray-900/80 p-4 backdrop-blur-sm shadow-inner'>
                      <p className='whitespace-pre-wrap text-gray-100 leading-relaxed'>
                        {selectedPrompt.content}
                      </p>
                    </div>
                  </div>

                  <div>
                    <label className='mb-1 block text-sm font-medium text-gray-300'>
                      Application
                    </label>
                    <div className='flex items-center space-x-2'>
                      <span
                        className={`inline-block h-3 w-3 rounded-full ${getApplicationColor(selectedPrompt.application)}`}
                      />
                      <span className='text-gray-100 font-medium'>
                        {selectedPrompt.application}
                      </span>
                    </div>
                  </div>

                  <div>
                    <label className='mb-1 block text-sm font-medium text-gray-300'>
                      Timestamp
                    </label>
                    <p className='text-gray-100 font-medium'>
                      {formatDate(selectedPrompt.timestamp)}
                    </p>
                  </div>

                  <div>
                    <label className='mb-1 block text-sm font-medium text-gray-300'>
                      Usage Count
                    </label>
                    <p className='text-gray-100 font-medium'>{typeof selectedPrompt.usage_count === 'number' ? selectedPrompt.usage_count : 0}</p>
                  </div>

                  {selectedPrompt.tags && selectedPrompt.tags.length > 0 && (
                    <div>
                      <label className='mb-1 block text-sm font-medium text-gray-300'>
                        Tags
                      </label>
                      <div className='flex flex-wrap gap-1'>
                        {selectedPrompt.tags.map((tag, index) => (
                          <span
                            key={index}
                            className='inline-block rounded-full bg-blue-500/20 px-2 py-1 text-xs text-blue-300'
                          >
                            {String(tag)}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}

                  <div className='flex space-x-2 pt-4'>
                    <button
                      onClick={() => copyToClipboard(selectedPrompt.content)}
                      className='flex flex-1 items-center justify-center space-x-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700'
                    >
                      <Copy className='h-4 w-4' />
                      <span>Copy</span>
                    </button>
                    <button
                      onClick={() => toggleStar(selectedPrompt.id)}
                      className={`flex items-center justify-center space-x-2 rounded-lg px-4 py-2 transition-colors ${
                        selectedPrompt.starred
                          ? 'bg-yellow-600 text-white hover:bg-yellow-700'
                          : 'bg-gray-600 text-white hover:bg-gray-700'
                      }`}
                    >
                      <Star
                        className={`h-4 w-4 ${selectedPrompt.starred ? 'fill-current' : ''}`}
                      />
                    </button>
                  </div>
                </div>
              ) : (
                <div className='p-8 text-center text-gray-300'>
                  <p>Select a prompt to view details</p>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

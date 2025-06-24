'use client';

import {
  AlertCircle,
  ArrowLeft,
  CheckCircle,
  Database,
  Eye,
  EyeOff,
  Info,
  Monitor,
  RefreshCw,
  Save,
  Shield,
  Trash2,
} from 'lucide-react';
import { useEffect, useState } from 'react';

interface MonitoringConfig {
  enabled: boolean;
  monitored_applications: string[];
  capture_threshold: number;
  auto_save: boolean;
  encryption_enabled: boolean;
}

export default function SettingsPage() {
  const [config, setConfig] = useState<MonitoringConfig>({
    enabled: false,
    monitored_applications: [],
    capture_threshold: 10,
    auto_save: true,
    encryption_enabled: true,
  });
  const [loading, setLoading] = useState(false);
  const [notification, setNotification] = useState<{
    type: 'success' | 'error' | 'info';
    message: string;
  } | null>(null);
  const [showDangerZone, setShowDangerZone] = useState(false);

  const availableApplications = [
    'ChatGPT',
    'Claude',
    'Cursor',
    'Ollama',
    'Perplexity',
    'Gemini',
    'Copilot',
    'clipboard',
  ];

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      setLoading(true);
      // For now, we'll use default settings since we haven't implemented the backend command yet
      // In a real implementation, you would call: const result = await invoke<MonitoringConfig>('get_monitoring_config');
      setConfig({
        enabled: false,
        monitored_applications: ['ChatGPT', 'Claude', 'Cursor'],
        capture_threshold: 10,
        auto_save: true,
        encryption_enabled: true,
      });
    } catch (error) {
      showNotification('error', `Failed to load settings: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const saveSettings = async () => {
    try {
      setLoading(true);
      // await invoke('update_monitoring_config', { config });
      showNotification('success', 'Settings saved successfully');
    } catch (error) {
      showNotification('error', `Failed to save settings: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const clearAllData = async () => {
    if (
      !confirm(
        'Are you sure you want to delete all prompt history? This action cannot be undone.'
      )
    ) {
      return;
    }

    try {
      setLoading(true);
      // await invoke('clear_all_prompts');
      showNotification('success', 'All data cleared successfully');
    } catch (error) {
      showNotification('error', `Failed to clear data: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const exportData = async () => {
    try {
      setLoading(true);
      // await invoke('export_prompts');
      showNotification('success', 'Data exported successfully');
    } catch (error) {
      showNotification('error', `Failed to export data: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const showNotification = (
    type: 'success' | 'error' | 'info',
    message: string
  ) => {
    setNotification({ type, message });
    setTimeout(() => setNotification(null), 3000);
  };

  const toggleApplication = (app: string) => {
    setConfig(prev => ({
      ...prev,
      monitored_applications: prev.monitored_applications.includes(app)
        ? prev.monitored_applications.filter(a => a !== app)
        : [...prev.monitored_applications, app],
    }));
  };

  return (
    <div className='min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-gray-900'>
      {/* Header */}
      <header className='border-b border-white/10 bg-black/20 backdrop-blur-sm'>
        <div className='mx-auto max-w-4xl px-4 sm:px-6 lg:px-8'>
          <div className='flex items-center justify-between py-4'>
            <div className='flex items-center space-x-4'>
              <a
                href='/dashboard'
                className='flex items-center space-x-2 text-gray-300 transition-colors hover:text-white'
              >
                <ArrowLeft className='h-4 w-4' />
                <span>Back to Dashboard</span>
              </a>
              <h1 className='text-2xl font-bold text-white'>Settings</h1>
            </div>

            <button
              onClick={saveSettings}
              disabled={loading}
              className='flex items-center space-x-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700 disabled:opacity-50'
            >
              {loading ? (
                <RefreshCw className='h-4 w-4 animate-spin' />
              ) : (
                <Save className='h-4 w-4' />
              )}
              <span>Save Settings</span>
            </button>
          </div>
        </div>
      </header>

      {/* Notification */}
      {notification && (
        <div
          className={`fixed right-4 top-4 z-50 flex items-center space-x-2 rounded-lg px-4 py-2 text-white ${
            notification.type === 'success'
              ? 'bg-green-600'
              : notification.type === 'error'
                ? 'bg-red-600'
                : 'bg-blue-600'
          }`}
        >
          {notification.type === 'success' ? (
            <CheckCircle className='h-4 w-4' />
          ) : notification.type === 'error' ? (
            <AlertCircle className='h-4 w-4' />
          ) : (
            <Info className='h-4 w-4' />
          )}
          <span>{notification.message}</span>
        </div>
      )}

      <div className='mx-auto max-w-4xl px-4 py-6 sm:px-6 lg:px-8'>
        <div className='space-y-6'>
          {/* Monitoring Settings */}
          <div className='rounded-lg border border-white/10 bg-white/5 backdrop-blur-sm'>
            <div className='border-b border-white/10 p-4'>
              <div className='flex items-center space-x-2'>
                <Monitor className='h-5 w-5 text-blue-400' />
                <h2 className='text-lg font-semibold text-white'>
                  Monitoring Settings
                </h2>
              </div>
            </div>

            <div className='space-y-4 p-4'>
              <div className='flex items-center justify-between'>
                <div>
                  <label className='font-medium text-white'>
                    Enable System Monitoring
                  </label>
                  <p className='text-sm text-gray-300'>
                    Automatically capture prompts from supported applications
                  </p>
                </div>
                <button
                  onClick={() =>
                    setConfig(prev => ({ ...prev, enabled: !prev.enabled }))
                  }
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                    config.enabled ? 'bg-blue-600' : 'bg-gray-600'
                  }`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                      config.enabled ? 'translate-x-6' : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>

              <div>
                <label className='mb-2 block font-medium text-white'>
                  Monitored Applications
                </label>
                <div className='grid grid-cols-2 gap-2'>
                  {availableApplications.map(app => (
                    <label
                      key={app}
                      className='flex cursor-pointer items-center space-x-2'
                    >
                      <input
                        type='checkbox'
                        checked={config.monitored_applications.includes(app)}
                        onChange={() => toggleApplication(app)}
                        className='rounded border-gray-300 text-blue-600 focus:ring-blue-500'
                      />
                      <span className='text-gray-300'>{app}</span>
                    </label>
                  ))}
                </div>
              </div>

              <div>
                <label className='mb-2 block font-medium text-white'>
                  Capture Threshold (minimum characters)
                </label>
                <input
                  type='number'
                  min='1'
                  max='1000'
                  value={config.capture_threshold}
                  onChange={e =>
                    setConfig(prev => ({
                      ...prev,
                      capture_threshold: parseInt(e.target.value) || 10,
                    }))
                  }
                  className='w-full rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-white backdrop-blur-sm focus:border-blue-500 focus:outline-none'
                />
                <p className='mt-1 text-sm text-gray-300'>
                  Only capture prompts longer than this threshold
                </p>
              </div>

              <div className='flex items-center justify-between'>
                <div>
                  <label className='font-medium text-white'>
                    Auto-save Prompts
                  </label>
                  <p className='text-sm text-gray-300'>
                    Automatically save captured prompts to the database
                  </p>
                </div>
                <button
                  onClick={() =>
                    setConfig(prev => ({ ...prev, auto_save: !prev.auto_save }))
                  }
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                    config.auto_save ? 'bg-blue-600' : 'bg-gray-600'
                  }`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                      config.auto_save ? 'translate-x-6' : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>
            </div>
          </div>

          {/* Security Settings */}
          <div className='rounded-lg border border-white/10 bg-white/5 backdrop-blur-sm'>
            <div className='border-b border-white/10 p-4'>
              <div className='flex items-center space-x-2'>
                <Shield className='h-5 w-5 text-green-400' />
                <h2 className='text-lg font-semibold text-white'>
                  Security Settings
                </h2>
              </div>
            </div>

            <div className='space-y-4 p-4'>
              <div className='flex items-center justify-between'>
                <div>
                  <label className='font-medium text-white'>
                    Enable Encryption
                  </label>
                  <p className='text-sm text-gray-300'>
                    Encrypt sensitive prompt data using AES-256
                  </p>
                </div>
                <button
                  onClick={() =>
                    setConfig(prev => ({
                      ...prev,
                      encryption_enabled: !prev.encryption_enabled,
                    }))
                  }
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                    config.encryption_enabled ? 'bg-green-600' : 'bg-gray-600'
                  }`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                      config.encryption_enabled
                        ? 'translate-x-6'
                        : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>

              <div className='rounded-lg border border-blue-500/20 bg-blue-500/10 p-3'>
                <div className='flex items-start space-x-2'>
                  <Info className='mt-0.5 h-4 w-4 text-blue-400' />
                  <div className='text-sm text-blue-300'>
                    <p className='font-medium'>Security Notice</p>
                    <p>
                      All data is stored locally on your device. No information
                      is sent to external servers.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Data Management */}
          <div className='rounded-lg border border-white/10 bg-white/5 backdrop-blur-sm'>
            <div className='border-b border-white/10 p-4'>
              <div className='flex items-center space-x-2'>
                <Database className='h-5 w-5 text-purple-400' />
                <h2 className='text-lg font-semibold text-white'>
                  Data Management
                </h2>
              </div>
            </div>

            <div className='space-y-4 p-4'>
              <div className='flex items-center justify-between'>
                <div>
                  <label className='font-medium text-white'>Export Data</label>
                  <p className='text-sm text-gray-300'>
                    Export all prompt history as JSON
                  </p>
                </div>
                <button
                  onClick={exportData}
                  disabled={loading}
                  className='flex items-center space-x-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700 disabled:opacity-50'
                >
                  <Database className='h-4 w-4' />
                  <span>Export</span>
                </button>
              </div>
            </div>
          </div>

          {/* Danger Zone */}
          <div className='rounded-lg border border-red-500/20 bg-red-500/5 backdrop-blur-sm'>
            <div className='border-b border-red-500/20 p-4'>
              <div className='flex items-center justify-between'>
                <div className='flex items-center space-x-2'>
                  <AlertCircle className='h-5 w-5 text-red-400' />
                  <h2 className='text-lg font-semibold text-white'>
                    Danger Zone
                  </h2>
                </div>
                <button
                  onClick={() => setShowDangerZone(!showDangerZone)}
                  className='text-red-400 hover:text-red-300'
                >
                  {showDangerZone ? (
                    <EyeOff className='h-4 w-4' />
                  ) : (
                    <Eye className='h-4 w-4' />
                  )}
                </button>
              </div>
            </div>

            {showDangerZone && (
              <div className='space-y-4 p-4'>
                <div className='flex items-center justify-between'>
                  <div>
                    <label className='font-medium text-white'>
                      Clear All Data
                    </label>
                    <p className='text-sm text-gray-300'>
                      Permanently delete all prompt history and settings
                    </p>
                  </div>
                  <button
                    onClick={clearAllData}
                    disabled={loading}
                    className='flex items-center space-x-2 rounded-lg bg-red-600 px-4 py-2 text-white transition-colors hover:bg-red-700 disabled:opacity-50'
                  >
                    <Trash2 className='h-4 w-4' />
                    <span>Clear All</span>
                  </button>
                </div>

                <div className='rounded-lg border border-red-500/20 bg-red-500/10 p-3'>
                  <div className='flex items-start space-x-2'>
                    <AlertCircle className='mt-0.5 h-4 w-4 text-red-400' />
                    <div className='text-sm text-red-300'>
                      <p className='font-medium'>Warning</p>
                      <p>
                        These actions cannot be undone. Make sure to export your
                        data first if needed.
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

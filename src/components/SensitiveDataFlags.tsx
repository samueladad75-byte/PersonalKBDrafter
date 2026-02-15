import { useEffect, useState } from 'react';
import { invoke } from '../lib/tauri';
import type { FlaggedSection } from '../bindings/FlaggedSection';

interface Props {
  content: string;
  onClose: () => void;
  onConfirm: () => void;
}

export function SensitiveDataFlags({ content, onClose, onConfirm }: Props) {
  const [flags, setFlags] = useState<FlaggedSection[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function scanContent() {
      try {
        const result = await invoke<FlaggedSection[]>('scan_sensitive_data', { content });
        setFlags(result);
      } catch (error) {
        console.error('Failed to scan for sensitive data:', error);
      } finally {
        setLoading(false);
      }
    }
    scanContent();
  }, [content]);

  const highSeverityCount = flags.filter(f => f.severity === 'high').length;
  const mediumSeverityCount = flags.filter(f => f.severity === 'medium').length;

  // Auto-confirm if no flags found
  useEffect(() => {
    if (!loading && flags.length === 0) {
      onConfirm();
    }
  }, [loading, flags.length, onConfirm]);

  if (loading) {
    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="bg-white rounded-lg p-6">
          <div className="text-center">Scanning for sensitive data...</div>
        </div>
      </div>
    );
  }

  if (flags.length === 0) {
    return null;
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 max-w-2xl w-full max-h-[80vh] overflow-auto">
        <div className="mb-4">
          <h2 className="text-2xl font-bold text-red-600">Sensitive Data Detected</h2>
          <p className="text-sm text-gray-600 mt-1">
            {highSeverityCount > 0 && (
              <span className="text-red-600 font-semibold">
                {highSeverityCount} high-severity
              </span>
            )}
            {highSeverityCount > 0 && mediumSeverityCount > 0 && ', '}
            {mediumSeverityCount > 0 && (
              <span className="text-yellow-600 font-semibold">
                {mediumSeverityCount} medium-severity
              </span>
            )}
            {' '}flag{flags.length > 1 ? 's' : ''} detected. Review before publishing.
          </p>
        </div>

        <div className="space-y-3 mb-6">
          {flags.map((flag, index) => (
            <div
              key={index}
              className={`border-l-4 p-3 rounded ${
                flag.severity === 'high'
                  ? 'border-red-500 bg-red-50'
                  : 'border-yellow-500 bg-yellow-50'
              }`}
            >
              <div className="flex justify-between items-start mb-2">
                <div>
                  <span className="font-semibold text-sm">
                    {flag.pattern_type.replace(/_/g, ' ').toUpperCase()}
                  </span>
                  <span
                    className={`ml-2 px-2 py-0.5 text-xs rounded ${
                      flag.severity === 'high'
                        ? 'bg-red-200 text-red-800'
                        : 'bg-yellow-200 text-yellow-800'
                    }`}
                  >
                    {flag.severity}
                  </span>
                </div>
                <span className="text-xs text-gray-500">Line {flag.line_number}</span>
              </div>
              <div className="font-mono text-sm bg-white p-2 rounded border">
                {flag.matched_text}
              </div>
            </div>
          ))}
        </div>

        <div className="bg-yellow-50 border border-yellow-200 rounded p-3 mb-4">
          <p className="text-sm text-yellow-800">
            <strong>⚠️ Warning:</strong> Publishing this article may expose sensitive
            information. Please review the flagged sections and remove any credentials,
            internal IPs, or secrets before proceeding.
          </p>
        </div>

        <div className="flex gap-3 justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 border border-gray-300 rounded hover:bg-gray-50"
          >
            Cancel
          </button>
          <button
            onClick={onConfirm}
            className="px-4 py-2 bg-yellow-600 text-white rounded hover:bg-yellow-700"
          >
            I've Reviewed - Proceed Anyway
          </button>
        </div>
      </div>
    </div>
  );
}

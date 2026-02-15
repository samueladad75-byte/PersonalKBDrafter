import { useState, useEffect } from 'react';
import { invoke } from '../lib/tauri';
import type { Article } from '../bindings/Article';

interface Props {
  isOpen: boolean;
  onClose: () => void;
  onLoadDraft: (article: Article) => void;
}

export function DraftsList({ isOpen, onClose, onLoadDraft }: Props) {
  const [drafts, setDrafts] = useState<Article[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isOpen) {
      loadDrafts();
    }
  }, [isOpen]);

  const loadDrafts = async () => {
    setLoading(true);
    setError(null);
    try {
      const allDrafts = await invoke<Article[]>('list_articles', { status: null });
      setDrafts(allDrafts);
    } catch (err: any) {
      setError(`Failed to load drafts: ${err.message || err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: bigint) => {
    if (!confirm('Are you sure you want to delete this draft?')) return;

    try {
      await invoke('delete_draft', { id });
      setDrafts(drafts.filter(d => d.id !== id));
    } catch (err: any) {
      alert(`Failed to delete: ${err.message || err}`);
    }
  };

  const formatDate = (dateStr: string) => {
    try {
      return new Date(dateStr).toLocaleString();
    } catch {
      return dateStr;
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 max-w-4xl w-full max-h-[80vh] overflow-auto">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-2xl font-bold">Saved Drafts</h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700 text-2xl"
          >
            ×
          </button>
        </div>

        {loading ? (
          <p className="text-center py-8 text-gray-500">Loading drafts...</p>
        ) : error ? (
          <div className="text-center py-8">
            <p className="text-red-600 mb-4">{error}</p>
            <button
              onClick={loadDrafts}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              Retry
            </button>
          </div>
        ) : drafts.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <p className="mb-2">No drafts yet</p>
            <p className="text-sm">Create your first article to get started!</p>
          </div>
        ) : (
          <div className="space-y-2">
            {drafts.map((draft) => (
              <div
                key={draft.id}
                className="border border-gray-300 rounded p-4 hover:bg-gray-50 transition-colors"
              >
                <div className="flex justify-between items-start">
                  <div className="flex-1">
                    <h3 className="font-semibold text-lg mb-1">{draft.title}</h3>
                    <div className="text-sm text-gray-600 space-y-1">
                      {draft.ticket_key && (
                        <p>
                          <span className="font-medium">Ticket:</span> {draft.ticket_key}
                        </p>
                      )}
                      <p>
                        <span className="font-medium">Status:</span>{' '}
                        <span
                          className={`px-2 py-0.5 rounded text-xs ${
                            draft.status === 'Published'
                              ? 'bg-green-100 text-green-800'
                              : 'bg-gray-100 text-gray-800'
                          }`}
                        >
                          {draft.status}
                        </span>
                      </p>
                      <p>
                        <span className="font-medium">Updated:</span>{' '}
                        {formatDate(draft.updated_at)}
                      </p>
                      {draft.tags.length > 0 && (
                        <p>
                          <span className="font-medium">Tags:</span> {draft.tags.join(', ')}
                        </p>
                      )}
                    </div>
                  </div>
                  <div className="flex gap-2 ml-4">
                    <button
                      onClick={() => {
                        onLoadDraft(draft);
                        onClose();
                      }}
                      className="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm"
                    >
                      Load
                    </button>
                    {draft.confluence_url && (
                      <a
                        href={draft.confluence_url}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 text-sm"
                      >
                        View
                      </a>
                    )}
                    <button
                      onClick={() => handleDelete(draft.id)}
                      className="px-3 py-1 border border-red-600 text-red-600 rounded hover:bg-red-50 text-sm"
                    >
                      Delete
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}

        <div className="mt-6 flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-gray-800 text-white rounded hover:bg-gray-700"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

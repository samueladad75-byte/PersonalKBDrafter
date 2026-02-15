import { useState } from 'react';
import { invoke } from '../lib/tauri';
import type { JiraTicket } from '../bindings/JiraTicket';

interface Props {
  isOpen: boolean;
  onClose: () => void;
  onSelectTicket: (ticket: JiraTicket) => void;
}

export function JiraTicketPicker({ isOpen, onClose, onSelectTicket }: Props) {
  const [searchQuery, setSearchQuery] = useState('');
  const [ticketKey, setTicketKey] = useState('');
  const [tickets, setTickets] = useState<JiraTicket[]>([]);
  const [selectedTicket, setSelectedTicket] = useState<JiraTicket | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;

    setLoading(true);
    setError(null);

    try {
      const results = await invoke<JiraTicket[]>('search_jira_tickets', {
        query: searchQuery,
      });
      setTickets(results);
    } catch (err: any) {
      setError(err.message || 'Search failed');
      console.error('Jira search failed:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleFetchByKey = async () => {
    if (!ticketKey.trim()) return;

    setLoading(true);
    setError(null);

    try {
      const ticket = await invoke<JiraTicket>('fetch_jira_ticket', {
        key: ticketKey.trim().toUpperCase(),
      });
      setSelectedTicket(ticket);
      setTickets([ticket]);
    } catch (err: any) {
      setError(err.message || 'Failed to fetch ticket');
      console.error('Failed to fetch ticket:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleSelectTicket = (ticket: JiraTicket) => {
    setSelectedTicket(ticket);
  };

  const handleConfirmSelection = () => {
    if (selectedTicket) {
      onSelectTicket(selectedTicket);
      onClose();
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 max-w-4xl w-full max-h-[90vh] overflow-auto">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-2xl font-bold">Load from Jira</h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700 text-2xl"
          >
            ×
          </button>
        </div>

        {/* Search by key or text */}
        <div className="mb-6 space-y-3">
          <div className="flex gap-2">
            <input
              type="text"
              value={ticketKey}
              onChange={(e) => setTicketKey(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleFetchByKey()}
              placeholder="Enter ticket key (e.g., JIRA-123)"
              className="flex-1 px-3 py-2 border border-gray-300 rounded-md"
            />
            <button
              onClick={handleFetchByKey}
              disabled={loading || !ticketKey.trim()}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
            >
              Fetch
            </button>
          </div>

          <div className="flex items-center gap-2">
            <div className="flex-1 h-px bg-gray-300" />
            <span className="text-sm text-gray-500">or search</span>
            <div className="flex-1 h-px bg-gray-300" />
          </div>

          <div className="flex gap-2">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
              placeholder="Search tickets by text"
              className="flex-1 px-3 py-2 border border-gray-300 rounded-md"
            />
            <button
              onClick={handleSearch}
              disabled={loading || !searchQuery.trim()}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
            >
              {loading ? 'Searching...' : 'Search'}
            </button>
          </div>
        </div>

        {error && (
          <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded text-red-700 text-sm">
            {error}
          </div>
        )}

        {/* Ticket list */}
        {tickets.length > 0 && (
          <div className="mb-6">
            <h3 className="text-sm font-semibold mb-2">Results ({tickets.length})</h3>
            <div className="space-y-2 max-h-60 overflow-auto">
              {tickets.map((ticket) => (
                <div
                  key={ticket.key}
                  onClick={() => handleSelectTicket(ticket)}
                  className={`p-3 border rounded cursor-pointer hover:bg-gray-50 ${
                    selectedTicket?.key === ticket.key
                      ? 'border-blue-500 bg-blue-50'
                      : 'border-gray-300'
                  }`}
                >
                  <div className="flex justify-between items-start">
                    <div>
                      <div className="font-semibold">{ticket.key}</div>
                      <div className="text-sm text-gray-600">{ticket.summary}</div>
                    </div>
                    <div className="text-xs text-gray-500">
                      {ticket.status}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Selected ticket detail */}
        {selectedTicket && (
          <div className="mb-6 border border-gray-300 rounded p-4 bg-gray-50">
            <h3 className="font-semibold mb-2">Selected Ticket</h3>
            <div className="space-y-2 text-sm">
              <div>
                <span className="font-medium">Key:</span> {selectedTicket.key}
              </div>
              <div>
                <span className="font-medium">Summary:</span> {selectedTicket.summary}
              </div>
              <div>
                <span className="font-medium">Status:</span> {selectedTicket.status}
              </div>
              {selectedTicket.description && (
                <div>
                  <span className="font-medium">Description:</span>
                  <div className="mt-1 p-2 bg-white rounded text-xs max-h-32 overflow-auto">
                    {selectedTicket.description}
                  </div>
                </div>
              )}
              {selectedTicket.comments.length > 0 && (
                <div>
                  <span className="font-medium">Latest Comment:</span>
                  <div className="mt-1 p-2 bg-white rounded text-xs">
                    <div className="font-medium">
                      {selectedTicket.comments[selectedTicket.comments.length - 1].author}
                    </div>
                    <div className="mt-1">
                      {selectedTicket.comments[selectedTicket.comments.length - 1].body}
                    </div>
                  </div>
                </div>
              )}
              {selectedTicket.labels.length > 0 && (
                <div>
                  <span className="font-medium">Labels:</span>{' '}
                  {selectedTicket.labels.join(', ')}
                </div>
              )}
            </div>
          </div>
        )}

        {/* Actions */}
        <div className="flex justify-end gap-2">
          <button
            onClick={onClose}
            className="px-4 py-2 border border-gray-300 rounded hover:bg-gray-50"
          >
            Cancel
          </button>
          <button
            onClick={handleConfirmSelection}
            disabled={!selectedTicket}
            className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50"
          >
            Load Ticket
          </button>
        </div>
      </div>
    </div>
  );
}

import { useState, useEffect } from 'react';
import { invoke } from '../lib/tauri';
import type { Template } from '../bindings/Template';

interface Props {
  onSelect: (template: Template) => void;
  selectedTemplateId: string | null;
}

export function TemplateSelector({ onSelect, selectedTemplateId }: Props) {
  const [templates, setTemplates] = useState<Template[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadTemplates() {
      try {
        const result = await invoke<Template[]>('list_templates');
        setTemplates(result);
      } catch (err) {
        setError('Failed to load templates');
        console.error(err);
      } finally {
        setLoading(false);
      }
    }
    loadTemplates();
  }, []);

  if (loading) {
    return <div className="text-sm text-gray-500">Loading templates...</div>;
  }

  if (error) {
    return <div className="text-sm text-red-500">{error}</div>;
  }

  return (
    <div className="template-selector">
      <label htmlFor="template" className="block text-sm font-medium mb-1">
        Template
      </label>
      <select
        id="template"
        value={selectedTemplateId || ''}
        onChange={(e) => {
          const template = templates.find(t => t.id === e.target.value);
          if (template) onSelect(template);
        }}
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      >
        <option value="">Select a template...</option>
        {templates.map((template) => (
          <option key={template.id} value={template.id}>
            {template.name}
            {template.is_builtin && ' (Built-in)'}
          </option>
        ))}
      </select>
      {selectedTemplateId && (
        <div className="mt-2 text-xs text-gray-600">
          {templates.find(t => t.id === selectedTemplateId)?.description}
        </div>
      )}
    </div>
  );
}

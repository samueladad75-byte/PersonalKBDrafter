import { useEffect } from 'react';
import { useArticleForm } from '../hooks/useArticleForm';
import { QualityScoreBadge } from './QualityScoreBadge';

interface Props {
  onContentChange: (markdown: string) => void;
  initialData?: any;
}

export function ArticleForm({ onContentChange, initialData }: Props) {
  const {
    formData,
    updateField,
    updateContentMarkdown,
    qualityScore,
    calculateQualityScore,
  } = useArticleForm(initialData);

  // Update markdown when fields change
  useEffect(() => {
    const markdown = updateContentMarkdown();
    onContentChange(markdown);
  }, [formData.title, formData.problem, formData.solution, formData.expectedResult, formData.prerequisites, formData.additionalNotes, updateContentMarkdown, onContentChange]);

  // Calculate quality score on meaningful field changes (not on every markdown edit)
  useEffect(() => {
    const timeout = setTimeout(() => {
      calculateQualityScore();
    }, 500);
    return () => clearTimeout(timeout);
  }, [
    formData.title,
    formData.problem,
    formData.solution,
    formData.expectedResult,
    formData.prerequisites,
    formData.additionalNotes,
    formData.tags,
    calculateQualityScore
  ]);

  return (
    <div className="article-form space-y-4 p-4">
      <div className="flex justify-between items-center">
        <h2 className="text-2xl font-bold">Article Details</h2>
        {qualityScore && <QualityScoreBadge score={qualityScore} />}
      </div>

      <div className="form-field">
        <label htmlFor="title" className="block text-sm font-medium mb-1">
          Title <span className="text-red-500">*</span>
        </label>
        <input
          id="title"
          type="text"
          value={formData.title}
          onChange={(e) => updateField('title', e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          placeholder="Brief descriptive title"
        />
      </div>

      <div className="form-field">
        <label htmlFor="problem" className="block text-sm font-medium mb-1">
          Problem Description <span className="text-red-500">*</span>
        </label>
        <textarea
          id="problem"
          value={formData.problem}
          onChange={(e) => updateField('problem', e.target.value)}
          rows={3}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          placeholder="1-2 sentences describing the user-reported issue"
        />
      </div>

      <div className="form-field">
        <label htmlFor="solution" className="block text-sm font-medium mb-1">
          Solution Steps <span className="text-red-500">*</span>
        </label>
        <textarea
          id="solution"
          value={formData.solution}
          onChange={(e) => updateField('solution', e.target.value)}
          rows={8}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
          placeholder="1. First step&#10;2. Second step&#10;3. Third step"
        />
      </div>

      <div className="form-field">
        <label htmlFor="expectedResult" className="block text-sm font-medium mb-1">
          Expected Result
        </label>
        <textarea
          id="expectedResult"
          value={formData.expectedResult}
          onChange={(e) => updateField('expectedResult', e.target.value)}
          rows={2}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          placeholder="What the user should see after following the steps"
        />
      </div>

      <div className="form-field">
        <label htmlFor="prerequisites" className="block text-sm font-medium mb-1">
          Prerequisites
        </label>
        <textarea
          id="prerequisites"
          value={formData.prerequisites}
          onChange={(e) => updateField('prerequisites', e.target.value)}
          rows={2}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          placeholder="Requirements before starting (e.g., admin access, specific software)"
        />
      </div>

      <div className="form-field">
        <label htmlFor="additionalNotes" className="block text-sm font-medium mb-1">
          Additional Notes
        </label>
        <textarea
          id="additionalNotes"
          value={formData.additionalNotes}
          onChange={(e) => updateField('additionalNotes', e.target.value)}
          rows={2}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          placeholder="Workarounds, related issues, prevention tips"
        />
      </div>

      <div className="form-field">
        <label htmlFor="tags" className="block text-sm font-medium mb-1">
          Tags
        </label>
        <input
          id="tags"
          type="text"
          value={formData.tags.join(', ')}
          onChange={(e) => updateField('tags', e.target.value.split(',').map(t => t.trim()).filter(Boolean))}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          placeholder="tag1, tag2, tag3"
        />
      </div>
    </div>
  );
}

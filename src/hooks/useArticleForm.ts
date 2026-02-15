import { useState, useCallback } from 'react';
import { invoke } from '../lib/tauri';
import type { QualityScore } from '../bindings/QualityScore';

export interface ArticleFormData {
  title: string;
  problem: string;
  solution: string;
  expectedResult: string;
  prerequisites: string;
  additionalNotes: string;
  tags: string[];
  contentMarkdown: string;
  templateId: string | null;
  ticketKey: string | null;
}

export function useArticleForm(initialData?: Partial<ArticleFormData>) {
  const [formData, setFormData] = useState<ArticleFormData>({
    title: initialData?.title ?? '',
    problem: initialData?.problem ?? '',
    solution: initialData?.solution ?? '',
    expectedResult: initialData?.expectedResult ?? '',
    prerequisites: initialData?.prerequisites ?? '',
    additionalNotes: initialData?.additionalNotes ?? '',
    tags: initialData?.tags ?? [],
    contentMarkdown: initialData?.contentMarkdown ?? '',
    templateId: initialData?.templateId ?? null,
    ticketKey: initialData?.ticketKey ?? null,
  });

  const [qualityScore, setQualityScore] = useState<QualityScore | null>(null);
  const [isScoring, setIsScoring] = useState(false);

  const updateField = useCallback(<K extends keyof ArticleFormData>(
    field: K,
    value: ArticleFormData[K]
  ) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  }, []);

  const updateContentMarkdown = useCallback(() => {
    // Assemble markdown from form fields
    const markdown = `# ${formData.title}

## Problem
${formData.problem}

## Solution
${formData.solution}

${formData.expectedResult ? `## Expected Result\n${formData.expectedResult}\n` : ''}
${formData.prerequisites ? `## Prerequisites\n${formData.prerequisites}\n` : ''}
${formData.additionalNotes ? `## Additional Notes\n${formData.additionalNotes}\n` : ''}
`.trim();

    setFormData(prev => ({ ...prev, contentMarkdown: markdown }));
    return markdown;
  }, [formData.title, formData.problem, formData.solution, formData.expectedResult, formData.prerequisites, formData.additionalNotes]);

  const calculateQualityScore = useCallback(async () => {
    setIsScoring(true);
    try {
      const score = await invoke<QualityScore>('score_quality', {
        article: {
          ticket_key: formData.ticketKey,
          title: formData.title,
          problem: formData.problem,
          solution: formData.solution,
          expected_result: formData.expectedResult ?? null,
          prerequisites: formData.prerequisites ?? null,
          additional_notes: formData.additionalNotes ?? null,
          tags: formData.tags,
          content_markdown: formData.contentMarkdown,
          template_id: formData.templateId,
        }
      });
      setQualityScore(score);
    } catch (error) {
      console.error('Failed to calculate quality score:', error);
    } finally {
      setIsScoring(false);
    }
  }, [formData]);

  const validate = useCallback(() => {
    const errors: string[] = [];
    if (!formData.title.trim()) errors.push('Title is required');
    if (!formData.problem.trim()) errors.push('Problem is required');
    if (!formData.solution.trim()) errors.push('Solution is required');
    return errors;
  }, [formData]);

  return {
    formData,
    updateField,
    updateContentMarkdown,
    qualityScore,
    isScoring,
    calculateQualityScore,
    validate,
    setFormData,
  };
}

import { useMutation } from '@tanstack/react-query';
import { invoke } from '../lib/tauri';
import type { JiraTicket } from '../bindings/JiraTicket';

interface DraftParams {
  ticket: JiraTicket;
  templateId: string;
  ollamaUrl: string;
  model: string;
}

export function useDraftArticle() {
  return useMutation({
    mutationFn: async ({ ticket, templateId, ollamaUrl, model }: DraftParams) => {
      const markdown = await invoke<string>('draft_with_llm', {
        ticket,
        templateId,
        ollamaUrl,
        model,
      });
      return markdown;
    },
  });
}

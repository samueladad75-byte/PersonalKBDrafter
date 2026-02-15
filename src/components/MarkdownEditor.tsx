import { useRef, useEffect } from 'react';
import { EditorView, basicSetup } from 'codemirror';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { languages } from '@codemirror/language-data';
import { oneDark } from '@codemirror/theme-one-dark';
import { EditorState } from '@codemirror/state';
import { keymap } from '@codemirror/view';
import { defaultKeymap } from '@codemirror/commands';

interface Props {
  value: string;
  onChange: (value: string) => void;
  theme?: 'light' | 'dark';
  className?: string;
}

export function MarkdownEditor({ value, onChange, theme = 'light', className = '' }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);
  const lastExternalValueRef = useRef<string>(value);

  useEffect(() => {
    if (!containerRef.current) return;

    const extensions = [
      basicSetup,
      markdown({ base: markdownLanguage, codeLanguages: languages }),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          const newValue = update.state.doc.toString();
          lastExternalValueRef.current = newValue;
          onChange(newValue);
        }
      }),
      keymap.of(defaultKeymap),
    ];

    if (theme === 'dark') {
      extensions.push(oneDark);
    }

    const view = new EditorView({
      state: EditorState.create({
        doc: value,
        extensions,
      }),
      parent: containerRef.current,
    });

    viewRef.current = view;
    lastExternalValueRef.current = value;

    return () => {
      view.destroy();
      viewRef.current = null;
    };
  }, [theme]); // Only recreate on theme change

  // Sync external value changes (guard against circular updates)
  useEffect(() => {
    const view = viewRef.current;
    if (view && value !== lastExternalValueRef.current) {
      const currentDoc = view.state.doc.toString();
      if (value !== currentDoc) {
        view.dispatch({
          changes: { from: 0, to: view.state.doc.length, insert: value },
        });
        lastExternalValueRef.current = value;
      }
    }
  }, [value]);

  return <div ref={containerRef} className={`markdown-editor ${className}`} />;
}

interface ParsedArticle {
  title: string;
  problem: string;
  solution: string;
  expectedResult: string;
  prerequisites: string;
  additionalNotes: string;
  tags: string[];
}

export function parseMarkdownToArticle(markdown: string): ParsedArticle {
  const lines = markdown.split('\n');

  let title = '';
  let problem = '';
  let solution = '';
  let expectedResult = '';
  let prerequisites = '';
  let additionalNotes = '';
  let tags: string[] = [];

  let currentSection = '';
  let sectionContent: string[] = [];
  const seenSections = new Set<string>();

  for (const line of lines) {
    // Check for title (first # heading)
    if (!title && line.match(/^#\s+/)) {
      title = line.replace(/^#+\s*/, '').trim();
      continue;
    }

    // Check for section headers
    if (line.match(/^##\s+/)) {
      // Save previous section
      if (currentSection && sectionContent.length > 0) {
        const content = sectionContent.join('\n').trim();
        const normalizedSection = currentSection.toLowerCase();

        // Check for duplicate sections
        if (seenSections.has(normalizedSection)) {
          console.warn(`Duplicate section found: "${currentSection}". Appending content to existing section.`);
        }

        switch (normalizedSection) {
          case 'problem':
            problem = seenSections.has(normalizedSection) ? `${problem}\n\n${content}` : content;
            seenSections.add(normalizedSection);
            break;
          case 'solution':
          case 'resolution':
            solution = seenSections.has('solution') || seenSections.has('resolution')
              ? `${solution}\n\n${content}`
              : content;
            seenSections.add('solution');
            seenSections.add('resolution');
            break;
          case 'expected result':
          case 'expected outcome':
            expectedResult = seenSections.has('expected result') || seenSections.has('expected outcome')
              ? `${expectedResult}\n\n${content}`
              : content;
            seenSections.add('expected result');
            seenSections.add('expected outcome');
            break;
          case 'prerequisites':
          case 'requirements':
            prerequisites = seenSections.has('prerequisites') || seenSections.has('requirements')
              ? `${prerequisites}\n\n${content}`
              : content;
            seenSections.add('prerequisites');
            seenSections.add('requirements');
            break;
          case 'additional notes':
          case 'notes':
            additionalNotes = seenSections.has('additional notes') || seenSections.has('notes')
              ? `${additionalNotes}\n\n${content}`
              : content;
            seenSections.add('additional notes');
            seenSections.add('notes');
            break;
          case 'tags':
          case 'labels':
            const newTags = content.split(',').map(t => t.trim()).filter(t => t);
            tags = seenSections.has('tags') || seenSections.has('labels')
              ? [...tags, ...newTags]
              : newTags;
            seenSections.add('tags');
            seenSections.add('labels');
            break;
        }
      }

      // Start new section
      currentSection = line.replace(/^##\s*/, '').trim();
      sectionContent = [];
      continue;
    }

    // Add line to current section
    if (currentSection) {
      sectionContent.push(line);
    }
  }

  // Save last section
  if (currentSection && sectionContent.length > 0) {
    const content = sectionContent.join('\n').trim();
    const normalizedSection = currentSection.toLowerCase();

    // Check for duplicate sections
    if (seenSections.has(normalizedSection)) {
      console.warn(`Duplicate section found: "${currentSection}". Appending content to existing section.`);
    }

    switch (normalizedSection) {
      case 'problem':
        problem = seenSections.has(normalizedSection) ? `${problem}\n\n${content}` : content;
        break;
      case 'solution':
      case 'resolution':
        solution = seenSections.has('solution') || seenSections.has('resolution')
          ? `${solution}\n\n${content}`
          : content;
        break;
      case 'expected result':
      case 'expected outcome':
        expectedResult = seenSections.has('expected result') || seenSections.has('expected outcome')
          ? `${expectedResult}\n\n${content}`
          : content;
        break;
      case 'prerequisites':
      case 'requirements':
        prerequisites = seenSections.has('prerequisites') || seenSections.has('requirements')
          ? `${prerequisites}\n\n${content}`
          : content;
        break;
      case 'additional notes':
      case 'notes':
        additionalNotes = seenSections.has('additional notes') || seenSections.has('notes')
          ? `${additionalNotes}\n\n${content}`
          : content;
        break;
      case 'tags':
      case 'labels':
        const newTags = content.split(',').map(t => t.trim()).filter(t => t);
        tags = seenSections.has('tags') || seenSections.has('labels')
          ? [...tags, ...newTags]
          : newTags;
        break;
    }
  }

  return {
    title: title || 'Untitled Article',
    problem,
    solution,
    expectedResult,
    prerequisites,
    additionalNotes,
    tags,
  };
}

import type { QualityScore } from '../bindings/QualityScore';

interface Props {
  score: QualityScore;
}

export function QualityScoreBadge({ score }: Props) {
  const getColorClass = (overall: number) => {
    if (overall >= 80) return 'bg-green-500 text-white';
    if (overall >= 60) return 'bg-yellow-500 text-white';
    return 'bg-red-500 text-white';
  };

  return (
    <div className="quality-score-badge relative group">
      <div className={`px-4 py-2 rounded-full font-bold ${getColorClass(score.overall)}`}>
        {score.overall}%
      </div>

      {/* Tooltip */}
      <div className="absolute right-0 top-full mt-2 w-64 p-3 bg-gray-900 text-white text-sm rounded-lg shadow-lg opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-10">
        <div className="font-semibold mb-2">Quality Breakdown</div>
        <div className="space-y-1 text-xs">
          <div className="flex justify-between">
            <span>Title:</span>
            <span>{score.has_title ? '✓' : '✗'}</span>
          </div>
          <div className="flex justify-between">
            <span>Problem:</span>
            <span>{score.has_problem ? '✓' : '✗'}</span>
          </div>
          <div className="flex justify-between">
            <span>Solution:</span>
            <span>{score.has_solution ? '✓' : '✗'}</span>
          </div>
          <div className="flex justify-between">
            <span>Expected Result:</span>
            <span>{score.has_expected_result ? '✓' : '✗'}</span>
          </div>
          <div className="flex justify-between">
            <span>Prerequisites:</span>
            <span>{score.has_prerequisites ? '✓' : '✗'}</span>
          </div>
          <div className="flex justify-between">
            <span>Solution Steps:</span>
            <span>{score.solution_step_count}</span>
          </div>
          <div className="flex justify-between">
            <span>Word Count:</span>
            <span>{score.word_count}</span>
          </div>
        </div>

        {score.warnings.length > 0 && (
          <div className="mt-3 pt-2 border-t border-gray-700">
            <div className="font-semibold mb-1">Warnings:</div>
            <ul className="text-xs space-y-1">
              {score.warnings.map((warning, i) => (
                <li key={i} className="text-yellow-400">⚠ {warning}</li>
              ))}
            </ul>
          </div>
        )}
      </div>
    </div>
  );
}

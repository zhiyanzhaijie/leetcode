import { startTransition, useEffect, useState } from 'react';
import { Button } from '../ui/button';

export default function BackToTopButton() {
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const onScroll = () => {
      startTransition(() => {
        setVisible(window.scrollY > 520);
      });
    };
    window.addEventListener('scroll', onScroll, { passive: true });
    onScroll();
    return () => window.removeEventListener('scroll', onScroll);
  }, []);

  if (!visible) return null;

  return (
    <Button
      size="icon"
      className="fixed bottom-6 right-6 z-50 rounded-full border border-[var(--line)] bg-[var(--paper)] text-[var(--ink)] shadow-none hover:bg-[var(--paper-deep)]"
      onClick={() => window.scrollTo({ top: 0, behavior: 'smooth' })}
      aria-label="Back to top"
    >
      ↑
    </Button>
  );
}

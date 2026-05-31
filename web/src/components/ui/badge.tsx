import { cva, type VariantProps } from 'class-variance-authority';
import * as React from 'react';
import { cn } from '../../lib/utils';

const badgeVariants = cva('inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors', {
  variants: {
    variant: {
      default: 'border-transparent bg-[var(--primary)] text-[var(--primary-foreground)]',
      secondary: 'border-transparent bg-[var(--secondary)] text-[var(--secondary-foreground)]',
      outline: 'text-[var(--foreground)] border-[var(--border)]',
      easy: 'border-transparent bg-[#e8f5e9] text-[#1aae39]',
      medium: 'border-transparent bg-[#ffe8d4] text-[#793400]',
      hard: 'border-transparent bg-[#fde0ec] text-[#a02e6d]',
      unknown: 'border-transparent bg-[var(--muted)] text-[var(--muted-foreground)]',
    },
  },
  defaultVariants: {
    variant: 'default',
  },
});

export interface BadgeProps extends React.HTMLAttributes<HTMLDivElement>, VariantProps<typeof badgeVariants> {}

function Badge({ className, variant, ...props }: BadgeProps) {
  return <div className={cn(badgeVariants({ variant }), className)} {...props} />;
}

export { Badge, badgeVariants };

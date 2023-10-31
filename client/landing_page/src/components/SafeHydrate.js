import React from 'react';
export function SafeHydrate({ children }) {
	return (
		<div suppressHydrationWarning>
			{typeof window === 'undefined' ? null : children}
		</div>
	);
}

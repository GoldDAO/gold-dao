'use client';

import { useEffect } from 'react';
import RewardsContent from '../../components/Rewards/RewardsContent';

export const viewport = {
  themeColor: '#c6c6c6',
};

export default function Rewards() {
  // edit navigation bar color
  useEffect(() => {
    let metaTag = document.querySelector('meta[name="theme-color"]');

    if (!metaTag) {
      metaTag = document.createElement('meta');
      metaTag.setAttribute('name', 'theme-color');
      document.getElementsByTagName('head')[0].appendChild(metaTag);
    }

    metaTag.setAttribute('content', '#c6c6c6');
  }, []);

  return (
    <main>
      <RewardsContent />
    </main>
  );
}

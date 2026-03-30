'use client';

import React, { useEffect, useState } from 'react';
import {Joyride} from 'react-joyride';
import type { CallBackProps, STATUS, Step } from 'react-joyride';

const steps: Step[] = [
  {
    target: '.connect-wallet-btn',
    content: 'Welcome to StellarStream Wave 3! First, connect your Freighter or Stellar wallet to get started.',
    title: 'Step 1: Connect Wallet',
    placement: 'bottom',
  },
  {
    target: '.migration-wizard',
    content: 'If you have V1 streams, use the Migration Wizard to safely move them to the new V2 system with improved features.',
    title: 'Step 2: Migrate from V1',
    placement: 'top',
  },
  {
    target: '.create-stream-btn',
    content: 'Now you’re ready! Create your first V2 Stream using the new Permit system for better control and security.',
    title: 'Step 3: Start Your First V2 Stream',
    placement: 'bottom',
  },
];

export default function OnboardingTour() {
  const [run, setRun] = useState(false);
  const [isFirstVisit, setIsFirstVisit] = useState(false);

  useEffect(() => {
    const hasSeenTour = localStorage.getItem('stellar-wave3-onboarding-seen');
    if (!hasSeenTour) {
      setIsFirstVisit(true);
      // Small delay to ensure DOM elements are rendered
      setTimeout(() => setRun(true), 1200);
    }
  }, []);

  const handleJoyrideCallback = (data: CallBackProps) => {
    const { status } = data;

    if ([STATUS.FINISHED, STATUS.SKIPPED].includes(status)) {
      setRun(false);
      localStorage.setItem('stellar-wave3-onboarding-seen', 'true');
    }
  };

  if (!isFirstVisit) return null;

  return (
    <Joyride
      steps={steps}
      run={run}
      callback={handleJoyrideCallback}
      continuous
      showProgress
      showSkipButton
      hideCloseButton
      disableBeacon
      styles={{
        options: {
          primaryColor: '#3b82f6',
          textColor: '#1f2937',
          backgroundColor: '#ffffff',
          arrowColor: '#ffffff',
          zIndex: 10000,
        },
        tooltip: {
          borderRadius: '16px',
          padding: '20px',
        },
        buttonNext: {
          backgroundColor: '#3b82f6',
          borderRadius: '9999px',
        },
      }}
      locale={{
        last: "Got it, let's go!",
        skip: 'Skip tour',
        next: 'Next',
        back: 'Back',
      }}
    />
  );
}
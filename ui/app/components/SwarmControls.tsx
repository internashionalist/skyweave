

"use client";

import React from "react";

export type SwarmSettings = {
  cohesion: number; // how strongly UAVs pull toward group center
  separation: number; // how strongly they avoid crowding
  alignment: number; // how strongly they match neighbor heading
  maxSpeed: number; // cap on speed (m/s)
  targetAltitude: number; // preferred cruise altitude
};

type SwarmControlsProps = {
  settings: SwarmSettings;
  onChange: (next: SwarmSettings) => void;
};

/**
 * SwarmControls
 *
 * Mission Control-style panel for tuning swarm behavior parameters.
 * - Exposes cohesion / separation / alignment weights
 * - Controls max speed and target altitude
 *
 * This component is presentational + emits setting changes upward via onChange.
 */
export default function SwarmControls({ settings, onChange }: SwarmControlsProps) {
  const updateField = (field: keyof SwarmSettings, value: number) => {
    onChange({ ...settings, [field]: value });
  };

  return (
    <section className="border border-zinc-700 rounded-lg p-4 bg-black/80 nasa-text text-xs">
      <h2 className="mb-3 uppercase tracking-wide text-[0.7rem] text-zinc-400">
        Swarm Behavior Controls
      </h2>

      <div className="space-y-3">
        {/* Cohesion */}
        <ControlRow
          label="COHESION"
          min={0}
          max={2}
          step={0.05}
          value={settings.cohesion}
          onChange={(v) => updateField("cohesion", v)}
        />

        {/* Separation */}
        <ControlRow
          label="SEPARATION"
          min={0}
          max={2}
          step={0.05}
          value={settings.separation}
          onChange={(v) => updateField("separation", v)}
        />

        {/* Alignment */}
        <ControlRow
          label="ALIGNMENT"
          min={0}
          max={2}
          step={0.05}
          value={settings.alignment}
          onChange={(v) => updateField("alignment", v)}
        />

        <div className="h-px bg-zinc-700 my-2" />

        {/* Max Speed */}
        <ControlRow
          label="MAX SPEED (m/s)"
          min={0}
          max={60}
          step={1}
          value={settings.maxSpeed}
          onChange={(v) => updateField("maxSpeed", v)}
        />

        {/* Target Altitude */}
        <ControlRow
          label="TARGET ALT (m)"
          min={0}
          max={500}
          step={10}
          value={settings.targetAltitude}
          onChange={(v) => updateField("targetAltitude", v)}
        />
      </div>
    </section>
  );
}

type ControlRowProps = {
  label: string;
  min: number;
  max: number;
  step: number;
  value: number;
  onChange: (value: number) => void;
};

function ControlRow({ label, min, max, step, value, onChange }: ControlRowProps) {
  return (
    <div>
      <div className="flex justify-between mb-1">
        <span className="text-[0.7rem] tracking-wide">{label}</span>
        <span className="text-[0.7rem]">{value.toFixed(2)}</span>
      </div>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(e) => onChange(parseFloat(e.target.value))}
        className="w-full accent-emerald-400 cursor-pointer"
      />
    </div>
  );
}

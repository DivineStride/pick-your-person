# Design Document: Finger Selector App

## Overview

The Finger Selector app is a multi-touch application where multiple users place their fingers on the screen simultaneously, and after a short countdown, one finger is randomly selected as the "winner." Each finger is assigned a unique color, and when a winner is chosen, the app highlights the winning finger's color.

## Core Features

1. Multi-touch detection and tracking
2. Unique color assignment for each finger
3. Countdown timer before selection
4. Random selection of one finger
5. Visual feedback for the selected finger/person

## Technical Requirements

### Platform

- Target platform: Cross-platform (iOS/Android)
- Development framework: Bevy game engine with Rust

### Key Components

1. Touch Input System

- Track multiple simultaneous touch inputs
- Assign unique IDs to each touch point
- Map finger positions to screen coordinates
- Detect when fingers are added or removed

2. Visual Representation

- Create a visual element (circle/blob) for each finger
- Assign a unique color from a predefined palette to each finger
- Scale visual elements based on touch pressure (if available)

3. Selection Logic

- Start countdown when touch count remains stable for a set period
- Implement random selection algorithm
- Provide visual and possibly audio feedback for the countdown
- Highlight the winner with animation effects

4. User Interface

- Clean, minimalist design
- Clear visual indication of app state (waiting for fingers, countdown, selection)
- Option to restart the selection process

## Implementation Plan

### Bevy ECS Structure

#### Components

- `FingerTouch`: Contains touch ID, position, color, and selection status
- `CountdownTimer`: Manages the selection countdown
- `GameState`: Tracks current app state (waiting, counting, selected)

#### Systems

- `input_system`: Processes touch events and updates finger entities
- `color_assignment_system`: Assigns unique colors to fingers
- `countdown_system`: Manages the countdown timer
- `selection_system`: Handles the random selection logic
- `render_system`: Draws the finger visuals and UI elements

#### Resources

- `TouchInputResource`: Stores global touch state
- `ColorPalette`: Contains the predefined colors for assignment
- `AppConfig`: Stores configurable parameters like countdown duration

## User Flow

1. App starts in "waiting" state with empty screen
2. Users place fingers on the screen
3. Visual feedback shows each finger with a unique color
4. After a short stability period, countdown begins (3-5 seconds)
5. Random selection occurs, highlighting the winner
6. App displays the winning color prominently
7. After a few seconds, app returns to ready state for another round

## Technical Considerations

- Handle edge cases such as fingers being removed during countdown
- Optimize for different screen sizes and device capabilities
- Consider accessibility features like vibration feedback or clear visual indicators
- Ensure responsive performance with many simultaneous touches

## Future Enhancements

- Sound effects for countdown and selection
- Customizable color palettes
- History of selections
- Name assignment to different fingers/colors
- Sharing selection results

This design document provides a solid foundation for implementing your finger selector app in Bevy.

use std::time::Duration;

/// An `Animation` progresses from a current value, towards a target value.
///
/// This struct does not cause an animation by itself,
/// as that requires registering subscriptions among other things.
/// Instead, it provides a way to scale a value towards a target value via exponential decay.
/// Therefore the [`current()`] value of the animation can be bound to width,
/// height or other animation properties.
///
/// The smoothness of the animation is determined by the speed of change, and the frame rate.
/// These can be specified via the builder methods [`with_speed`] and [`with_frame_rate`].
pub struct Animation {
    current: f32,
    target: f32,
    speed: f32,
    frame_rate: u64,
}

impl Animation {
    /// The threshold in value difference that determines if the animation is still active.
    const ACTIVE_ANIMATION_THRESHOLD: f32 = 0.5;

    /// Note that the animation is not linear, it is exponential based on this factor.
    const DEFAULT_ANIMATION_RATE_OF_CHANGE: f32 = 0.2;

    /// The default frame rate an animation will run in.
    const DEFAULT_ANIMATION_FRAME_RATE: u64 = 60;

    /// Create a new [`Animation`].
    ///
    /// The starting point of the animation is `current`, and the end point is `target`.
    ///
    /// # Examples
    /// ```rust
    /// use widgets::Animation;
    ///
    /// // create a new animation
    /// let animation = Animation::new(0.0, 50.0);
    /// ```
    pub fn new(current: f32, target: f32) -> Self {
        Self {
            current,
            target,
            speed: Self::DEFAULT_ANIMATION_RATE_OF_CHANGE,
            frame_rate: Self::DEFAULT_ANIMATION_FRAME_RATE,
        }
    }

    /// Retarget the animation by defining a new `target`.
    ///
    /// This allows the animation to smoothly reverse halfway through instead of restarting.
    ///
    /// # Examples
    /// ```rust
    /// use widgets::Animation;
    ///
    /// // create a new animation
    /// let mut animation = Animation::new(0.0, 50.0);
    ///
    /// // some stuff happens, and the animation finishes
    ///
    /// // we want to flip our animation
    /// animation.retarget(0.0);
    /// ```
    pub fn retarget(&mut self, target: f32) {
        self.target = target;
    }

    /// A builder to set the `speed` of the animation.
    ///
    /// The animation is made using exponential decay, this means that the `speed` is the fraction
    /// of progress the animation makes on each [`tick`].
    ///
    /// For example, if an animation has `target = 50.0`, `current = 0.0`, and `speed = 0.5`, then
    /// the animation would progress to `current = 25.0` in the first step, `current = 37.5` in the
    /// second step, and so on.
    ///
    /// This results in a spring-like animation that starts off fast, and ends slowly.
    ///
    /// # Examples
    /// ```rust
    /// use widgets::Animation;
    ///
    /// // create a new animation with custom speed
    /// let animation = Animation::new(0.0, 50.0).with_speed(0.3);
    /// ```
    #[must_use = "builder methods return a new value and don't modify self"]
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// A builder to set the `frame_rate` of the animation.
    ///
    /// The frame rate determines how often the animation updates are rendered.
    ///
    /// # Panics
    /// This function will panic if `frame_rate` is `0`.
    ///
    /// # Examples
    /// ```rust
    /// use widgets::Animation;
    ///
    /// // create a new animation with custom frame rate
    /// let animation = Animation::new(0.0, 50.0).with_frame_rate(100);
    /// ```
    #[must_use = "builder methods return a new value and don't modify self"]
    pub fn with_frame_rate(mut self, frame_rate: u64) -> Self {
        assert!(frame_rate > 0, "frame rate must be greater than 0");

        self.frame_rate = frame_rate;
        self
    }

    /// Whether the animation is currently in progress.
    pub fn in_progress(&self) -> bool {
        self.signed_difference().abs() > Self::ACTIVE_ANIMATION_THRESHOLD
    }

    /// Returns the current state of the animation.
    pub fn current(&self) -> f32 {
        self.current
    }

    /// Advance the animation by a tick.
    pub fn tick(&mut self) {
        match self.in_progress() {
            true => self.current += self.signed_difference() * self.speed,
            false => self.current = self.target,
        }
    }

    /// Returns a [`Duration`] representing the tick rate of the animation.
    ///
    /// The tick rate is determined from the frame rate of the animation.
    pub fn tick_rate(&self) -> Duration {
        Duration::from_millis(1000 / self.frame_rate)
    }

    fn signed_difference(&self) -> f32 {
        self.target - self.current
    }
}
